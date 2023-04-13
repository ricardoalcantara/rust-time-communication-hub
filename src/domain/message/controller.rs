use super::{dto::message::NofityMessage, service::MessageService};
use crate::{
    domain::{
        auth::dto::claims::Claims,
        dto::{error::HttpResult, validator::ValidatedJson},
    },
    hub::hub_connection::HubConnection,
};
use axum::{http::StatusCode, response::IntoResponse, Extension};

pub async fn notify(
    _claims: Claims,
    Extension(hub_connection): Extension<HubConnection>,
    ValidatedJson(notify_message): ValidatedJson<NofityMessage>,
) -> HttpResult<impl IntoResponse> {
    let message_service = MessageService::new(hub_connection);
    message_service.notify(notify_message).await?;

    Ok(StatusCode::ACCEPTED)
}

pub async fn empty() -> HttpResult<&'static str> {
    Ok("Hello World")
}
