use crate::hub::hub_packet::HubPackage;
use axum::response::sse::Event;
use std::convert::Infallible;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct HubConnection {
    tx: Sender<HubPackage>,
}

impl HubConnection {
    pub fn new(tx: Sender<HubPackage>) -> Self {
        HubConnection { tx }
    }

    pub async fn new_client(
        &self,
        user_id: String,
        group_id: Option<String>,
    ) -> Receiver<Result<Event, Infallible>> {
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(4);

        self.tx
            .send(HubPackage::AddClient {
                user_id,
                group_id,
                client: tx,
            })
            .await
            .unwrap();

        rx
    }

    pub async fn notify_user(&self, user_id: String, message: String) {
        self.tx
            .send(HubPackage::NotifyUser { user_id, message })
            .await
            .unwrap()
    }
    pub async fn notify_group(&self, group_id: String, message: String) {
        self.tx
            .send(HubPackage::NotifyGroup { group_id, message })
            .await
            .unwrap()
    }
}
