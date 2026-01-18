use axum::{ extract::Json, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::str::FromStr;
use chess::Board;
use tracing::{ info, warn, debug, instrument };

use crate::bot::algorithm::eval::evaluate_board;

#[derive(Debug, Deserialize)]
pub struct EvalRequest {
    pub current_fen: String,
}

#[derive(Debug, Serialize)]
pub struct StaticEvalResponse {
    pub eval: i32,
}

#[instrument(skip(payload))]
pub async fn static_eval_handler(Json(payload): Json<EvalRequest>) -> impl IntoResponse {
    let current_board = match Board::from_str(&payload.current_fen) {
        Ok(board) => board,
        Err(e) => {
            warn!(
                fen = %payload.current_fen,
                error = ?e,
                "Failed to parse FEN for static evaluation"
            );
            return (StatusCode::BAD_REQUEST, Json(StaticEvalResponse { eval: 0 }));
        }
    };

    debug!(fen = %payload.current_fen, "Board parsed successfully");

    let eval = evaluate_board(&current_board);

    info!(
        fen = %payload.current_fen,
        score = eval,
        "Static evaluation completed"
    );

    (StatusCode::OK, Json(StaticEvalResponse { eval }))
}
