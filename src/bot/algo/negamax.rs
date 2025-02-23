use std::time::{ Duration, Instant };
use crate::base::defs::{ Board, Search };

impl Search {
    pub fn nega_max(
        &mut self,
        board: &Board,
        mut alpha: f64,
        beta: f64,
        depth_remaining: u32,
        time_limit: Duration,
        start_time: &Instant,
        colour: f64
    ) -> (Option<Board>, f64) {
        self.num_nodes += 1;

        if depth_remaining <= 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (None, board.eval * colour);
        }

        let mut best_score = f64::NEG_INFINITY;
        let mut best_move: Option<Board> = None;

        // Null Move Pruning
        if depth_remaining > 2 && !board.in_check() {
            let null_beta = -beta;
            let null_depth = depth_remaining - 2; // Reduce depth more aggressively for null moves
            let (_, null_score) = self.nega_max(
                board, // No move played, skipping the turn
                null_beta - 1.0, // Use a slightly tighter bound
                null_beta,
                null_depth,
                time_limit,
                start_time,
                -colour
            );

            if -null_score >= beta {
                return (None, beta); // Prune the branch
            }
        }

        let mut end = true;
        for new_board in board.get_legal_moves() {
            end &= false;
            let (_, mut score) = self.nega_max(
                &new_board,
                -beta,
                -alpha,
                depth_remaining - 1,
                time_limit,
                start_time,
                -colour
            );
            score *= -1.0;

            if score > best_score {
                best_score = score;
                best_move = Some(new_board);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                self.num_prunes += 1;
                break;
            }
        }

        if end {
            return (None, board.evaluate(true) * (depth_remaining as f64) * colour);
        }

        (best_move, best_score)
    }
}
