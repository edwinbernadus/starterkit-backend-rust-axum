use std::sync::Arc;

use db_utility::{Album};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::MethodRouter;
use axum::Router;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    routing::{get, post},
    Json,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
pub mod db_utility;
struct AppState {
    // ...
}
// #[derive(Deserialize)]
#[derive(Deserialize, Serialize)]
struct CreateUserPayload {
    // ...
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState { /* ... */ });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        //web_socket
        .route("/ws", get(websocket_handler))
        //get
        .route("/hello2", get(|| async { "Hello, World! 2" }))
        .merge(post_foo2())
        //get_header
        .route("/info_header", get(move |header| info_header(header)))
        //post
        .route(
            "/users",
            post(move |header, body| create_user(body, header)),
        )
        //routing_id
        .route(
            "/users/:id",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |path| get_user(path, shared_state)
            }),
        )
        .route(
            "/db/total_rows",
            get(|| async {
                let output = db_utility::get_total_albums().await;
                match output {
                    Ok(total) => format!("{}", total),
                    Err(err) => format!("{}", err.to_string()),
                }
            }),
        )
        .route(
            "/db/query_all",
            get(|| async {
                let output = db_utility::get_list_albums().await;
                match output {
                    Ok(albums) => {
                        let json1 = to_string(&albums);
                        match json1 {
                            Ok(json) => {
                                format!("{}", json)
                            }
                            Err(err) => format!("{}", err.to_string()),
                        }
                    }
                    Err(err) => format!("{}", err.to_string()),
                }
            }),
        )
        .route(
            "/db/insert",
            get(|| async {
                let result = db_utility::insert_album("title 123").await;
                match result {
                    Ok(_) => format!("{}", "insert-ok"),
                    Err(err) => format!("{}", err.to_string()),
                }
            }),
        )
        .route(
            "/db/update/:id",
            post(|Path(id): Path<i64>, Json(album): Json<Album> |  async move  {
                let result = db_utility::update_album(id, &album.title).await;
                match result {
                    Ok(_) => format!("{}", "update-ok"),
                    Err(err) => format!("{}", err.to_string()),
                }
            }),
        )
        .route(
            // "/db/delete",
            "/db/delete/:id",
            get( |Path(id): Path<i64>| async move{
                let result = db_utility::delete_album(id).await;
                match result {
                    Ok(_) => format!("{}", "delete-ok"),
                    Err(err) => format!("{}", err.to_string()),
                }
            }),
        )
        .route("/foo/bar", get(foo_bar));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// which calls one of these handlers
async fn foo_bar() {}

fn post_foo2() -> Router {
    async fn handler() -> &'static str {
        "Hi from `POST /foo`"
    }
    route("/foo", post(handler))
}

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) -> String {
    println!("user_id = {}", user_id);
    let output = format!("user_id = {}", user_id);
    return output;
}

async fn info_header(headers: HeaderMap) -> impl IntoResponse {
    let auth = headers.get("authorization");
    let username = match auth {
        None => "no_data",
        Some(x) => x.to_str().unwrap(),
    };
    username.to_string()
}

async fn create_user(Json(payload): Json<CreateUser>, headers: HeaderMap) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    //json_convert
    (StatusCode::CREATED, Json(user))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    // State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket))
}

async fn websocket(stream: WebSocket) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    while let Some(Ok(message)) = receiver.next().await {
        match message.into_text() {
            Ok(msg) => {
                let reply = format!("{} {}", "reply: ", msg);
                let _ = sender.send(Message::Text(reply)).await;
            }
            Err(_) => {
                let _ = sender
                    .send(Message::Text("Error: message is not text".to_string()))
                    .await;
            }
        }
    }
}
