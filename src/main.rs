use axum::{
    extract::{Path, Json},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// GET /hello
async fn hello() -> &'static str {
    "Hello, Axum!"
}

// GET /user/:id
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    let user = User {
        id,
        name: "Alice".to_string(),
    };
    Json(user)
}

// POST /user
async fn create_user(Json(payload): Json<User>) -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(payload))
}

#[tokio::main]
async fn main() {
    // Build the app with routes
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user));

    // Define the address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Axum server running at http://{addr}");

    // Start the server using Hyper via Axum
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
