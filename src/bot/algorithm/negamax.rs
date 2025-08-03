use chess::{ Board, ChessMove };
use std::time::Instant;
use crate::bot::algorithm::eval::{ evaluate_board, is_terminal };
use crate::bot::algorithm::quiet::quiescence_search;
use crate::bot::algorithm::root::get_prioritized_moves;
use crate::bot::include::types::{ BoundType, EngineState, TTEntry };

pub fn negamax(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    nodes: &mut u64,
    deadline: Instant,
    engine_state: &mut EngineState,
    depth: u8,
    current_depth: u8,
    max_depth_reached: &mut u8,
    color: i32
) -> (Option<ChessMove>, i32) {
    if Instant::now() >= deadline {
        return (None, 0);
    }

    *nodes += 1;
    *max_depth_reached = (*max_depth_reached).max(current_depth);

    let board_hash = board.get_hash();
    let repetition_count = engine_state.history.get(board_hash);

    // Transposition Table Lookup
    {
        if let Some(entry) = engine_state.transposition_table.get(&(board_hash, depth)) {
            match entry.flag {
                BoundType::Exact => {
                    return (entry.best_move, entry.value);
                }
                BoundType::LowerBound if entry.value >= beta => {
                    return (entry.best_move, entry.value);
                }
                BoundType::UpperBound if entry.value <= alpha => {
                    return (entry.best_move, entry.value);
                }
                _ => {}
            }
        }
    }

    // Check terminal state
    if
        let Some(result) = is_terminal(
            board,
            board_hash,
            depth,
            current_depth,
            color,
            repetition_count,
            engine_state
        )
    {
        return result;
    }

    // Perform quiet search if depth is 0
    if depth == 0 {
        // let eval = quiescence_search(
        //     board,
        //     alpha,
        //     beta,
        //     nodes,
        //     deadline,
        //     engine_state,
        //     current_depth,
        //     max_depth_reached,
        //     color
        // );
        let eval = evaluate_board(board);
        return (None, eval * color);
    }

    // Move generation
    let prioritized_moves = get_prioritized_moves(board, false);
    if prioritized_moves.is_empty() {
        let eval = evaluate_board(board);
        return (None, eval * color);
    }

    // Negamax search
    let mut best_move = None;
    let mut best_eval = i32::MIN;
    let mut flag = BoundType::UpperBound;

    for (mv, _) in prioritized_moves {
        let new_board = board.make_move_new(mv);
        let new_hash = new_board.get_hash();
        engine_state.history.increment(new_hash);

        let (_, eval) = negamax(
            &new_board,
            -beta,
            -alpha,
            nodes,
            deadline,
            engine_state,
            depth - 1,
            current_depth + 1,
            max_depth_reached,
            -color
        );

        let score = -eval;

        engine_state.history.decrement(new_hash);

        if score > best_eval {
            best_eval = score;
            best_move = Some(mv);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            flag = BoundType::LowerBound;
            break;
        } else {
            flag = BoundType::Exact;
        }
    }

    // Store result in transposition table
    engine_state.transposition_table.put((board_hash, depth), TTEntry {
        value: best_eval,
        depth,
        flag,
        best_move,
    });

    (best_move, best_eval)
}
