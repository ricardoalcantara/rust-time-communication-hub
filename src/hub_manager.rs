use axum::response::sse::Event;
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum MessageType {
    NotifyUser {
        user_id: String,
        message: String,
    },
    NotifyGroup {
        group_id: String,
        message: String,
    },
    AddClient {
        user_id: String,
        group_id: Option<String>,
        client: SseSender,
    },
}

type Db<T> = HashMap<String, Vec<T>>;
type ArcMutexDb<T> = Arc<Mutex<Db<T>>>;
type SseSender = Sender<Result<Event, Infallible>>;

#[derive(Debug, Default)]
pub struct HubManager {
    pub users: Db<SseSender>,
    pub groups: Db<String>,
}

impl HubManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn spawn() -> Sender<MessageType> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<MessageType>(32);

        tokio::spawn(async move {
            let mut manager = HubManager::new();
            while let Some(message) = rx.recv().await {
                match message {
                    MessageType::NotifyUser { user_id, message } => {
                        manager.notify_user(&user_id, &message).await
                    }
                    MessageType::NotifyGroup { group_id, message } => {
                        manager.notify_group(&group_id, &message).await
                    }
                    MessageType::AddClient {
                        user_id,
                        group_id,
                        client,
                    } => manager.add_client(user_id, group_id, client),
                }
            }
        });

        tx
    }

    fn add_client(
        &mut self,
        user_id: String,
        group_id: Option<String>,
        tx: Sender<Result<Event, Infallible>>,
    ) {
        self.add_user(user_id.clone(), group_id);

        if let Some(user_idx) = self.users.get_mut(&user_id) {
            user_idx.push(tx);
        }
    }

    fn add_user(&mut self, user_id: String, group_id: Option<String>) {
        if let Some(group) = group_id {
            self.add_user_to_group(user_id.clone(), group.clone());
        }

        if !self.users.contains_key(&user_id) {
            self.users.insert(user_id, vec![]);
        }
    }

    fn add_user_to_group(&mut self, user_id: String, group_id: String) {
        if let Some(group_users) = self.groups.get_mut(&group_id) {
            if !group_users.contains(&user_id) {
                group_users.push(user_id);
            }
        } else {
            self.groups.insert(group_id, vec![user_id]);
        }
    }

    async fn notify_user(&mut self, user_id: &str, msg: &str) {
        let mut ok_clients = Vec::new();
        {
            let users = self.users.clone();

            if let Some(user_clients) = users.get(user_id) {
                let clients = user_clients.clone();

                for client in clients {
                    if client.send(Ok(Event::default().data(msg))).await.is_ok() {
                        ok_clients.push(client.clone());
                    }
                }
            } else {
                eprintln!("User {user_id} not found");
            }
        }

        self.users
            .entry(user_id.to_owned())
            .and_modify(|v| *v = ok_clients);
    }

    async fn notify_group(&mut self, group_id: &str, msg: &str) {
        let mut messages = Vec::new();
        {
            let groups = &self.groups;

            if let Some(groups) = groups.get(group_id) {
                for user_id in groups {
                    messages.push((user_id.clone(), msg));
                }
            } else {
                eprintln!("Group {group_id} not found");
            }
        }

        for (user_id, msg) in messages {
            self.notify_user(&user_id, msg).await;
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
