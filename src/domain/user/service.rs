use crate::{
    core::jwt::Jwt,
    domain::{
        auth::dto::{auth::AccessToken, claims::Claims},
        dto::error::{HttpError, HttpResult},
    },
};

pub struct UserService {}

impl UserService {
    pub fn new() -> UserService {
        UserService {}
    }

    pub fn token(self, user_id: String) -> HttpResult<AccessToken> {
        let expire_at = chrono::Duration::minutes(5);
        let exp = chrono::Utc::now()
            .checked_add_signed(expire_at)
            .expect("valid timestamp")
            .timestamp();

        let sub = user_id;
        let jti = uuid::Uuid::new_v4().to_string();
        let claims = Claims { jti, sub, exp };

        let access_token = Jwt::from_env("JWT_SSE_USER_SECRET")
            .encode(claims)
            .map_err(|_| HttpError::internal_server_error("Jwt encode error"))?;

        Ok(AccessToken {
            access_token,
            token_type: String::from("Bearer"),
        })
    }
}
