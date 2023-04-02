use axum::{
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    Json,
};
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};

use crate::{core::jwt::Jwt, domain::dto::error::HttpResult};

use super::dto::{auth::AccessToken, claims::Claims};

pub async fn token() -> HttpResult<impl IntoResponse> {
    let user_id = uuid::Uuid::new_v4().to_string();

    let expire_at = chrono::Duration::seconds(5);
    let exp = chrono::Utc::now()
        .checked_add_signed(expire_at)
        .expect("valid timestamp")
        .timestamp();

    let jti = uuid::Uuid::new_v4().to_string();
    let claims = Claims {
        jti: jti.clone(),
        sub: user_id,
        exp,
    };

    let access_token = Jwt::from_env("JWT_SSE_USER_SECRET").encode(claims)?;

    let access_token = AccessToken {
        access_token,
        token_type: String::from("Bearer"),
    };

    Ok((StatusCode::OK, Json(access_token)))
}
