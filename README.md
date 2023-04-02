# Rust Time Communication Hub

Simple Real Time Communication Hub

## Test Client

```javascript
const EventSource = require('eventsource');
const eventSource = new EventSource("http://localhost:4500/sse?user_id=ricardo");
eventSource.onmessage = (e) => { console.log(e.data); }
```


------------------------------------------------------------------------------------------

#### Auth

<details>
<summary><code>POST</code> <code><b>/api/auth/token</b></code> <code>{{description}}</code></summary>

##### Parameters

> None

> | name      |  type     | data type               | description                                                           |
> |-----------|-----------|-------------------------|-----------------------------------------------------------------------|
> | client_id      |  required | object JSON    | N/A  |
> | client_secret      |  required | object JSON    | N/A  |

##### Payload

```json
{
    "client_id": "",
    "client_secret": ""
}
```

##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `application/json`                | `{"access_token":"xxx","type":"bearer"}`                                |
> | `400`         | `application/json`                | `{"error":"Invalid client credentials"}`                            |

##### Example cURL

> curl -X POST -H "Content-Type: application/json" --data @post.json http://localhost:4501/api/auth/token


</details>

------------------------------------------------------------------------------------------

#### Message

<details>
<summary><code>POST</code> <code><b>/api/message/notify</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>
<details>
<summary><code>POST</code> <code><b>/api/message/acknowledge</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>

------------------------------------------------------------------------------------------

#### User

<details>
<summary><code>GET</code> <code><b>/api/user</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>
<details>
<summary><code>POST</code> <code><b>/api/user</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>
<details>
<summary><code>DELETE</code> <code><b>/api/user/{user_id}</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>
<details>
<summary><code>GET</code> <code><b>/api/user/{user_id}/token</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>

------------------------------------------------------------------------------------------

#### Group

<details>
<summary><code>GET</code> <code><b>/api/group</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>
<details>
<summary><code>POST</code> <code><b>/api/group</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>
<details>
<summary><code>DELETE</code> <code><b>/api/group/{group_id}</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>

------------------------------------------------------------------------------------------

#### SSE

<details>
<summary><code>GET</code> <code><b>/sse</b></code> <code>{{description}}</code></summary>

##### Parameters

> Empty

##### Responses

> Empty

##### Example cURL

> Empty

</details>

------------------------------------------------------------------------------------------