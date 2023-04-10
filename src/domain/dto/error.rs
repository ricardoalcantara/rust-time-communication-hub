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

    pub fn internal_server_error(message: &str) -> HttpError {
        HttpError::new_message(String::from(message), StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn unauthorized(message: &str) -> HttpError {
        HttpError::new_message(String::from(message), StatusCode::UNAUTHORIZED)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let mut detail = self.message;
        let error_message = if let Some(error) = detail.as_mut() {
            if !error.is_empty() && error.contains_key("message") {
                error.remove("message").unwrap()
            } else {
                format!("{}", self.status_code)
            }
        } else {
            format!("{}", self.status_code)
        };

        let body = Json(ErrorResponse {
            error: error_message,
            detail,
        });
        (self.status_code, body).into_response()
    }
}

impl From<std::env::VarError> for HttpError {
    fn from(error: std::env::VarError) -> Self {
        tracing::error!("{error}");
        HttpError::internal_server_error("EnvVar error")
    }
}

impl From<RepositoryError> for HttpError {
    fn from(error: RepositoryError) -> Self {
        tracing::debug!("{error}");
        HttpError::internal_server_error("Repository error")
    }
}
