use axum::{routing::post, Router};

pub mod controller;
pub mod dto;
pub mod service;

pub fn route() -> Router {
    Router::new()
        .route("/api/message/notify", post(self::controller::notify))
        .route("/api/message/acknowledge", post(self::controller::empty))
}
