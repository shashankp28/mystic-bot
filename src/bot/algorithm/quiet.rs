use chess::{ Board, ChessMove };
use std::time::Instant;
use crate::bot::algorithm::eval::{ evaluate_board, is_terminal };
use crate::bot::algorithm::root::get_prioritized_moves;
use crate::bot::include::types::EngineState;
use crate::bot::util::board::BoardExt;

pub fn quiescence_search(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    nodes: &mut u64,
    deadline: Instant,
    engine_state: &mut EngineState,
    current_depth: u8,
    max_depth_reached: &mut u8,
    color: i32,
    qdepth: u8
) -> (Vec<ChessMove>, i32, i32) {
    if Instant::now() >= deadline {
        return (vec![], 0, board.noiseness());
    }

    *nodes += 1;
    *max_depth_reached = (*max_depth_reached).max(current_depth);
    let board_hash = board.get_hash();
    let repetition_count = engine_state.history.get(board_hash);

    if let Some((_, score)) = is_terminal(board, current_depth, repetition_count) {
        return (vec![], score * color, 0);
    }

    if qdepth == 0 || board.is_endgame() {
        let static_eval = evaluate_board(board);
        return (vec![], color * static_eval, board.noiseness());
    }

    let mut best_line = vec![];
    let mut best_score = i32::MIN;
    let mut best_noise = 0;

    for (mv, _) in get_prioritized_moves(board, true) {
        let new_board = board.make_move_new(mv);
        let new_hash = new_board.get_hash();
        engine_state.history.increment(new_hash);

        let (child_line, eval, noise) = quiescence_search(
            &new_board,
            -beta,
            -alpha,
            nodes,
            deadline,
            engine_state,
            current_depth + 1,
            max_depth_reached,
            -color,
            qdepth - 1
        );

        engine_state.history.decrement(new_hash);
        let score = -eval;

        if score >= beta {
            return (vec![mv], score, noise);
        }

        if score > best_score {
            best_score = score;
            alpha = alpha.max(score);
            best_line = vec![mv];
            best_line.extend(child_line);
            best_noise = noise;
        }
    }

    if best_score == i32::MIN {
        let static_eval = color * evaluate_board(board);
        return (vec![], static_eval, board.noiseness());
    }

    (best_line, best_score, best_noise)
}
