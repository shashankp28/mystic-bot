use chess::Board;
use std::time::Instant;
use crate::bot::algorithm::eval::{ evaluate_board, is_terminal };
use crate::bot::algorithm::root::get_prioritized_moves;
use crate::bot::include::types::EngineState;

pub fn quiescence_search(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    nodes: &mut u64,
    deadline: Instant,
    engine_state: &mut EngineState,
    current_depth: u8,
    max_depth_reached: &mut u8,
    color: i32
) -> i32 {
    if Instant::now() >= deadline {
        return 0;
    }

    *nodes += 1;
    *max_depth_reached = (*max_depth_reached).max(current_depth);

    let board_hash = board.get_hash();
    let repetition_count = engine_state.history.get(board_hash);

    // ✅ Reuse is_terminal
    if
        let Some((_, score)) = is_terminal(
            board,
            board_hash,
            0, // quiescence search is depth = 0
            current_depth,
            color,
            repetition_count,
            engine_state
        )
    {
        return score;
    }

    let stand_pat = color * evaluate_board(board);

    if stand_pat >= beta {
        return stand_pat;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    for (mv, _) in get_prioritized_moves(board, true) {
        let new_board = board.make_move_new(mv);
        let new_hash = new_board.get_hash();
        engine_state.history.increment(new_hash);

        let score = -quiescence_search(
            &new_board,
            -beta,
            -alpha,
            nodes,
            deadline,
            engine_state,
            current_depth + 1,
            max_depth_reached,
            -color
        );

        engine_state.history.decrement(new_hash);

        if score >= beta {
            return score;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
