use axum::response::sse::Event;
use futures_util::future;
use parking_lot::Mutex;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::time::interval;

#[derive(Debug, Clone)]
pub struct Hub {
    inner: Arc<Mutex<HubInner>>,
}

#[derive(Debug, Clone, Default)]
struct HubInner {
    clients: Vec<tokio::sync::mpsc::Sender<Result<Event, Infallible>>>,
}

impl Hub {
    pub fn new() -> Self {
        let hub = Hub {
            inner: Arc::new(Mutex::new(HubInner::default())),
        };
        Hub::spawn_ping(hub.clone());
        hub
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast list if not.
    fn spawn_ping(hub: Hub) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));

            loop {
                interval.tick().await;
                hub.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        println!("active client {:?}", clients);

        let mut ok_clients = Vec::new();

        println!("okay active client {:?}", ok_clients);

        for client in clients {
            if client.send(Ok(Event::default().data("hi!"))).await.is_ok() {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self) -> tokio::sync::mpsc::Receiver<Result<Event, Infallible>> {
        println!("starting creation");
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(4);

        tx.send(Ok(Event::default().data("hi!"))).await.unwrap();
        println!("creating new clients success {:?}", tx);
        self.inner.lock().clients.push(tx);
        rx
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, msg: &str) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients
            .iter()
            .map(|client| client.send(Ok(Event::default().data(msg))));

        // try to send to all clients, ignoring failures
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = future::join_all(send_futures).await;
    }
}
