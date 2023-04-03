use super::error::ErrorResponse;
use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug)]
pub enum ServerError {
    ValidationError(validator::ValidationErrors),
    AxumJsonRejection(axum::extract::rejection::JsonRejection),
}

impl From<validator::ValidationErrors> for ServerError {
    fn from(error: validator::ValidationErrors) -> Self {
        tracing::debug!("{error}");
        ServerError::ValidationError(error)
    }
}

impl From<axum::extract::rejection::JsonRejection> for ServerError {
    fn from(error: axum::extract::rejection::JsonRejection) -> Self {
        tracing::debug!("{error}");
        ServerError::AxumJsonRejection(error)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                let mut message = HashMap::new();
                for (field, validation_error) in e.field_errors() {
                    for validation_error_item in validation_error {
                        let value = match validation_error_item.message.as_ref() {
                            Some(x) => x.to_string().replace('"', ""),
                            None => format!("Field {field} has an invalid value"),
                        };

                        message.insert(field.to_owned(), value);
                    }
                }
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Validation error".to_owned(),
                        detail: Some(message),
                    }),
                )
                    .into_response()
            }
            ServerError::AxumJsonRejection(e) => {
                let error = match e {
                    axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                        String::from("Expected request with `Content-Type: application/json`")
                    }
                    _ => format!("Invalid JSON format {}", e),
                };
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::<()> {
                        error,
                        detail: None,
                    }),
                )
                    .into_response()
            }
        }
    }
}
