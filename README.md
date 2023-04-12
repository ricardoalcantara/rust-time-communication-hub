# Rust Time Communication Hub

Simple Real Time Communication Hub

## Test Client

```javascript
const EventSource = require('eventsource');
const eventSource = new EventSource("http://localhost:4500/sse?user_id=ricardo");
eventSource.onmessage = (e) => { console.log(e.data); }
```

## Offline Mode (requires the offline feature)

The macros can be configured to not require a live database connection for compilation, but it requires a couple extra steps:

  - Run `cargo install sqlx-cli`.
  - In your project with DATABASE_URL set (or in a .env file) and the database server running, run `cargo sqlx prepare`.
  - Check the generated sqlx-data.json file into version control.
  - Don’t have DATABASE_URL set during compilation.

Your project can now be built without a database connection (you must omit DATABASE_URL or else it will still try to connect). To update the generated file simply run `cargo sqlx prepare` again.

To ensure that your sqlx-data.json file is kept up-to-date, both with the queries in your project and your database schema itself, run `cargo install sqlx-cli && cargo sqlx prepare --check` in your Continuous Integration script.

See the README for sqlx-cli for more information.


------------------------------------------------------------------------------------------

#### Auth

<details>
<summary><code>POST</code> <code><b>/api/auth/token</b></code> <code>{{description}}</code></summary>

##### Parameters

> None

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
> | `200`         | `application/json`                | `{"access_token":"xxx","type":"Bearer"}`                                |
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
<summary><code>GET</code> <code><b>/sse?token={token}</b></code> <code>{{description}}</code></summary>

##### Parameters

> | name              |  type     | data type      | description                                 |
> |-------------------|-----------|----------------|---------------------------------------------|
> | `token`           |  required | string         | The access token provided by the application|

##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `text/event-stream`               | Server Side Event Stream                                            |
> | `400`         | `application/json`                | `{"error": ""}`                                                     |


##### Example cURL

> curl -N http://localhost:4500/sse?token=xxx

</details>

------------------------------------------------------------------------------------------