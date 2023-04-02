use axum::{
    extract::Query,
    response::sse::{Event, KeepAlive, Sse},
    Extension,
};
use futures_util::stream::{self, Stream};
use serde::Deserialize;
use std::{convert::Infallible, time::Duration};
use tokio_stream::{wrappers::ReceiverStream, StreamExt as _};

use crate::hub::hub_connection::{self, HubConnection};

// TODO: Temporary
#[derive(Deserialize)]
pub struct Params {
    pub user_id: String,
}

pub async fn sse(
    Query(params): Query<Params>,
    Extension(hub_connection): Extension<HubConnection>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::info!("`{}` connected", &params.user_id);
    let rx = hub_connection
        .new_client(params.user_id, Some("Default".to_owned()))
        .await;

    let stream = ReceiverStream::<Result<Event, Infallible>>::new(rx);

    Sse::new(stream).keep_alive(KeepAlive::default())
}
