use super::{dto::sse::SseParams, service::SseService};
use crate::{domain::dto::error::HttpResult, hub::hub_connection::HubConnection};
use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
    Extension,
};
use futures_util::stream::Stream;
use std::convert::Infallible;

pub async fn sse(
    Query(params): Query<SseParams>,
    Extension(hub_connection): Extension<HubConnection>,
) -> HttpResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let sse_service = SseService::new();
    let stream = sse_service.sse(params, hub_connection).await?;
    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
