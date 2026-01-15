use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
use serde::{ Deserialize, Serialize };
use crate::bot::{ include::types::ServerState };

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
    // 1. Get the game engine (mutable access needed to update board/search)
    let Some(mut engine) = state.engines.get_mut(&params.game_id) else {
        return (StatusCode::NOT_FOUND, Json(create_empty_response())).into_response();
    };

    // 2. Ensure a search is actually running
    let Some(search_handle) = engine.search.as_ref() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(create_empty_response())).into_response();
    };

    // 3. Safely lock and clone the latest best result found by the thread
    let snapshot_opt = search_handle.best.lock().unwrap().clone();

    let Some(snapshot) = snapshot_opt else {
        // Search thread exists but hasn't completed Depth 1 yet
        return (StatusCode::ACCEPTED, Json(create_empty_response())).into_response();
    };

    let mut new_position_fen = engine.current_board.to_string();
    let best_move = snapshot.best_move;

    // 4. Update state if requested (Advance the game)
    if params.update_state.unwrap_or(false) {
        // 1. Take the handle out of the engine (leaves None in its place)
        if let Some(old_search) = engine.search.take() {
            // 2. Signal stop
            old_search.stop.store(true, std::sync::atomic::Ordering::SeqCst);

            // 3. Wait for the thread to finish
            let _ = old_search.handle.join();
        }

        // 4. Update the board and history
        engine.current_board = engine.current_board.make_move_new(best_move);
        engine.history.increment(engine.current_board.get_hash());
        new_position_fen = engine.current_board.to_string();

        // 5. Start a fresh search
        engine.search = Some(
            start_background_search(engine.current_board, engine.transposition_table.clone())
        );
    }

    // 5. Build the successful response
    (
        StatusCode::OK,
        Json(BestMoveResponse {
            best_move: best_move.to_string(),
            line: snapshot.pv
                .iter()
                .map(|m| m.to_string())
                .collect(),
            eval: snapshot.eval,
            nodes: snapshot.nodes,
            time: 0, // You can track start_time in SearchHandle if needed
            depth: snapshot.depth,
            new_position: new_position_fen,
        }),
    ).into_response()
}

/// Helper for clean error responses
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
