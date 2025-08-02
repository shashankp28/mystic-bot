use chess::{ Board, ChessMove };
use std::time::{ Duration, Instant };
use crate::bot::algorithm::ab::alpha_beta;
use crate::bot::algorithm::eval::evaluate_board;
use crate::bot::util::lookup::lookup_opening_db;
use crate::bot::{ include::types::{ EngineState } };

pub const QUIET_SEARCH_DEPTH: u8 = 4;

pub fn search(
    time_left_ms: u128,
    time_limit_ms: Option<u128>,
    board: &Board,
    engine_state: &mut EngineState
) -> (Option<ChessMove>, u64, u128, i32, u8) {
    // Try Opening DB first
    let start_time = Instant::now();
    if let Some(chess_move) = lookup_opening_db(board) {
        return (
            Some(chess_move),
            0,
            start_time.elapsed().as_millis(),
            evaluate_board(&board.make_move_new(chess_move)),
            0,
        );
    }

    let start_time = Instant::now();
    let max_time = time_limit_ms
        .unwrap_or(time_left_ms / 40)
        .min(time_left_ms)
        .min(40_000);
    let deadline = start_time + Duration::from_millis(max_time as u64);
    let mut final_depth = 0;

    let mut best_move = None;
    let mut best_eval = 0;
    let mut total_nodes = 0;

    for depth in 1..=64 {
        let mut nodes = 0;
        let mut max_depth = 0;
        let (mv, eval) = alpha_beta(
            &board,
            i32::MIN + 1,
            i32::MAX - 1,
            board.side_to_move() == chess::Color::White,
            &mut nodes,
            deadline,
            engine_state,
            depth,
            false,
            1,
            &mut max_depth
        );

        if Instant::now() >= deadline {
            break;
        }

        if let Some(m) = mv {
            best_move = Some(m);
            best_eval = eval;
            total_nodes += nodes;
            final_depth = max_depth;
        } else {
            break; // Timeout hit
        }
    }

    let time_taken = start_time.elapsed().as_millis();
    (best_move, total_nodes, time_taken, best_eval, final_depth)
}
