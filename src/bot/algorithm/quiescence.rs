use chess::MoveGen;
use crate::bot::algorithm::eval::evaluate_board;
use crate::bot::include::types::*;
use crate::bot::util::board::BoardExt;
use tracing::{ debug, trace };

pub fn quiescence(context: &mut SearchContext, mut alpha: i32, beta: i32, depth: i32) -> i32 {
    context.nodes_visited += 1;

    if (context.nodes_visited & 0xfffff) == 0 {
        debug!(nodes = context.nodes_visited, alpha, beta);
    }

    let stand_pat = evaluate_board(&context.board);
    if stand_pat >= beta {
        return beta;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves: Vec<_> = MoveGen::new_legal(&context.board)
        .filter(|m| (context.board.is_attack(*m) || context.board.is_en_passant(*m)))
        .collect();

    moves.sort_by_key(|m| -context.board.move_priority(*m));

    for mv in moves {
        let next = context.board.make_move_new(mv);
        let old = std::mem::replace(&mut context.board, next);

        let score = -quiescence(context, -beta, -alpha, depth + 1);

        context.board = old;

        if score >= beta {
            if score > stand_pat + 1000 {
                trace!(mv = ?mv, score, "Major tactical shift");
            }
            return beta;
        }

        if score > alpha {
            alpha = score;
            if depth == 0 {
                debug!(mv = ?mv, score, "New best move in Q-search");
            }
        }
    }

    alpha
}
