use crate::hub::Hub;
use axum::{
    extract::Query,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    Extension, TypedHeader,
};
use axum_macros::debug_handler;
use futures::stream::Stream;
use serde::Deserialize;
use std::convert::Infallible;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Deserialize)]
pub struct Params {
    pub user_id: String,
}

#[debug_handler]
pub async fn handler(
    Query(params): Query<Params>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    Extension(hub): Extension<Hub>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::info!("`{}` connected", user_agent.as_str());

    let user_id = params.user_id;
    let rx = hub.new_client(user_id, Some("Default".to_owned())).await;

    let stream = ReceiverStream::<Result<Event, Infallible>>::new(rx);

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[debug_handler]
pub async fn notify(Query(params): Query<Params>, Extension(hub): Extension<Hub>) -> StatusCode {
    let user_id = params.user_id;
    println!("{user_id}");

    hub.notify_user(user_id, "Hello, world!".to_owned()).await;
    hub.notify_group("Default".to_owned(), "Hello, world!".to_owned())
        .await;

    StatusCode::NO_CONTENT
}
