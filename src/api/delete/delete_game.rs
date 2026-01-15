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
    // 1. Remove the engine from the map to take ownership
    // We use remove() to get the EngineState out so we can access the handle
    if let Some((_, mut engine)) = state.engines.remove(&params.game_id) {
        // 2. Properly shut down the search thread if it exists
        if let Some(search_handle) = engine.search.take() {
            // Signal the atomic stop flag
            search_handle.stop.store(true, std::sync::atomic::Ordering::SeqCst);

            // Wait for the thread to actually exit to reclaim CPU/RAM
            let _ = search_handle.handle.join();
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
