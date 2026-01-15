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
    // 1. Get the game engine
    let Some(mut engine) = state.engines.get_mut(&payload.game_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(MoveResponse {
                message: format!("Game ID '{}' not found", payload.game_id),
                new_fen: "".to_string(),
            }),
        );
    };

    // 2. Parse the move
    let Ok(chess_move) = ChessMove::from_str(&payload.mov) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(MoveResponse {
                message: "Invalid move format".to_string(),
                new_fen: engine.current_board.to_string(),
            }),
        );
    };

    // 3. Check Legality
    let mut legal_moves = MoveGen::new_legal(&engine.current_board);
    if !legal_moves.any(|m| m == chess_move) {
        return (
            StatusCode::BAD_REQUEST,
            Json(MoveResponse {
                message: "Illegal move".to_string(),
                new_fen: engine.current_board.to_string(),
            }),
        );
    }

    // 4. 🔥 CLEANUP: Stop and Join the obsolete search
    // If the human moves, the bot's current search is now for the wrong position.
    if let Some(old_search) = engine.search.take() {
        old_search.stop.store(true, std::sync::atomic::Ordering::SeqCst);
        let _ = old_search.handle.join();
    }

    // 5. Update the board and repetition history
    engine.current_board = engine.current_board.make_move_new(chess_move);
    engine.history.increment(engine.current_board.get_hash());

    // 6. 🔥 RESTART: Start searching for the bot's response immediately
    engine.search = Some(
        start_background_search(engine.current_board, engine.transposition_table.clone())
    );

    let new_fen = engine.current_board.to_string();

    (
        StatusCode::OK,
        Json(MoveResponse {
            message: format!("Move {} played successfully. Bot is thinking...", payload.mov),
            new_fen,
        }),
    )
}
