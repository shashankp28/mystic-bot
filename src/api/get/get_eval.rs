use axum::{ extract::Json, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::{ str::FromStr, sync::Arc, time::Instant };
use chess::Board;
use crate::bot::{
    include::types::{ SearchHandle, TT_TABLE_SIZE, TranspositionTable, RepetitionHistory },
    util::search::estimate_search_time,
};

#[derive(Debug, Deserialize)]
pub struct EvalRequest {
    pub current_fen: String,
    pub history: Vec<String>,
    pub time_left_ms: u128,
    pub time_limit_ms: Option<u128>,
}

#[derive(Debug, Serialize)]
pub struct BestMoveResponse {
    pub best_move: String,
    pub line: Vec<String>, // Full principal variation in UCI format
    pub eval: i32, // Evaluation score
    pub nodes: u64, // Total nodes searched
    pub time: u128, // Time taken in milliseconds
    pub depth: u8, // Maximum search depth reached
}

pub async fn eval_position_handler(Json(payload): Json<EvalRequest>) -> impl IntoResponse {
    // Parse board
    let board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(create_empty_response())).into_response();
        }
    };

    // Rebuild repetition history
    let mut history = RepetitionHistory::new();
    for fen in &payload.history {
        if let Ok(b) = Board::from_str(fen) {
            history.increment(b.get_hash());
        }
    }

    let tt = TranspositionTable::new(TT_TABLE_SIZE);
    let mut search_handle = SearchHandle::start(board, tt, history);

    // Estimate time and prepare timer
    let estimated_ms = estimate_search_time(payload.time_left_ms, payload.time_limit_ms);
    let time_limit = std::time::Duration::from_millis(estimated_ms as u64);
    let stop_signal: Arc<std::sync::atomic::AtomicBool> = Arc::clone(&search_handle.stop);
    let start_instant = Instant::now();

    // Wait until either search finishes or timer expires
    tokio::select! {
        _ = tokio::time::sleep(time_limit) => {
            // Timer expired
        }
        _ = async {
            loop {
                if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        } => {
            // Search finished early (e.g., found mate)
        }
    }

    // Stop the search thread
    search_handle.stop();

    // Grab the final search result
    let final_snapshot = search_handle.best.lock().unwrap().clone();

    match final_snapshot {
        Some(result) =>
            (
                StatusCode::OK,
                Json(BestMoveResponse {
                    best_move: result.best_move.to_string(),
                    line: result.pv
                        .iter()
                        .map(|m| m.to_string())
                        .collect(),
                    eval: result.eval,
                    nodes: result.nodes,
                    time: start_instant.elapsed().as_millis(),
                    depth: result.depth,
                }),
            ).into_response(),
        None => (StatusCode::INTERNAL_SERVER_ERROR, Json(create_empty_response())).into_response(),
    }
}

fn create_empty_response() -> BestMoveResponse {
    BestMoveResponse {
        best_move: String::new(),
        line: vec![],
        eval: 0,
        nodes: 0,
        time: 0,
        depth: 0,
    }
}
