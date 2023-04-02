use axum::{
    routing::{delete, get},
    Router,
};

pub mod controller;
pub mod dto;
pub mod service;

pub fn route() -> Router {
    Router::new()
        .route(
            "/api/user",
            get(self::controller::empty).post(self::controller::empty),
        )
        .route("/api/user/{user_id}", delete(self::controller::empty))
        .route("/api/user/{user_id}/token", get(self::controller::empty))
}
