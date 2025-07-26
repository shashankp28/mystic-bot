use chess::{ Board, ChessMove, MoveGen, Piece, Square };
use std::time::{ Duration, Instant };
use std::str::FromStr;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bot::{ eval::evaluate_board, include::types::{ EngineState, GlobalMap } };

fn parse_uci_move(uci: &str, board: &Board) -> Option<ChessMove> {
    if uci.len() < 4 {
        return None;
    }

    let from = Square::from_str(&uci[0..2]).ok()?;
    let to = Square::from_str(&uci[2..4]).ok()?;

    let promotion = if uci.len() == 5 {
        match uci.chars().nth(4)? {
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            _ => None,
        }
    } else {
        None
    };

    let candidate = ChessMove::new(from, to, promotion);

    let legal = MoveGen::new_legal(board);
    legal.into_iter().find(|m| *m == candidate)
}

pub fn search(
    time_left_ms: u128,
    time_limit_ms: Option<u128>,
    board: &Board,
    engine_state: &EngineState
) -> (Option<ChessMove>, u64, u128, i32, u8) {
    let start_time = Instant::now();
    // Try Opening DB first
    let board_hash_str = board.get_hash().to_string();
    if let Some(opening_db) = GlobalMap::opening_db().as_object() {
        if let Some(entry_array) = opening_db.get(&board_hash_str).and_then(|v| v.as_array()) {
            let mut rng = thread_rng();
            if let Some(random_entry) = entry_array.choose(&mut rng) {
                if let Some(uci_str) = random_entry.get(0).and_then(|v| v.as_str()) {
                    if let Some(chess_move) = parse_uci_move(uci_str, board) {
                        println!("{}", chess_move);
                        return (
                            Some(chess_move),
                            0,
                            start_time.elapsed().as_millis(),
                            evaluate_board(&board.make_move_new(chess_move)),
                            0,
                        );
                    }
                }
            }
        }
    }

    let max_time = time_limit_ms.unwrap_or(time_left_ms / 40).min(time_left_ms);
    let deadline = start_time + Duration::from_millis(max_time as u64);

    let mut best_move = None;
    let mut best_eval = 0;
    let mut total_nodes = 0;
    let mut final_depth = 0;

    for depth in 1..=64 {
        let mut nodes = 0;
        let (mv, eval) = alpha_beta(
            &board,
            depth,
            i32::MIN + 1,
            i32::MAX - 1,
            board.side_to_move() == chess::Color::White,
            &mut nodes,
            deadline,
            engine_state
        );

        if Instant::now() >= deadline {
            break;
        }

        if let Some(m) = mv {
            best_move = Some(m);
            best_eval = eval;
            total_nodes += nodes;
            final_depth += 1;
        } else {
            break; // Timeout hit
        }
    }

    let time_taken = start_time.elapsed().as_millis();
    (best_move, total_nodes, time_taken, best_eval, final_depth)
}

fn alpha_beta(
    board: &Board,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
    nodes: &mut u64,
    deadline: Instant,
    engine_state: &EngineState
) -> (Option<ChessMove>, i32) {
    if Instant::now() >= deadline {
        return (None, 0); // Timeout
    }

    *nodes += 1;

    if depth == 0 || board.status() == chess::BoardStatus::Checkmate {
        let eval = evaluate_board(board);
        return (None, eval);
    }

    let mut best_move = None;
    let movegen = MoveGen::new_legal(board);

    if maximizing {
        let mut max_eval = i32::MIN;
        for mv in movegen {
            let new_board = board.make_move_new(mv);
            let (_, eval) = alpha_beta(
                &new_board,
                depth - 1,
                alpha,
                beta,
                false,
                nodes,
                deadline,
                engine_state
            );

            if Instant::now() >= deadline {
                return (None, 0);
            }

            if eval > max_eval {
                max_eval = eval;
                best_move = Some(mv);
            }

            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        (best_move, max_eval)
    } else {
        let mut min_eval = i32::MAX;
        for mv in movegen {
            let new_board = board.make_move_new(mv);
            let (_, eval) = alpha_beta(
                &new_board,
                depth - 1,
                alpha,
                beta,
                true,
                nodes,
                deadline,
                engine_state
            );

            if Instant::now() >= deadline {
                return (None, 0);
            }

            if eval < min_eval {
                min_eval = eval;
                best_move = Some(mv);
            }

            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        (best_move, min_eval)
    }
}
