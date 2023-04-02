use crate::domain::dto::error::HttpResult;
// use axum::http::StatusCode;

pub async fn empty() -> HttpResult<&'static str> {
    Ok("Hello World")
}
