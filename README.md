# Rust Time Communication Hub

Simple  Real Time Communication Hub

## Test Client
```javascript
const EventSource = require('eventsource');
const eventSource = new EventSource("http://localhost:4500/sse?user_id=ricardo");
eventSource.onmessage = (e) => { console.log(e.data); }
```