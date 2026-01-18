use axum::{ extract::State, response::IntoResponse, Json, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use chess::Board;
use tracing::{ info, warn, error, debug, instrument };
use crate::bot::include::types::*;

#[derive(Debug, Deserialize)]
pub struct NewGameRequest {
    pub game_id: String,
    pub current_fen: String,
    pub history: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct NewGameResponse {
    pub message: String,
}

#[instrument(skip(state, payload), fields(game_id = %payload.game_id))]
pub async fn new_game_handler(
    State(state): State<ServerState>,
    Json(payload): Json<NewGameRequest>
) -> impl IntoResponse {
    if state.engines.contains_key(&payload.game_id) {
        warn!(game_id = %payload.game_id, "Attempted to create a game that already exists");
        return (
            StatusCode::CONFLICT,
            Json(NewGameResponse {
                message: format!("Game ID '{}' already exists", payload.game_id),
            }),
        );
    }

    let board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(e) => {
            error!(error = ?e, fen = %payload.current_fen, "Failed to parse initial FEN");
            return (
                StatusCode::BAD_REQUEST,
                Json(NewGameResponse { message: "Invalid FEN".to_string() }),
            );
        }
    };

    let mut history = RepetitionHistory::new();
    for fen in &payload.history {
        if let Ok(b) = Board::from_str(fen) {
            history.increment(b.get_hash());
        }
    }
    debug!(history_count = payload.history.len(), "Repetition history initialized");

    let tt = TranspositionTable::new(TT_TABLE_SIZE);

    let search_handle = SearchHandle::start(board, tt.clone(), history.clone());

    let engine = EngineState {
        game_id: payload.game_id.clone(),
        current_board: board,
        history,
        transposition_table: tt,
        search: Some(search_handle),
    };

    state.engines.insert(payload.game_id.clone(), engine);

    info!(
        game_id = %payload.game_id, 
        fen = %payload.current_fen, 
        "New engine state successfully initialized and cached"
    );

    (
        StatusCode::CREATED,
        Json(NewGameResponse {
            message: format!("Game '{}' initialized.", payload.game_id),
        }),
    )
}
