use chess::{ Board, ChessMove, MoveGen };
use std::time::{ Duration, Instant };
use crate::bot::algorithm::negamax::negamax; // assumes renamed or placed negamax in the module
use crate::bot::algorithm::eval::evaluate_board;
use crate::bot::include::types::SpecialMove;
use crate::bot::util::lookup::lookup_opening_db;
use crate::bot::include::types::EngineState;
use crate::bot::util::board::BoardExt;
use crate::bot::util::piece::piece_value;

pub fn get_prioritized_moves(board: &Board, only_noise: bool) -> Vec<(ChessMove, i32)> {
    let mut move_priority_pairs = Vec::new();

    for mv in MoveGen::new_legal(board) {
        if only_noise {
            let move_tags = board.classify_move(mv);
            if
                !move_tags.contains(&SpecialMove::Promotion) &&
                !move_tags.contains(&SpecialMove::Capture)
            {
                continue;
            }
            if let Some((attacker, victim)) = board.capture_pieces(mv) {
                if piece_value(victim) < piece_value(attacker) {
                    continue;
                }
            }
        }

        let priority = board.move_priority(mv);
        move_priority_pairs.push((mv, priority));
    }

    move_priority_pairs.sort_by(|a, b| b.1.cmp(&a.1));
    move_priority_pairs
}

pub fn search(
    time_left_ms: u128,
    time_limit_ms: Option<u128>,
    board: &Board,
    engine_state: &mut EngineState
) -> (Option<ChessMove>, u64, u128, i32, u8) {
    // Opening DB fallback
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

    let color = if board.side_to_move() == chess::Color::White { 1 } else { -1 };

    for depth in 1..=64 {
        let mut nodes = 0;
        let mut max_depth = 0;

        let (mv, eval) = negamax(
            board,
            i32::MIN + 1,
            i32::MAX - 1,
            &mut nodes,
            deadline,
            engine_state,
            depth,
            0,
            &mut max_depth,
            color
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
            break;
        }
    }

    let time_taken = start_time.elapsed().as_millis();
    (best_move, total_nodes, time_taken, best_eval, final_depth)
}
