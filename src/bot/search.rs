use crate::base::defs::{Board, Search, GameState};

pub fn generate_game_tree( curr_board: Board, max_depth: u32, num_nodes: &mut u64 ) {
    *num_nodes += 1;
    if max_depth == 0 {
        return;
    }
    for board in curr_board.get_legal_moves() {
        generate_game_tree(board, max_depth-1, num_nodes);
    }
}

impl Search {

    pub fn alpha_beta_pruning(board: Board, mut alpha: f64, mut beta: f64, depth: u32, maximizing_player: bool) -> f64 {
        if depth == 0 || matches!(board.get_game_state(), GameState::Checkmate | GameState::Stalemate) {
            return board.evaluate();
        }

        let legal_moves = board.get_legal_moves();

        if maximizing_player {
            let mut max_eval = f64::NEG_INFINITY;
            for next_board in legal_moves {
                let eval = Search::alpha_beta_pruning(next_board, alpha, beta, depth - 1, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = f64::INFINITY;
            for next_board in legal_moves {
                let eval = Search::alpha_beta_pruning(next_board, alpha, beta, depth - 1, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }

    pub fn best_next_board(&self) -> Board {
        let is_black: i32 = if ( self.board.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };
        let mut best_move = None;
        let mut best_eval = if is_black==0 { f64::NEG_INFINITY } else { f64::INFINITY };
        let depth = 5;
        for next_board in self.board.get_legal_moves() {
            let eval = Search::alpha_beta_pruning(next_board, f64::NEG_INFINITY, f64::INFINITY, depth - 1, is_black==1);
            
            if is_black == 0 && eval > best_eval {
                best_eval = eval;
                best_move = Some(next_board);
            }
            else if is_black == 1 && eval < best_eval {
                best_eval = eval;
                best_move = Some(next_board);
            }
        }
        
        println!( "Evaluation Function: {}", best_move.expect("There are no legal moves left").evaluate() );
        best_move.expect("There should be at least one legal move.")
    }

}