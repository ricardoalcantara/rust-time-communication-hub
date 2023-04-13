use super::service::UserService;
use crate::{
    domain::{auth::dto::claims::Claims, dto::error::HttpResult},
    hub::hub_connection::HubConnection,
};
use axum::{extract::Path, Extension, Json};
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

pub async fn token(
    _claims: Claims,
    Path(user_id): Path<String>,
    Extension(_hub_connection): Extension<HubConnection>,
) -> HttpResult<impl IntoResponse> {
    let user_service = UserService::new();
    let access_token = user_service.token(user_id)?;
    Ok((StatusCode::OK, Json(access_token)))
}
