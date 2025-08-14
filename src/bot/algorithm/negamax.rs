use chess::{ Board, ChessMove };
use std::time::Instant;
use crate::bot::algorithm::eval::is_terminal;
use crate::bot::algorithm::quiet::quiescence_search;
use crate::bot::algorithm::root::get_prioritized_moves;
use crate::bot::include::types::{ EngineState, QUIET_FALL_SHARPNESS };
use rayon::prelude::*;

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
) -> (Vec<ChessMove>, i32) {
    if Instant::now() >= deadline {
        return (vec![], 0);
    }

    *nodes += 1;
    *max_depth_reached = (*max_depth_reached).max(current_depth);

    let board_hash = board.get_hash();
    let repetition_count = engine_state.history.get(board_hash);

    if let Some((_, score)) = is_terminal(board, current_depth, repetition_count) {
        return (vec![], score * color);
    }

    if depth == 0 {
        let (q_line, q_eval, noise_level) = quiescence_search(
            board,
            alpha,
            beta,
            nodes,
            deadline,
            engine_state,
            current_depth + 1,
            max_depth_reached,
            color,
            current_depth
        );

        let static_eval = crate::bot::algorithm::eval::evaluate_board(board) * color;
        let noise_factor = ((noise_level as f32) / 2500.0).clamp(0.0, 1.0);
        let n = QUIET_FALL_SHARPNESS;
        let q_weight = 1.0 - (1.0 - (1.0 - noise_factor).powf(n)).powf(1.0 / n);
        let adjusted_eval = ((static_eval as f32) * (1.0 - q_weight) +
            (q_eval as f32) * q_weight) as i32;

        return (q_line, adjusted_eval);
    }

    let prioritized_moves = get_prioritized_moves(board, false);
    let mut best_line = vec![];
    let mut best_eval = i32::MIN;

    if current_depth == 0 {
        let beta_shared = beta;
        let depth_shared = depth;
        let deadline_shared = deadline;
        let color_shared = color;

        let results: Vec<(Vec<ChessMove>, i32, u64, u8)> = prioritized_moves
            .par_iter()
            .map(|(mv, _)| {
                let mut local_engine_state = engine_state.clone();
                let new_board = board.make_move_new(*mv);
                let new_hash = new_board.get_hash();
                local_engine_state.history.increment(new_hash);

                let mut local_nodes = 0u64;
                let mut local_max_depth = *max_depth_reached;

                let (child_line, eval) = negamax(
                    &new_board,
                    -beta_shared,
                    -alpha,
                    &mut local_nodes,
                    deadline_shared,
                    &mut local_engine_state,
                    depth_shared - 1,
                    current_depth + 1,
                    &mut local_max_depth,
                    -color_shared
                );

                let score = -eval;
                let mut line = vec![*mv];
                line.extend(child_line);

                (line, score, local_nodes, local_max_depth)
            })
            .collect();

        let mut total_nodes_from_threads = 0u64;
        for (line, score, local_nodes, local_max_depth) in results {
            total_nodes_from_threads += local_nodes;
            *max_depth_reached = (*max_depth_reached).max(local_max_depth);

            if score > best_eval {
                best_eval = score;
                best_line = line;
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }
        *nodes += total_nodes_from_threads;
    } else {
        for (mv, _) in prioritized_moves {
            let new_board = board.make_move_new(mv);
            let new_hash = new_board.get_hash();
            engine_state.history.increment(new_hash);

            let (child_line, eval) = negamax(
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

            engine_state.history.decrement(new_hash);
            let score = -eval;

            if score > best_eval {
                best_eval = score;
                best_line = vec![mv];
                best_line.extend(child_line);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }
    }
    (best_line, best_eval)
}
