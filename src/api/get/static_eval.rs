use axum::{ extract::Json, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use chess::Board;

use crate::bot::algorithm::eval::evaluate_board;

#[derive(Debug, Deserialize)]
pub struct EvalRequest {
    pub current_fen: String,
}

#[derive(Debug, Serialize)]
pub struct StaticEvalResponse {
    pub eval: i32,
}

pub async fn static_eval_handler(Json(payload): Json<EvalRequest>) -> impl IntoResponse {
    let current_board = match Board::from_str(&payload.current_fen) {
        Ok(board) => board,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(StaticEvalResponse {
                    eval: 0,
                }),
            );
        }
    };

    let eval = evaluate_board(&current_board);

    (
        StatusCode::OK,
        Json(StaticEvalResponse {
            eval,
        }),
    )
}
