use axum::{routing::{get, post}, Router, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // logging setup
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root)).route("/users", post(create_user));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn create_user(
    // Deserialize the request body as JSON
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1,
        name: payload.name,
    };
    // Serialize the response as JSON
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}
