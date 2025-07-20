use axum::{ extract::State, http::StatusCode, response::IntoResponse, Json };
use serde::{ Deserialize, Serialize };
use std::{ time::Instant };
use crate::bot::{ include::types::{ ServerState, Statistics } };
use crate::bot::search::search;

#[derive(Debug, Deserialize)]
pub struct BestMoveQuery {
    game_id: String,
    time_left_ms: u128,
    time_limit_ms: Option<u128>,
    update_state: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BestMoveResponse {
    best_move: Option<String>,
    eval: i32,
    nodes: u64,
    time: u128,
    depth: u8,
}

pub async fn best_move_handler(
    State(state): State<ServerState>,
    Json(params): Json<BestMoveQuery>
) -> impl IntoResponse {
    let Some(mut engine) = state.engines.get_mut(&params.game_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(BestMoveResponse {
                best_move: None,
                eval: 0,
                nodes: 0,
                time: 0,
                depth: 0,
            }),
        );
    };

    let now = Instant::now();

    let (best_move, nodes, time, eval, depth) = search(
        params.time_left_ms,
        params.time_limit_ms,
        &engine.current_board,
        &engine
    );

    let time_taken_ms = now.elapsed().as_millis();

    // Update engine state statistics if requested
    if params.update_state.unwrap_or(false) {
        // Update cumulative statistics under a fixed key (e.g., 0)
        let key = engine.current_board.get_hash();
        engine.statistics.entry(key).or_insert(Statistics {
            nodes_explored: nodes,
            time_taken_ms: time_taken_ms,
        });
        if let Some(best) = best_move {
            // Update the current board with the selected move
            let new_board = engine.current_board.make_move_new(best);
            engine.current_board = new_board;
        }
    }

    (
        StatusCode::OK,
        Json(BestMoveResponse {
            best_move: best_move.map(|m| m.to_string()),
            eval,
            nodes,
            time,
            depth,
        }),
    )
}
