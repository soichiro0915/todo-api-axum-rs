use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // logging setup
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello world!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{"name": "John"}"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot cover USer instance.");
        assert_eq!(
            user,
            User {
                id: 1,
                name: "John".to_string()
            }
        );
    }
}
