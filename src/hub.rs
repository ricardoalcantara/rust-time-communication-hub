use axum::response::sse::Event;
use std::sync::Mutex;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct Hub {
    inner: Arc<Mutex<HubManager>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub {
            inner: Arc::new(Mutex::new(HubManager::default())),
        }
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(
        &self,
        user_id: String,
        group_id: Option<String>,
    ) -> tokio::sync::mpsc::Receiver<Result<Event, Infallible>> {
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(4);

        tx.send(Ok(Event::default().data("hi!"))).await.unwrap();
        self.inner.lock().unwrap().add_client(tx, user_id, group_id);
        rx
    }

    pub async fn notify_user(&self, user_id: &str, msg: &str) {
        let mut hub_manager = self.inner.lock().unwrap().clone();
        hub_manager.notify_user(user_id, msg).await;
    }
    pub async fn notify_group(&self, group_id: &str, msg: &str) {
        let mut hub_manager = self.inner.lock().unwrap().clone();
        hub_manager.notify_group(group_id, msg).await;
    }
}

#[derive(Debug, Clone, Default)]
pub struct HubManager {
    pub users: HashMap<String, Vec<Sender<Result<Event, Infallible>>>>,
    pub groups: HashMap<String, Vec<String>>,
}

impl HubManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_client(
        &mut self,
        tx: Sender<Result<Event, Infallible>>,
        user_id: String,
        group_id: Option<String>,
    ) {
        self.add_user(user_id.clone(), group_id);

        if let Some(user_idx) = self.users.get_mut(&user_id) {
            user_idx.push(tx);
        }
    }

    pub fn add_user(&mut self, user_id: String, group_id: Option<String>) {
        if let Some(group) = group_id {
            self.add_user_to_group(user_id.clone(), group.clone());
        }

        if !self.users.contains_key(&user_id) {
            self.users.insert(user_id, vec![]);
        }
    }

    pub fn add_user_to_group(&mut self, user_id: String, group_id: String) {
        if let Some(group_users) = self.groups.get_mut(&group_id) {
            if !group_users.contains(&user_id) {
                group_users.push(user_id);
            }
        } else {
            self.groups.insert(group_id, vec![user_id]);
        }
    }

    pub async fn notify_user(&mut self, user_id: &str, msg: &str) {
        if let Some(user_clients) = self.users.get(user_id) {
            let clients = user_clients.clone();
            let mut ok_clients = Vec::new();

            for client in clients {
                if client.send(Ok(Event::default().data(msg))).await.is_ok() {
                    ok_clients.push(client.clone());
                }
            }

            self.users
                .entry(user_id.to_owned())
                .and_modify(|v| *v = ok_clients);
        } else {
            eprintln!("User {user_id} not found");
        }
    }

    pub async fn notify_group(&mut self, group_id: &str, msg: &str) {
        let mut messages = Vec::new();

        if let Some(groups) = self.groups.get(group_id) {
            for user_id in groups {
                messages.push((user_id.clone(), msg));
            }
        } else {
            eprintln!("Group {group_id} not found");
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

    pub fn remove_user(&mut self, user_id: &str) {
        self.users.remove(user_id);
    }

    pub fn remove_group(&mut self, group_id: &str) {
        self.groups.remove(group_id);
    }
}
