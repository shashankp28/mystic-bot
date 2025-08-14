use chess::{ Board, ChessMove, MoveGen };
use std::ops::Index;
use std::time::{ Duration, Instant };
use crate::bot::algorithm::negamax::negamax;
use crate::bot::algorithm::eval::evaluate_board;
use crate::bot::include::types::{ SpecialMove, MAX_THINK_TIME_MS };
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
                !move_tags.contains(&SpecialMove::Capture) &&
                !move_tags.contains(&SpecialMove::CastleKingside) &&
                !move_tags.contains(&SpecialMove::CastleQueenside)
            {
                continue;
            }
            if let Some((attacker, victim)) = board.capture_pieces(mv) {
                if piece_value(victim) <= piece_value(attacker) {
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
) -> (Vec<ChessMove>, u64, u128, i32, u8) {
    let start_time = Instant::now();

    if let Some(chess_move) = lookup_opening_db(board) {
        return (
            vec![chess_move],
            0,
            start_time.elapsed().as_millis(),
            evaluate_board(&board.make_move_new(chess_move)),
            0,
        );
    }

    let max_time = time_limit_ms
        .unwrap_or(time_left_ms / 40)
        .min(time_left_ms)
        .min(MAX_THINK_TIME_MS);
    let deadline = start_time + Duration::from_millis(max_time as u64);

    let color = if board.side_to_move() == chess::Color::White { 1 } else { -1 };
    let mut total_nodes = 0;
    let mut depth_reached = 0;

    let mut best_line = Vec::new();
    let mut best_eval = 0;

    for depth in 1..=64 {
        let mut nodes: u64 = 0;
        let mut max_depth = 0;

        let (line, mut eval) = negamax(
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
        eval *= color;
        total_nodes += nodes;

        if Instant::now() >= deadline {
            break;
        }

        if !line.is_empty() {
            best_line = line;
            best_eval = eval;
            depth_reached = max_depth;
        } else {
            break;
        }
        println!("Move: {}, Eval: {}, Depth: {}", best_line.index(0), best_eval, depth_reached);
    }

    let time_taken = start_time.elapsed().as_millis();
    (best_line, total_nodes, time_taken, best_eval, depth_reached)
}
