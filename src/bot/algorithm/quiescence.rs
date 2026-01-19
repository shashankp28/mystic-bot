use chess::MoveGen;
use crate::bot::{
    algorithm::eval::evaluate_board,
    include::types::SearchContext,
    util::board::BoardExt,
};

pub fn quiescence(context: &mut SearchContext, mut alpha: i32, beta: i32, depth: i32) -> i32 {
    context.nodes_visited += 1;

    let stand_pat = evaluate_board(&context.board);

    if stand_pat >= beta {
        return beta;
    }

    let queen_value = 900;
    if stand_pat < alpha - queen_value && depth > 0 {
        return alpha;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    if depth > 8 {
        return stand_pat;
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
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
