use std::{ sync::atomic::Ordering, time::{ Duration, Instant } };
use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
use serde::{ Deserialize, Serialize };
use tracing::{ info, warn, debug, error, instrument };
use crate::bot::include::types::{ SearchHandle, ServerState };

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
    pub line: Vec<String>,
    pub eval: i32,
    pub nodes: u64,
    pub time: u128,
    pub depth: u8,
    pub new_position: String,
}

#[instrument(skip(state, payload), fields(game_id = %payload.game_id))]
pub async fn best_move_handler(
    State(state): State<ServerState>,
    Json(payload): Json<BestMoveQuery>
) -> impl IntoResponse {
    let mut engine = match state.engines.get_mut(&payload.game_id) {
        Some(e) => e,
        None => {
            warn!(game_id = %payload.game_id, "Best move requested for non-existent game");
            return (StatusCode::NOT_FOUND, Json(create_empty_response())).into_response();
        }
    };

    if engine.search.is_none() {
        debug!("Initializing new search handle");
        engine.search = Some(
            SearchHandle::start(
                engine.current_board.clone(),
                engine.transposition_table.clone(),
                engine.history.clone()
            )
        );
    }

    let wait_ms = payload.time_limit_ms.unwrap_or(2000);
    let start_wait = Instant::now();

    while start_wait.elapsed().as_millis() < (wait_ms as u128) {
        let is_done = engine.search
            .as_ref()
            .map(|s| s.stop.load(Ordering::SeqCst))
            .unwrap_or(true);

        if is_done {
            break;
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    if let Some(mut handle) = engine.search.take() {
        let result = handle.best.lock().unwrap().clone();
        
        if let Some(res) = result {
            if payload.update_state.unwrap_or(false) {
                handle.stop();
                let mut next_board = engine.current_board.clone();
                engine.current_board.make_move(res.best_move, &mut next_board);
                engine.current_board = next_board;
                let board_hash = engine.current_board.get_hash();
                engine.history.increment(board_hash);

                engine.search = Some(
                    SearchHandle::start(
                        engine.current_board.clone(),
                        engine.transposition_table.clone(),
                        engine.history.clone()
                    )
                );
                info!("State updated and new search initiated");
            }

            let elapsed = start_wait.elapsed().as_millis();
            debug!(nodes = res.nodes, depth = res.depth, "Returning search results");

            return (
                StatusCode::OK,
                Json(BestMoveResponse {
                    best_move: res.best_move.to_string(),
                    line: res.pv
                        .iter()
                        .map(|m| m.to_string())
                        .collect(),
                    eval: res.eval,
                    nodes: res.nodes,
                    time: elapsed,
                    depth: res.depth,
                    new_position: engine.current_board.to_string(),
                }),
            ).into_response();
        }
    }

    error!("Search failed to produce a valid result within time limits");
    (StatusCode::INTERNAL_SERVER_ERROR, Json(create_empty_response())).into_response()
}

fn create_empty_response() -> BestMoveResponse {
    BestMoveResponse {
        best_move: String::new(),
        line: vec![],
        eval: 0,
        nodes: 0,
        time: 0,
        depth: 0,
        new_position: String::new(),
    }
}
