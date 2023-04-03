use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};

use crate::{core::jwt::Jwt, domain::dto::error::HttpError};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub jti: String,
    pub sub: String,
    pub exp: i64,
}

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            req.extract::<TypedHeader<Authorization<Bearer>>>().await?;

        let token = bearer.token();
        let claims: Claims = Jwt::new().decode(token)?;
        Ok(claims)
    }
}
