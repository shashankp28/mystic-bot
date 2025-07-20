use axum::{
    http::Response,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use mystic_bot::{
    api::{get::root::root_handler, post::add_game::new_game_handler},
    bot::types::ServerState,
};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnRequest};
use tracing::Span;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Set up tracing subscriber for logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    // Create shared state
    let state = ServerState {
        engines: Arc::new(DashMap::new()),
    };

    // Create trace layer with logging
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(log_response);

    // Build the app with routes and middleware
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/game", post(new_game_handler))
        .layer(trace_layer)
        .with_state(state);

    // Define the address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Axum server running at http://{addr}");

    // Start the server
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// Function used to log response metadata
fn log_response<B>(response: &Response<B>, latency: Duration, span: &Span)
where
    B: std::fmt::Debug,
{
    let status = response.status();
    tracing::info!(parent: span, ?status, ?latency, "response sent");
}
