use axum::{ extract::State, Json, http::StatusCode, response::IntoResponse };
use crate::bot::include::types::{ ServerState, SearchHandle };
use chess::{ ChessMove, MoveGen };
use std::str::FromStr;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub game_id: String,
    pub mov: String,
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
                    message: "Game not found".to_string(),
                    new_fen: "".to_string(),
                }),
            ).into_response();
        }
    };

    let Ok(chess_move) = ChessMove::from_str(&payload.mov) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(MoveResponse {
                message: "Invalid move format".to_string(),
                new_fen: engine.current_board.to_string(),
            }),
        ).into_response();
    };

    if !MoveGen::new_legal(&engine.current_board).any(|m| m == chess_move) {
        return (
            StatusCode::BAD_REQUEST,
            Json(MoveResponse {
                message: "Illegal move".to_string(),
                new_fen: engine.current_board.to_string(),
            }),
        ).into_response();
    }

    if let Some(mut old_search) = engine.search.take() {
        old_search.stop();
    }

    engine.current_board = engine.current_board.make_move_new(chess_move);
    let board_hash = engine.current_board.get_hash();
    engine.history.increment(board_hash);

    engine.search = Some(
        SearchHandle::start(
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
            message: format!("Move {} applied.", payload.mov),
            new_fen,
        }),
    ).into_response()
}
