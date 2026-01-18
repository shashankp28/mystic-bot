use axum::{ extract::State, response::IntoResponse, Json, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::{ str::FromStr, sync::{ Arc, Mutex } };
use crate::bot::include::types::{
    EngineState,
    RepetitionHistory,
    ServerState,
    TranspositionTable,
    TT_TABLE_SIZE,
};
use chess::Board;

#[derive(Debug, Deserialize)]
pub struct NewGameRequest {
    pub game_id: String,
    pub current_fen: String,
    pub history: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct NewGameResponse {
    message: String,
}

pub async fn new_game_handler(
    State(state): State<ServerState>,
    Json(payload): Json<NewGameRequest>
) -> impl IntoResponse {
    if state.engines.contains_key(&payload.game_id) {
        return (
            StatusCode::CONFLICT,
            Json(NewGameResponse {
                message: format!("Game ID '{}' already exists", payload.game_id),
            }),
        );
    }

    let board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(NewGameResponse {
                    message: "Invalid FEN".to_string(),
                }),
            );
        }
    };

    // 1. Properly rebuild the repetition history from the provided FEN list
    let mut history = RepetitionHistory::new();
    for fen in &payload.history {
        if let Ok(b) = Board::from_str(fen) {
            history.increment(b.get_hash());
        }
    }

    // 2. Initialize the Transposition Table
    let tt = TranspositionTable {
        inner: Arc::new(
            Mutex::new(lru::LruCache::new(std::num::NonZeroUsize::new(TT_TABLE_SIZE).unwrap()))
        ),
    };

    // 3. Start the background search thread
    // Note: history is moved into start(), and tt is cloned (Arc-based)
    let search_handle = crate::bot::include::types::SearchHandle::start(
        board,
        tt.clone(),
        history.clone()
    );

    // 4. Build the engine state
    let engine = EngineState {
        game_id: payload.game_id.clone(),
        current_board: board,
        history,
        transposition_table: tt,
        search: Some(search_handle),
    };

    state.engines.insert(payload.game_id.clone(), engine);

    (
        StatusCode::CREATED,
        Json(NewGameResponse {
            message: format!("Game '{}' initialized. Search thread started.", payload.game_id),
        }),
    )
}
