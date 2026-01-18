use std::{ sync::{ atomic::Ordering } };
use chess::{ ChessMove, MoveGen };
use tracing::{ info, debug };
use crate::bot::algorithm::quiescence::quiescence;
use crate::bot::include::types::*;
use crate::bot::util::board::BoardExt;
use crate::bot::util::lookup::{ store_killer };

pub fn negamax(
    context: &mut SearchContext,
    depth: i32,
    ply: usize,
    mut alpha: i32,
    beta: i32
) -> (i32, Vec<ChessMove>) {
    context.nodes_visited += 1;

    if context.stop_signal.load(Ordering::Relaxed) {
        return (0, vec![]);
    }

    if depth <= 0 {
        return (quiescence(context, alpha, beta, 0), vec![]);
    }

    let hash = context.board.get_hash();

    if let Some(tt) = context.tt.probe(hash, depth as u8, alpha, beta) {
        return (tt.value as i32, vec![]);
    }

    let mut best_score = -INF;
    let mut best_move = None;
    let mut best_pv = Vec::new();

    let moves = ordered_moves(context, ply);

    if moves.is_empty() {
        return if context.board.checkers().popcnt() > 0 {
            let mate_score = -MATE_SCORE_BASE + (ply as i32);
            info!(ply, score = mate_score, "Checkmate detected");
            (mate_score, vec![])
        } else {
            debug!(ply, "Stalemate detected");
            (0, vec![])
        };
    }

    for mv in moves {
        let next = context.board.make_move_new(mv);
        let old = std::mem::replace(&mut context.board, next);

        let (score, pv) = negamax(context, depth - 1, ply + 1, -beta, -alpha);
        let score = -score;

        context.board = old;

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
            best_pv = vec![mv];
            best_pv.extend(pv);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            store_killer(context, ply, mv);
            break;
        }
    }

    context.tt.store(hash, best_score, depth as u8, alpha, beta, best_move);

    (best_score, best_pv)
}

fn ordered_moves(context: &SearchContext, ply: usize) -> Vec<ChessMove> {
    let mut moves: Vec<_> = MoveGen::new_legal(&context.board).collect();

    moves.sort_by_key(|mv| {
        let mut score = context.board.move_priority(*mv);

        if Some(*mv) == context.killer_moves[ply][0] {
            score += 9_000;
        } else if Some(*mv) == context.killer_moves[ply][1] {
            score += 8_000;
        }

        let src = mv.get_source().to_index();
        let dst = mv.get_dest().to_index();
        score += context.history_scores[src][dst];

        -score
    });

    moves
}
