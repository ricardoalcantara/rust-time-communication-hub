use crate::hub::Hub;
use crate::hub_manager::{HubManager, MessageType};
use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use error::AppResult;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::CorsLayer;

mod error;
mod hub;
mod hub_manager;
mod sse_handler;

#[tokio::main]
async fn main() -> AppResult {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let tx = HubManager::spawn();
    let hub = Hub::new(tx);

    let origin = if let Ok(origin) = std::env::var("ORIGIN") {
        if origin == "*" {
            tower_http::cors::AllowOrigin::mirror_request()
        } else {
            tower_http::cors::AllowOrigin::exact(origin.parse()?)
        }
    } else {
        tower_http::cors::AllowOrigin::default()
    };

    let app = Router::new()
        .route("/", get(handler))
        .route("/notify", get(sse_handler::notify))
        .route("/sse", get(sse_handler::handler))
        .layer(Extension(hub))
        .layer(
            CorsLayer::new()
                .allow_methods(tower_http::cors::AllowMethods::mirror_request())
                .allow_headers(tower_http::cors::AllowHeaders::mirror_request())
                .allow_credentials(tower_http::cors::AllowCredentials::yes())
                .allow_origin(origin),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 4500));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn handler() -> &'static str {
    "Hello, world!"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
