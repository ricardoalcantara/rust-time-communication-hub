use axum::{
    routing::{get, put},
    Router,
};

pub mod controller;
pub mod dto;
pub mod service;

pub fn route() -> Router {
    Router::new().route("/api/auth/token", get(self::controller::token))
}
