use std::str::FromStr;
use chess::{ChessMove, MoveGen};
use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
use serde::{ Deserialize, Serialize };
use crate::{api::post::make_move::{MoveRequest, MoveResponse}, bot::{ include::types::ServerState }};

#[derive(Debug, Deserialize)]
pub struct BestMoveQuery {
    pub game_id: String,
    pub time_left_ms: u128,
    pub time_limit_ms: Option<u128>,
    pub update_state: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BestMoveResponse {
    pub best_move: String,
    pub line: Vec<String>,
    pub eval: i32,
    pub nodes: u64,
    pub time: u128,
    pub depth: u8,
    pub new_position: String,
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

    if let Some(mut old_search) = engine.search.take() {
        old_search.stop();
    }

    engine.current_board = engine.current_board.make_move_new(chess_move);
    engine.history.increment(engine.current_board.get_hash());

    engine.search = Some(
        crate::bot::include::types::SearchHandle::start(
            engine.current_board,
            engine.transposition_table.clone(),
            engine.history.clone()
        )
    );

    let new_fen = engine.current_board.to_string();
    drop(engine);

    (
        StatusCode::OK,
        Json(MoveResponse {
            message: format!("Move {} played successfully. Bot is thinking...", payload.mov),
            new_fen,
        }),
    ).into_response()
}
/// Helper for clean error responses
fn create_empty_response() -> BestMoveResponse {
    BestMoveResponse {
        best_move: String::new(),
        line: vec![],
        eval: 0,
        nodes: 0,
        time: 0,
        depth: 0,
        new_position: String::new(),
    }
}
