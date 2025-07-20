use axum::{ extract::{ Query, State }, response::IntoResponse, Json, http::StatusCode };
use serde::Deserialize;
use crate::bot::include::types::{ ServerState };

#[derive(Debug, Deserialize)]
pub struct DeleteGameQuery {
    pub game_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct DeleteGameResponse {
    pub message: String,
}

/// DELETE /delete — Removes an EngineState for a given game_id
pub async fn delete_game_handler(
    State(state): State<ServerState>,
    Query(params): Query<DeleteGameQuery>
) -> impl IntoResponse {
    if state.engines.remove(&params.game_id).is_some() {
        (
            StatusCode::OK,
            Json(DeleteGameResponse {
                message: format!("Game '{}' deleted successfully", params.game_id),
            }),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(DeleteGameResponse {
                message: format!("Game '{}' not found", params.game_id),
            }),
        )
    }
}
