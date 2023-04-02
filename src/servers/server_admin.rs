use crate::domain;
use crate::error::AppResult;
use crate::hub::hub_connection::HubConnection;
use crate::servers::shutdown_signal;
use axum::Extension;
use axum::{routing::get, Router};
use std::net::SocketAddr;

pub async fn start(hub_connection: HubConnection) -> AppResult {
    let app = Router::new()
        .route("/", get(handler))
        .merge(domain::auth::route())
        .merge(domain::group::route())
        .merge(domain::message::route())
        .merge(domain::user::route())
        .layer(Extension(hub_connection));
    let addr = SocketAddr::from(([0, 0, 0, 0], 4501));
    tracing::info!("listening on {}", addr);
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}

async fn handler() -> &'static str {
    "healthcheck!"
}
