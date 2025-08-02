use chess::{ Board, ChessMove, MoveGen };
use std::time::Instant;
use crate::bot::algorithm::eval::evaluate_board;
use crate::bot::algorithm::root::QUIET_SEARCH_DEPTH;
use crate::bot::util::board::BoardExt;
use crate::bot::util::piece::piece_value;
use crate::bot::{ include::types::{ EngineState } };

pub fn get_prioritized_moves(board: &Board, is_capture: bool) -> Vec<(ChessMove, i32)> {
    let mut move_priority_pairs = Vec::new();

    for mv in MoveGen::new_legal(board) {
        if is_capture {
            if let Some((attacker, victim)) = board.capture_pieces(mv) {
                if piece_value(victim) < piece_value(attacker) {
                    continue;
                }
            } else {
                continue;
            }
        }

        let priority = board.move_priority(mv);
        move_priority_pairs.push((mv, priority));
    }

    // Sort moves by decreasing priority
    move_priority_pairs.sort_by(|a, b| b.1.cmp(&a.1));
    move_priority_pairs
}

pub fn alpha_beta(
    board: &Board,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
    nodes: &mut u64,
    deadline: Instant,
    engine_state: &mut EngineState,
    depth: u8,
    is_noisy: bool,
    current_depth: u8,
    max_depth_reached: &mut u8
) -> (Option<ChessMove>, i32) {
    if Instant::now() >= deadline {
        return (None, 0);
    }

    *nodes += 1;

    // Track maximum depth reached
    *max_depth_reached = (*max_depth_reached).max(current_depth);

    let board_hash = board.get_hash();
    let repetition_count = engine_state.history.get(&board_hash).copied().unwrap_or(0);
    let prioritized_moves = get_prioritized_moves(board, is_noisy);

    match board.status() {
        chess::BoardStatus::Checkmate => {
            let eval = evaluate_board(board);
            let score = (eval * ((depth as i32) + 1)).clamp(-100_000, 100_000);
            return (None, score);
        }
        chess::BoardStatus::Stalemate => {
            return (None, 0);
        }
        _ if repetition_count >= 3 => {
            return (None, 0);
        }
        _ if !is_noisy && depth == 0 => {
            return alpha_beta(
                board,
                alpha,
                beta,
                maximizing,
                nodes,
                deadline,
                engine_state,
                QUIET_SEARCH_DEPTH,
                true,
                current_depth + 1,
                max_depth_reached
            );
        }
        _ if is_noisy && (prioritized_moves.is_empty() || depth == 0) => {
            let eval = evaluate_board(board);
            return (None, eval);
        }
        _ => {}
    }

    let mut best_move = None;
    let mut best_eval = if maximizing { i32::MIN } else { i32::MAX };

    for (mv, _) in prioritized_moves {
        let new_board = board.make_move_new(mv);
        let new_hash = new_board.get_hash();

        *engine_state.history.entry(new_hash).or_insert(0) += 1;

        let (_, eval) = alpha_beta(
            &new_board,
            alpha,
            beta,
            !maximizing,
            nodes,
            deadline,
            engine_state,
            depth.saturating_sub(1),
            is_noisy,
            current_depth + 1,
            max_depth_reached
        );


        if let Some(count) = engine_state.history.get_mut(&new_hash) {
            *count -= 1;
            if *count == 0 {
                engine_state.history.remove(&new_hash);
            }
        }

        if Instant::now() >= deadline {
            return (None, 0);
        }

        if maximizing {
            if eval > best_eval {
                best_eval = eval;
                best_move = Some(mv);
            }
            alpha = alpha.max(eval);
        } else {
            if eval < best_eval {
                best_eval = eval;
                best_move = Some(mv);
            }
            beta = beta.min(eval);
        }

        if beta <= alpha {
            break;
        }
    }

    (best_move, best_eval)
}
