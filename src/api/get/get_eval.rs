use axum::{ extract::Json, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::{ str::FromStr, sync::Arc, time::Instant };
use chess::Board;
use tracing::{ info, warn, debug, error, instrument };
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
    pub line: Vec<String>,
    pub eval: i32,
    pub nodes: u64,
    pub time: u128,
    pub depth: u8,
}

#[instrument(skip(payload))]
pub async fn eval_position_handler(Json(payload): Json<EvalRequest>) -> impl IntoResponse {
    let board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(e) => {
            warn!(fen = %payload.current_fen, error = ?e, "Invalid FEN received");
            return (StatusCode::BAD_REQUEST, Json(create_empty_response())).into_response();
        }
    };

    let mut history = RepetitionHistory::new();
    for (i, fen) in payload.history.iter().enumerate() {
        if let Ok(b) = Board::from_str(fen) {
            history.increment(b.get_hash());
        } else {
            debug!(index = i, fen = %fen, "Skipping invalid history FEN");
        }
    }

    let tt = TranspositionTable::new(TT_TABLE_SIZE);
    let mut search_handle = SearchHandle::start(board, tt, history);

    let estimated_ms = estimate_search_time(payload.time_left_ms, payload.time_limit_ms);
    let time_limit = std::time::Duration::from_millis(estimated_ms as u64);
    let stop_signal = Arc::clone(&search_handle.stop);
    let start_instant = Instant::now();

    info!(limit_ms = estimated_ms, "Search initiated");

    tokio::select! {
        _ = tokio::time::sleep(time_limit) => {
            debug!("Search time limit reached");
        }
        _ = async {
            loop {
                if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        } => {
            debug!("Search completed before time limit");
        }
    }

    search_handle.stop();
    let final_snapshot = search_handle.best.lock().unwrap().clone();

    match final_snapshot {
        Some(result) => {
            let elapsed = start_instant.elapsed().as_millis();
            info!(
                best_move = %result.best_move,
                eval = result.eval,
                depth = result.depth,
                nodes = result.nodes,
                elapsed_ms = elapsed,
                "Search result found"
            );

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
                    time: elapsed,
                    depth: result.depth,
                }),
            ).into_response()
        }
        None => {
            error!("Search handle returned no result");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(create_empty_response())).into_response()
        }
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
