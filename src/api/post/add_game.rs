use axum::{ extract::State, response::IntoResponse, Json, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use chess::Board;
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

    (
        StatusCode::CREATED,
        Json(NewGameResponse {
            message: format!("Game '{}' initialized.", payload.game_id),
        }),
    )
}
