use super::{dto::auth::TokenRequest, service::AuthService};
use crate::domain::dto::{error::HttpResult, validator::ValidatedJson};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn token(
    ValidatedJson(login): ValidatedJson<TokenRequest>,
) -> HttpResult<impl IntoResponse> {
    let auth_service = AuthService::new();
    let access_token = auth_service.token(login)?;

    Ok((StatusCode::OK, Json(access_token)))
}
