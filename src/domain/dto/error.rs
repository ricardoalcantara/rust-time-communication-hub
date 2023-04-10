use std::collections::HashMap;

use axum::{
    extract::rejection::TypedHeaderRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::repository::repository_base::RepositoryError;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<T> {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<HashMap<String, T>>,
}

pub type HttpResult<T = ()> = Result<T, HttpError>;

#[derive(Debug)]
pub struct HttpError {
    pub message: Option<HashMap<String, String>>,
    pub status_code: StatusCode,
}

impl HttpError {
    pub fn new_message(message: String, status_code: StatusCode) -> HttpError {
        HttpError {
            message: Some(HashMap::from([(String::from("message"), message)])),
            status_code,
        }
    }

    pub fn status_code(status_code: StatusCode) -> HttpError {
        HttpError {
            message: None,
            status_code,
        }
    }

    pub fn not_found() -> HttpError {
        HttpError {
            message: None,
            status_code: StatusCode::NOT_FOUND,
        }
    }

    pub fn bad_request(message: &str) -> HttpError {
        HttpError::new_message(String::from(message), StatusCode::BAD_REQUEST)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        if let Some(mut error) = self.message {
            let error_message = if error.len() > 0 && error.contains_key("message") {
                error.remove("message").unwrap()
            } else {
                format!("{}", self.status_code)
            };

            let detail = if error.len() > 0 { Some(error) } else { None };

            let body = Json(ErrorResponse {
                error: error_message,
                detail,
            });
            (self.status_code, body).into_response()
        } else {
            (self.status_code).into_response()
        }
    }
}

impl From<std::env::VarError> for HttpError {
    fn from(error: std::env::VarError) -> Self {
        tracing::error!("{error}");
        HttpError::status_code(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<jsonwebtoken::errors::Error> for HttpError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        tracing::error!("{error}");
        HttpError::bad_request("Invalid token")
    }
}

impl From<TypedHeaderRejection> for HttpError {
    fn from(error: TypedHeaderRejection) -> Self {
        tracing::debug!("{error}");
        HttpError::bad_request("Missing credentials")
    }
}

impl From<RepositoryError> for HttpError {
    fn from(error: RepositoryError) -> Self {
        tracing::debug!("{error}");
        HttpError::bad_request("Database error")
    }
}
