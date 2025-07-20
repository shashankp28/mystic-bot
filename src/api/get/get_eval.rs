use axum::{ extract::{ State, Json }, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize, Serialize };
use std::{ str::FromStr, collections::HashMap, sync::Arc };
use chess::Board;
use crate::bot::include::types::{ ServerState, EngineState };
use crate::bot::search::search;

#[derive(Debug, Deserialize)]
pub struct EvalRequest {
    pub current_fen: String,
    pub history: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BestMoveResponse {
    pub best_move: Option<String>,
    pub eval: i32,
    pub nodes: u64,
    pub time: u128,
    pub depth: u8,
}

pub async fn eval_position_handler(
    State(state): State<ServerState>,
    Json(payload): Json<EvalRequest>
) -> impl IntoResponse {
    let current_board = match Board::from_str(&payload.current_fen) {
        Ok(b) => b,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(BestMoveResponse {
                    best_move: None,
                    eval: 0,
                    nodes: 0,
                    time: 0,
                    depth: 0,
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
        game_id: "eval_temp".to_string(),
        current_board,
        history,
        statistics: HashMap::new(),
        global_map: Arc::clone(&state.global_map),
    };

    // Default time values — can tweak or make them params if needed
    let (best_move, nodes, time_taken_ms, eval, depth) = search(
        10000, // time_left_ms
        Some(300), // time_limit_ms (shorter for stateless call)
        &engine.current_board,
        &engine
    );

    (
        StatusCode::OK,
        Json(BestMoveResponse {
            best_move: best_move.map(|m| m.to_string()),
            eval,
            nodes,
            time: time_taken_ms,
            depth,
        }),
    )
}
