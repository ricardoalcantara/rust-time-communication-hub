use axum::{routing::get, Router};

pub mod controller;
pub mod dto;
pub mod service;

pub fn route() -> Router {
    Router::new().route("/sse", get(self::controller::sse))
}
