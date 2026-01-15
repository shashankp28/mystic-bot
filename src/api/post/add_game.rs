use axum::{ extract::State, response::IntoResponse, Json, http::StatusCode };
use lru::LruCache;
use serde::{ Deserialize, Serialize };
use std::{ str::FromStr, sync::{Arc, Mutex} };
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
    // 1. Check for existing game to prevent duplicates
    if state.engines.contains_key(&payload.game_id) {
        return (
            StatusCode::CONFLICT,
            Json(NewGameResponse {
                message: format!("Game ID '{}' already exists", payload.game_id),
            }),
        );
    }

    // 2. Parse the FEN string into a Board
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

    // 3. Build repetition history from the move list/FEN history
    let mut history = RepetitionHistory::new();
    for fen in &payload.history {
        if let Ok(b) = Board::from_str(fen) {
            history.increment(b.get_hash());
        }
    }

    // 4. Initialize the Transposition Table (TT)
    // Note: TT_TABLE_SIZE should be passed to the LruCache constructor
    let tt = TranspositionTable {
        inner: Arc::new(Mutex::new(LruCache::new(
            std::num::NonZeroUsize::new(TT_TABLE_SIZE).unwrap()
        ))),
    };

    // 5. Launch the background search thread
    // We pass clones of Board and TT because the search thread needs its own ownership
    let search_handle = start_background_search(board, tt.clone());

    // 6. Construct the EngineState using our established Blueprint
    let engine = EngineState {
        game_id: payload.game_id.clone(),
        current_board: board, // Aligned with blueprint
        history,              // Aligned with blueprint
        transposition_table: tt,
        search: Some(search_handle),
    };

    // 7. Store the game in the global state
    state.engines.insert(payload.game_id.clone(), engine);

    (
        StatusCode::CREATED,
        Json(NewGameResponse {
            message: format!("Game '{}' initialized. Search thread started.", payload.game_id),
        }),
    )
}