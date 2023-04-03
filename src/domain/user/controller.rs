use crate::{
    domain::{auth::dto::claims::Claims, dto::error::HttpResult},
    hub::hub_connection::HubConnection,
};
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse};

pub async fn empty() -> HttpResult<&'static str> {
    Ok("Hello World")
}

pub async fn add_user(
    _claims: Claims,
    Extension(_hub_connection): Extension<HubConnection>,
) -> HttpResult<impl IntoResponse> {
    Ok(StatusCode::ACCEPTED)
}
