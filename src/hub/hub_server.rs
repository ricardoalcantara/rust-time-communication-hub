use super::{
    hub_connection::HubConnection, hub_event_message::HubEventMessage, hub_packet::HubPackage,
};
use crate::repository::{
    models::{group::Group, message::Message, user::User},
    repositories::{
        group_repository::GroupRepository, message_repository::MessageRepository,
        user_repository::UserRepository,
    },
    repository_base::RepositoryBase,
};
use axum::response::sse::Event;
use std::{collections::HashMap, convert::Infallible, time::Duration};
use tokio::{
    sync::mpsc::{error::SendError, Sender},
    time::interval,
};

type Db<T> = HashMap<String, Vec<T>>;
type SseSender = Sender<Result<Event, Infallible>>;

pub struct HubServer {
    pub users: Db<SseSender>,
    pub groups: Db<String>,
    repository: RepositoryBase,
}

impl HubServer {
    pub fn new(repository: RepositoryBase) -> Self {
        HubServer {
            repository,
            users: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    // Todo: Queue Messages for later
    pub fn spawn(repository: RepositoryBase) -> HubConnection {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<HubPackage>(32);

        tokio::spawn(async move {
            let mut manager = HubServer::new(repository);
            while let Some(package) = rx.recv().await {
                match package {
                    HubPackage::NotifyUser { user_id, message } => {
                        manager.notify_user(&user_id, &message, true).await
                    }
                    HubPackage::NotifyGroup {
                        group_name,
                        message,
                    } => manager.notify_group(&group_name, &message).await,
                    HubPackage::AddClient {
                        user_id,
                        group_name,
                        client,
                    } => manager.add_client(user_id, group_name, client).await,
                }
            }
        });

        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let tx = tx_clone;
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;

                tx.send(HubPackage::NotifyGroup {
                    group_name: "ic-system-ping".to_owned(),
                    message: "ping".to_owned(),
                })
                .await
                .unwrap();
            }
        });

        HubConnection::new(tx)
    }

    fn is_internal_system(group_name: &str) -> bool {
        group_name.starts_with("ic-system-")
    }

    async fn add_client(
        &mut self,
        user_id: String,
        group_name: Option<String>,
        tx: Sender<Result<Event, Infallible>>,
    ) {
        self.add_user(user_id.clone(), group_name).await;

        let messages = self
            .repository
            .message_select_all_by_user_external_id(&user_id)
            .await
            .unwrap();

        if messages.is_empty() {
            self.notify(&tx, "ping").await.unwrap();
        } else {
            for message in messages {
                self.notify(&tx, &message.payload).await.unwrap();
            }
        }

        if let Some(user_idx) = self.users.get_mut(&user_id) {
            user_idx.push(tx);
        }
    }

    async fn add_user(&mut self, user_id: String, group_name: Option<String>) {
        self.repository
            .user_insert_if_not_exists(&User::new(user_id.clone()))
            .await
            .unwrap();
        if let Some(group) = group_name {
            self.add_user_to_group(user_id.clone(), group).await;
        }

        self.users.entry(user_id).or_insert_with(Vec::new);
    }

    async fn add_user_to_group(&mut self, user_id: String, group_name: String) {
        if !Self::is_internal_system(&group_name) {
            let group_id = self
                .repository
                .group_insert_if_not_exists(&Group::new(group_name.clone()))
                .await
                .unwrap();

            if let Some(user_db_id) = self.repository.user_get_id(&user_id).await.unwrap() {
                self.repository
                    .user_attach_group(user_db_id, group_id)
                    .await
                    .unwrap();
            } else {
                tracing::warn!("User {user_id} not found!");
            }
        }

        if let Some(group_users) = self.groups.get_mut(&group_name) {
            if !group_users.contains(&user_id) {
                group_users.push(user_id);
            }
        } else {
            self.groups.insert(group_name, vec![user_id]);
        }
    }

    async fn notify(
        &self,
        client: &SseSender,
        msg: &str,
    ) -> Result<(), SendError<Result<axum::response::sse::Event, Infallible>>> {
        let event = Event::default()
            .json_data(HubEventMessage::new(
                String::from(""),
                String::from(msg),
                false,
            ))
            .unwrap();

        client.send(Ok(event)).await
    }

    async fn notify_user(&mut self, user_id: &str, msg: &str, persist: bool) {
        if persist {
            let message_id = self
                .repository
                .message_insert(&Message::new(String::from(msg)))
                .await
                .unwrap();

            if let Some(user_db_id) = self.repository.user_get_id(&user_id).await.unwrap() {
                self.repository
                    .user_attach_message(user_db_id, message_id)
                    .await
                    .unwrap();
            } else {
                tracing::warn!("User {user_id} not found!");
            }
        }

        let mut ok_clients = Vec::new();
        {
            let users = self.users.clone();

            if let Some(user_clients) = users.get(user_id) {
                let clients = user_clients.clone();

                for client in clients {
                    if self.notify(&client, msg).await.is_ok() {
                        ok_clients.push(client.clone());
                    } else {
                        eprintln!("Sse disconected");
                    }
                }
            } else {
                tracing::trace!("User {user_id} not found");
            }
        }

        self.users
            .entry(user_id.to_owned())
            .and_modify(|v| *v = ok_clients);
    }

    async fn notify_group(&mut self, group_name: &str, msg: &str) {
        if !Self::is_internal_system(group_name) {
            let message_id = self
                .repository
                .message_insert(&Message::new(String::from(msg)))
                .await
                .unwrap();

            self.repository
                .group_attach_message(&group_name, message_id)
                .await
                .unwrap();
        }

        let mut messages = Vec::new();
        {
            let groups = &self.groups;

            if let Some(groups) = groups.get(group_name) {
                for user_id in groups {
                    messages.push((user_id.clone(), msg));
                }
            } else {
                tracing::trace!("Group {group_name} not found");
            }
        }

        for (user_id, msg) in messages {
            self.notify_user(&user_id, msg, false).await;
        }
    }

    // pub fn remove_client(&mut self, client: &DummyClient) {
    //     let (_, clients) = self
    //         .users
    //         .iter_mut()
    //         .find(|(_, y)| y.iter().find(|z| z.device_id == client.device_id).is_some())
    //         .unwrap();

    //     let client_index = clients
    //         .iter()
    //         .position(|p| p.device_id == client.device_id)
    //         .unwrap();
    //     clients.remove(client_index);
    // }

    fn remove_user(&mut self, user_id: &str) {
        self.users.remove(user_id);
    }

    fn remove_group(&mut self, group_id: &str) {
        self.groups.remove(group_id);
    }
}
