use crate::common::api_client::ApiClient;
use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};

mod common;

#[tokio::test]
async fn connect_sse_sucess() {
    let client = ApiClient::new();
    let response = client.auth_token().await;
    let response_token = response.json().await;
    let mut es = client.get_event_source(response_token.access_token);
    let event = es.next().await.unwrap();

    assert!(event.is_ok());
    assert_eq!(Event::Open, event.unwrap());
}

#[tokio::test]
async fn connect_sse_ping_sucess() {
    let client = ApiClient::new();
    let response = client.auth_token().await;
    let response_token = response.json().await;
    let mut es = client.get_event_source(response_token.access_token);
    let event = es.next().await.unwrap();
    assert_eq!(Event::Open, event.unwrap());
    let event = es.next().await.unwrap();

    let Event::Message(message) = event.unwrap() else {
        panic!("");
    };
    assert_eq!("Hi!".to_owned(), message.data);

    let event = es.next().await.unwrap();
    let Event::Message(message) = event.unwrap() else {
        panic!("");
    };
    assert_eq!("ping".to_owned(), message.data);
}

#[tokio::test]
async fn connect_sse_fail() {
    let mut es = EventSource::get("http://localhost:4500/sse");
    let event = es.next().await.unwrap();

    assert!(event.is_err());
}

/*
let mut es = EventSource::get("http://localhost:4500/sse");
while let Some(event) = es.next().await {
    match event {
        Ok(Event::Open) => println!("Connection Open!"),
        Ok(Event::Message(message)) => println!("Message: {:#?}", message),
        Err(err) => {
            println!("Error: {:?}", err)
            // es.close();
        }
    }
}
Ok(())
 */
