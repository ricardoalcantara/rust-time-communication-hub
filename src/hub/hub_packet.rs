use axum::response::sse::Event;
use std::convert::Infallible;
use tokio::sync::mpsc::Sender;

type SseClient = Sender<Result<Event, Infallible>>;

#[derive(Debug)]
pub enum HubPackage {
    NotifyUser {
        user_id: String,
        message: String,
    },
    NotifyGroup {
        group_name: String,
        message: String,
    },
    AddClient {
        user_id: String,
        group_name: Option<String>,
        client: SseClient,
    },
}
