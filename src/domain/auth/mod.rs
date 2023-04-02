use axum::{routing::post, Router};

pub mod controller;
pub mod dto;
pub mod service;

pub fn route() -> Router {
    Router::new().route("/api/auth/token", post(self::controller::token))
}
