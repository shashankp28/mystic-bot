use axum::{ routing::{ get }, Router };
use mystic_bot::{ api::get::root::root_handler, bot::types::ServerState };
use std::{ collections::HashMap, net::SocketAddr, sync::{ Arc, Mutex } };

#[tokio::main]
async fn main() {
    // Create shared state
    let state = ServerState {
        engines: Arc::new(Mutex::new(HashMap::new())),
    };

    // Build the app with routes
    let app = Router::new().route("/", get(root_handler)).with_state(state);

    // Define the address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Axum server running at http://{addr}");

    // Start the server using Hyper via Axum
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
