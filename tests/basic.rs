use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};

#[tokio::test]
async fn connect_sse_sucess() {
    let mut es = EventSource::get("http://localhost:4500/sse?user_id=rust");
    let event = es.next().await.unwrap();

    assert!(event.is_ok());
    assert_eq!(Event::Open, event.unwrap());
}

#[tokio::test]
async fn connect_sse_ping_sucess() {
    let mut es = EventSource::get("http://localhost:4500/sse?user_id=rust");
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
