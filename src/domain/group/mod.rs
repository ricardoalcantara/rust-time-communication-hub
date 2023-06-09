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
            "/api/group",
            get(self::controller::empty).post(self::controller::empty),
        )
        .route("/api/group/{group_id}", delete(self::controller::empty))
}
