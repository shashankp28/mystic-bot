use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
use serde::{ Deserialize, Serialize };
use std::time::Instant;
use crate::bot::{ algorithm::root::search, include::types::{ ServerState, Statistics } };

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
    pub line: Vec<String>, // Full principal variation
    pub eval: i32,
    pub nodes: u64,
    pub time: u128,
    pub depth: u8,
    pub new_position: String,
}

pub async fn best_move_handler(
    State(state): State<ServerState>,
    Json(params): Json<BestMoveQuery>
) -> impl IntoResponse {
    let Some(mut engine) = state.engines.get_mut(&params.game_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(BestMoveResponse {
                best_move: String::new(),
                line: vec![],
                eval: 0,
                nodes: 0,
                time: 0,
                depth: 0,
                new_position: String::new(),
            }),
        );
    };

    let now = Instant::now();
    let board = engine.current_board.clone();

    // search returns (Vec<ChessMove>, u64, u128, i32, u8)
    let (line, nodes, _, eval, depth) = search(
        params.time_left_ms,
        params.time_limit_ms,
        &board,
        &mut engine
    );
    let time_taken_ms = now.elapsed().as_millis();

    let mut new_position = engine.current_board.to_string();

    let best_move_str = line.first().map_or(String::new(), |m| m.to_string());

    if params.update_state.unwrap_or(false) {
        let key = engine.current_board.get_hash();
        engine.statistics.entry(key).or_insert(Statistics {
            nodes_explored: nodes,
            time_taken_ms,
        });

        if let Some(first_move) = line.first() {
            let new_board = engine.current_board.make_move_new(*first_move);
            engine.current_board = new_board;
            new_position = engine.current_board.to_string();
        }
    }

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
            new_position,
        }),
    )
}
