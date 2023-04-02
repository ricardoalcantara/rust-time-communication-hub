use axum::{
    routing::{get, put},
    Router,
};

pub mod controller;
pub mod dto;
pub mod service;

pub fn route() -> Router {
    Router::new()
        .route("/api/message/notify", get(self::controller::sse))
        .route("/api/message/acknowledge", get(self::controller::sse))
}
