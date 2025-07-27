use chess::{ Board, ChessMove, MoveGen };
use std::time::{ Duration, Instant };
use crate::bot::util::board::BoardExt;
use crate::bot::util::lookup::lookup_opening_db;
use crate::bot::{ eval::evaluate_board, include::types::{ EngineState } };

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
    let max_time = time_limit_ms.unwrap_or(time_left_ms / 40).min(time_left_ms);
    let deadline = start_time + Duration::from_millis(max_time as u64);
    let mut final_depth = 0;

    let mut best_move = None;
    let mut best_eval = 0;
    let mut total_nodes = 0;

    for depth in 1..=64 {
        let mut nodes = 0;
        let (mv, eval) = alpha_beta(
            &board,
            i32::MIN + 1,
            i32::MAX - 1,
            board.side_to_move() == chess::Color::White,
            &mut nodes,
            deadline,
            engine_state,
            depth
        );

        if Instant::now() >= deadline {
            break;
        }

        if let Some(m) = mv {
            best_move = Some(m);
            best_eval = eval;
            total_nodes += nodes;
            final_depth = depth;
        } else {
            break; // Timeout hit
        }
    }

    let time_taken = start_time.elapsed().as_millis();
    (best_move, total_nodes, time_taken, best_eval, final_depth)
}

fn get_prioritized_moves(board: &Board) -> Vec<(ChessMove, i32)> {
    let mut move_priority_pairs = Vec::new();

    for mv in MoveGen::new_legal(board) {
        let priority = board.move_priority(mv);
        move_priority_pairs.push((mv, priority));
    }

    // Sort moves by decreasing priority
    move_priority_pairs.sort_by(|a, b| b.1.cmp(&a.1));

    move_priority_pairs
}

fn alpha_beta(
    board: &Board,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
    nodes: &mut u64,
    deadline: Instant,
    engine_state: &mut EngineState,
    depth: u8
) -> (Option<ChessMove>, i32) {
    if Instant::now() >= deadline {
        return (None, 0);
    }

    *nodes += 1;
    // Base Cases: Checkmate / Stalemate / Repetition / Depth == 0
    let board_hash = board.get_hash();
    let repetition_count = engine_state.history.get(&board_hash).copied().unwrap_or(0);

    match board.status() {
        chess::BoardStatus::Checkmate => {
            let eval = evaluate_board(board);
            let score = (eval * ((depth as i32) + 1)).clamp(-100_000, 100_000);
            return (None, score);
        }
        chess::BoardStatus::Stalemate => {
            return (None, 0); // draw
        }
        _ if repetition_count >= 3 => {
            return (None, 0); // threefold repetition draw
        }
        _ if depth == 0 => {
            let eval = evaluate_board(board);
            return (None, eval);
        }
        _ => {}
    }

    let mut best_move = None;
    let prioritized_moves = get_prioritized_moves(board);

    let mut best_eval = if maximizing { i32::MIN } else { i32::MAX };

    for (_, (mv, _)) in prioritized_moves.into_iter().enumerate() {
        let new_board = board.make_move_new(mv);
        let board_hash = new_board.get_hash();

        *engine_state.history.entry(board_hash).or_insert(0) += 1;
        let (_, eval) = alpha_beta(
            &new_board,
            alpha,
            beta,
            !maximizing,
            nodes,
            deadline,
            engine_state,
            depth - 1
        );
        if let Some(count) = engine_state.history.get_mut(&board_hash) {
            *count -= 1;
            if *count == 0 {
                engine_state.history.remove(&board_hash);
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
