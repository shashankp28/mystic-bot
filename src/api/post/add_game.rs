use axum::{ extract::State, response::IntoResponse, Json, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::{ collections::HashMap, str::FromStr };
use crate::bot::types::{ EngineState, ServerState };
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

/// POST /new — Creates a new EngineState for a game
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

    let current_board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(NewGameResponse {
                    message: "Invalid FEN for current_board".to_string(),
                }),
            );
        }
    };

    let mut history: HashMap<u64, u32> = HashMap::new();
    for fen in &payload.history {
        if let Ok(board) = Board::from_str(fen) {
            let hash = board.get_hash();
            *history.entry(hash).or_insert(0) += 1;
        }
    }

    let engine = EngineState {
        game_id: payload.game_id.clone(),
        current_board,
        history,
        statistics: HashMap::new(),
    };

    state.engines.insert(payload.game_id.clone(), engine);

    (
        StatusCode::CREATED,
        Json(NewGameResponse {
            message: format!("Game '{}' created", payload.game_id),
        }),
    )
}
