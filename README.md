# Rust Time Communication Hub

Simple  Real Time Communication Hub

## Test Client
```javascript
const EventSource = require('eventsource');
const eventSource = new EventSource("http://localhost:4500/sse?user_id=ricardo");
eventSource.onmessage = (e) => { console.log(e.data); }
```


```
:4501/api/auth/token
{
    id: user_id
    group: Some([groupA])
}


:4501/api/message/notify
{
    ids: Some([]),
    groups: Some([])
    message: "{}"
}

:4501/api/message/acknowledge
{
    message_id: uuid
}


api/sse_token

token=xxx
:4500/sse?token=xxx
```
