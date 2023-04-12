use crate::{
    core::jwt::Jwt,
    domain::dto::error::{HttpError, HttpResult},
};

use super::dto::{
    auth::{AccessToken, TokenRequest},
    claims::Claims,
};

pub struct AuthService {}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService {}
    }

    pub fn token(self, login: TokenRequest) -> HttpResult<AccessToken> {
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

        Ok(AccessToken {
            access_token,
            token_type: String::from("Bearer"),
        })
    }
}
