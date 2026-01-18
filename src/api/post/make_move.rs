use axum::{ extract::State, Json, http::StatusCode, response::IntoResponse };
use crate::bot::include::types::{ ServerState };
use chess::{ ChessMove, MoveGen };
use std::str::FromStr;

use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub game_id: String,
    pub mov: String, // "move" is a reserved keyword
}

#[derive(Debug, Serialize)]
pub struct MoveResponse {
    pub message: String,
    pub new_fen: String,
}

pub async fn make_move_handler(
    State(state): State<ServerState>,
    Json(payload): Json<MoveRequest>
) -> impl IntoResponse {
    let mut engine = match state.engines.get_mut(&payload.game_id) {
        Some(e) => e,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(MoveResponse {
                    message: format!("Game ID '{}' not found", payload.game_id),
                    new_fen: "".to_string(),
                }),
            ).into_response();
        }
    };

    let Ok(chess_move) = ChessMove::from_str(&payload.mov) else {
        let fen = engine.current_board.to_string();
        return (
            StatusCode::BAD_REQUEST,
            Json(MoveResponse {
                message: "Invalid move format".to_string(),
                new_fen: fen,
            }),
        ).into_response();
    };

    let mut legal_moves = MoveGen::new_legal(&engine.current_board);
    if !legal_moves.any(|m| m == chess_move) {
        let fen = engine.current_board.to_string();
        return (
            StatusCode::BAD_REQUEST,
            Json(MoveResponse {
                message: "Illegal move".to_string(),
                new_fen: fen,
            }),
        ).into_response();
    }

    // Stop current search as the position is changing
    if let Some(mut old_search) = engine.search.take() {
        old_search.stop();
    }

    // Apply the move and update history
    engine.current_board = engine.current_board.make_move_new(chess_move);
    engine.history.increment(engine.current_board.get_hash());

    // Start a new search with the updated board and history
    engine.search = Some(
        crate::bot::include::types::SearchHandle::start(
            engine.current_board,
            engine.transposition_table.clone(),
            engine.history.clone() // Added required history parameter
        )
    );

    let new_fen = engine.current_board.to_string();

    // Explicitly release the lock on the engine entry
    drop(engine);

    (
        StatusCode::OK,
        Json(MoveResponse {
            message: format!("Move {} played successfully. Bot is thinking...", payload.mov),
            new_fen,
        }),
    ).into_response()
}
