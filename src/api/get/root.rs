use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;
use tracing::{info, debug, instrument};
use crate::bot::include::types::ServerState;

#[derive(Serialize)]
pub struct RootResponse {
    message: String,
    games: Vec<String>,
}

#[instrument(skip(state))]
pub async fn root_handler(State(state): State<ServerState>) -> impl IntoResponse {
    debug!("Accessing global engine state map");

    let games = state.engines
        .iter()
        .map(|entry| entry.key().clone())
        .collect::<Vec<String>>();

    info!(
        active_games_count = games.len(),
        "Root handler accessed successfully"
    );

    let response = RootResponse {
        message: "Welcome to MysticBot".to_string(),
        games,
    };

    Json(response)
}