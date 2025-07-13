use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on {addr:?}");

    axum::serve(listner, router()).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/hello", get("World!"))
        .route("/404", get(not_found))
        .route("/user", get(user_handler))
}

async fn handler() -> &'static str {
    "Hello na eiei"
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Error Not Found")
}

async fn user_handler() -> impl IntoResponse {
    let users = vec![
        User {
            id: String::from("1"),
            name: String::from("Alice"),
        },
        User {
            id: "2".into(),
            name: "Bob".into(),
        },
    ];

    (StatusCode::OK, Json(users)).into_response()
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
}
