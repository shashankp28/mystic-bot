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
    let current_board = match Board::from_str(&payload.current_fen) {
        Ok(board) => board,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(BestMoveResponse {
                    best_move: String::new(),
                    line: vec![],
                    eval: 0,
                    nodes: 0,
                    time: 0,
                    depth: 0,
                }),
            );
        }
    };

    // Reconstruct repetition history
    let mut history = RepetitionHistory::new();
    for fen in &payload.history {
        if let Ok(past_board) = Board::from_str(fen) {
            let hash = past_board.get_hash();
            history.increment(hash);
        }
    }

    let transposition_table = TranspositionTable::new(TT_TABLE_SIZE);

    let mut engine = EngineState {
        game_id: "eval_temp".to_string(),
        current_board,
        history,
        statistics: Default::default(),
        global_map: Arc::clone(&state.global_map),
        transposition_table,
    };

    let start_time = Instant::now();
    let board = engine.current_board.clone();
    let (line, nodes, _, eval, depth) = search(
        payload.time_left_ms,
        payload.time_limit_ms,
        &board,
        &mut engine
    );
    let time_taken_ms = start_time.elapsed().as_millis();

    let best_move_str = line.first().map_or(String::new(), |m| m.to_string());

    (
        StatusCode::OK,
        Json(BestMoveResponse {
            best_move: best_move_str,
            line: line
                .iter()
                .map(|m| m.to_string())
                .collect(),
            eval,
            nodes,
            time: time_taken_ms,
            depth,
        }),
    )
}
