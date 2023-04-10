use super::dto::{
    auth::{AccessToken, TokenRequest},
    claims::Claims,
};
use crate::{
    core::jwt::Jwt,
    domain::dto::{
        error::{HttpError, HttpResult},
        validator::ValidatedJson,
    },
};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn token(
    ValidatedJson(login): ValidatedJson<TokenRequest>,
) -> HttpResult<impl IntoResponse> {
    if login.client_id != std::env::var("CLIENT_ID")?
        || login.client_secret != std::env::var("CLIENT_SECRET")?
    {
        return Err(HttpError::bad_request("Client id or Client secret invalid"));
    }

    let expire_at = chrono::Duration::minutes(5);
    let exp = chrono::Utc::now()
        .checked_add_signed(expire_at)
        .expect("valid timestamp")
        .timestamp();

    let sub = uuid::Uuid::new_v4().to_string();
    let jti = uuid::Uuid::new_v4().to_string();
    let claims = Claims { jti, sub, exp };

    let access_token = Jwt::from_env("JWT_SECRET")
        .encode(claims)
        .map_err(|_| HttpError::internal_server_error("Jwt encode error"))?;

    let access_token = AccessToken {
        access_token,
        token_type: String::from("Bearer"),
    };

    Ok((StatusCode::OK, Json(access_token)))
}
