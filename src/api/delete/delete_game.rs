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
pub async fn delete_game_handler(
    State(state): State<ServerState>,
    Query(params): Query<DeleteGameQuery>
) -> impl IntoResponse {
    if let Some((_, mut engine)) = state.engines.remove(&params.game_id) {
        if let Some(mut search_handle) = engine.search.take() {
            search_handle.stop();
        }

        (
            StatusCode::OK,
            Json(DeleteGameResponse {
                message: format!(
                    "Game '{}' and its search threads cleaned up successfully",
                    params.game_id
                ),
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
