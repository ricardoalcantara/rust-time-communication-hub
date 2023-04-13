use super::dto::sse::SseParams;
use crate::{
    core::jwt::Jwt,
    domain::{
        auth::dto::claims::Claims,
        dto::error::{HttpError, HttpResult},
    },
    hub::hub_connection::HubConnection,
};
use axum::response::sse::Event;
use std::convert::Infallible;
use tokio_stream::wrappers::ReceiverStream;

pub struct SseService {}

impl SseService {
    pub fn new() -> SseService {
        SseService {}
    }

    pub async fn sse(
        self,
        params: SseParams,
        hub_connection: HubConnection,
    ) -> HttpResult<ReceiverStream<Result<Event, Infallible>>> {
        let claims: Claims = Jwt::from_env("JWT_SSE_USER_SECRET")
            .decode(&params.token)
            .map_err(|_| HttpError::unauthorized("Invalid token"))?;

        tracing::info!("`{}` connected", &claims.sub);
        let rx = hub_connection.new_client(claims.sub, None).await;

        let stream: ReceiverStream<Result<Event, Infallible>> = ReceiverStream::new(rx);
        Ok(stream)
    }
}
