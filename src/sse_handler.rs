use crate::hub::Hub;
use axum::{
    response::sse::{Event, KeepAlive, Sse},
    Extension, TypedHeader,
};
use futures::stream::Stream;
use std::convert::Infallible;

pub async fn handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    Extension(hub): Extension<Hub>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::info!("`{}` connected", user_agent.as_str());

    let rx = hub.new_client().await;

    let stream = tokio_stream::wrappers::ReceiverStream::<Result<Event, Infallible>>::new(rx);

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn notify(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    Extension(hub): Extension<Hub>,
) -> &'static str {
    tracing::info!("`{}` notify", user_agent.as_str());

    hub.broadcast("Hello, world!").await;
    "Hello, world!"
}
