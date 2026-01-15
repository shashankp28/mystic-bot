use axum::{ extract::{ State, Json }, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::{ str::FromStr, sync::Arc, time::Instant };
use chess::Board;
use crate::bot::{
    algorithm::root::search,
    include::types::{
        EngineState,
        RepetitionHistory,
        ServerState,
        TranspositionTable,
        TT_TABLE_SIZE,
    },
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
pub async fn eval_position_handler(
    State(state): State<ServerState>,
    Json(payload): Json<EvalRequest>
) -> impl IntoResponse {
    // 1. Setup the Board
    let board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(create_empty_response())).into_response();
        }
    };

    // 2. Setup the Transposition Table
    let tt = TranspositionTable {
        inner: Arc::new(
            std::sync::Mutex::new(
                lru::LruCache::new(std::num::NonZeroUsize::new(TT_TABLE_SIZE).unwrap())
            )
        ),
    };

    // 3. START the background engine search immediately
    let search_handle = start_background_search(board, tt.clone());
    let start_instant = Instant::now();

    // 4. WAIT for exactly 5 seconds (without blocking the server)
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // 5. STOP the search and JOIN the thread
    search_handle.stop.store(true, std::sync::atomic::Ordering::SeqCst);

    // We move the handle out to join it
    let _ = search_handle.handle.join();

    // 6. EXTRACT the final results from the Mutex
    let final_snapshot = search_handle.best.lock().unwrap().clone();

    // 7. Cleanup is automatic here: search_handle and tt go out of scope and are dropped.

    match final_snapshot {
        Some(result) => {
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
            ).into_response()
        }
        None => {
            // This happens if the search didn't even finish Depth 1 in 5 seconds (unlikely)
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
