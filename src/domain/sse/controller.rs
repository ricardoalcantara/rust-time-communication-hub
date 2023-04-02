use crate::{
    core::jwt::Jwt,
    domain::{auth::dto::claims::Claims, dto::error::HttpResult},
    hub::hub_connection::HubConnection,
};
use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
    Extension,
};
use futures_util::stream::Stream;
use serde::Deserialize;
use std::convert::Infallible;
use tokio_stream::wrappers::ReceiverStream;

// TODO: Temporary
#[derive(Deserialize)]
pub struct Params {
    pub token: String,
}

pub async fn sse(
    Query(params): Query<Params>,
    Extension(hub_connection): Extension<HubConnection>,
) -> HttpResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let claims: Claims = Jwt::from_env("JWT_SSE_USER_SECRET").decode(&params.token)?;

    tracing::info!("`{}` connected", &claims.sub);
    let rx = hub_connection
        .new_client(claims.sub, Some("Default".to_owned()))
        .await;

    let stream = ReceiverStream::<Result<Event, Infallible>>::new(rx);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
