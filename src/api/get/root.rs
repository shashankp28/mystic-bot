use axum::{ extract::State, response::IntoResponse, Json };
use serde::Serialize;
use crate::bot::types::ServerState;

// Response type for "/"
#[derive(Serialize)]
pub struct RootResponse {
    message: String,
    games: Vec<String>,
}

// Handler for GET /

pub async fn root_handler(State(state): State<ServerState>) -> impl IntoResponse {
    let games = state.engines
        .iter()
        .map(|entry| entry.key().clone())
        .collect::<Vec<String>>();

    let response = RootResponse {
        message: "Welcome to MysticBot".to_string(),
        games,
    };

    Json(response)
}
