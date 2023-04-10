use crate::error::AppResult;
use crate::repository::repository_base::RepositoryBase;
use crate::servers::shutdown_signal;
use crate::{domain, hub::hub_connection::HubConnection};
use axum::{routing::get, Extension, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

pub async fn start(hub_connection: HubConnection, repository: RepositoryBase) -> AppResult {
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
        .merge(domain::sse::route())
        .layer(Extension(repository))
        .layer(Extension(hub_connection))
        .layer(
            CorsLayer::new()
                .allow_methods(tower_http::cors::AllowMethods::mirror_request())
                .allow_headers(tower_http::cors::AllowHeaders::mirror_request())
                .allow_origin(origin),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 4500));
    tracing::info!("listening on {}", addr);
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}

async fn handler() -> &'static str {
    "healthcheck!"
}
