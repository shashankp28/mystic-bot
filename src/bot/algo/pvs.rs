use std::time::{ Duration, Instant };
use crate::base::defs::{ Board, Search };

impl Search {
    /// Principal Variation Search (PVS) with Alpha-Beta Pruning
    ///
    /// Reference: https://en.wikipedia.org/wiki/Principal_variation_search
    pub fn pvs(
        &mut self,
        board: &Board,
        mut alpha: f64,
        beta: f64,
        depth_remaining: u32,
        time_limit: Duration,
        start_time: &Instant,
        colour: f64
    ) -> (Option<Board>, f64) {
        // If depth is zero or time runs out, evaluate the position
        if depth_remaining <= 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (None, board.eval * colour);
        }

        let mut is_first_child = true;
        let mut score: f64;
        let mut best_move: Option<Board> = None;
        // Ideally it should be `let end = legal_moves.len()==0`
        // For some reason, pre-computing legal_moves is slower than iterating on the fly!!
        // Even though it is not returning an iterator
        let mut end = true;
        for next_board in board.get_legal_moves() {
            end &= false;
            self.num_nodes += 1;

            if is_first_child {
                // Full window search for the first move
                (_, score) = self.pvs(
                    &next_board,
                    -beta,
                    -alpha,
                    depth_remaining - 1,
                    time_limit,
                    start_time,
                    -colour
                );
                score *= -1.0;
            } else {
                // Narrow window search for other moves (Principal Variation Search)
                (_, score) = self.pvs(
                    &next_board,
                    -alpha - 1.0,
                    -alpha,
                    depth_remaining - 1,
                    time_limit,
                    start_time,
                    -colour
                );
                score *= -1.0;

                // If the narrow window search fails, do a full re-search
                if score > alpha && score < beta {
                    (_, score) = self.pvs(
                        &next_board,
                        -beta,
                        -alpha,
                        depth_remaining - 1,
                        time_limit,
                        start_time,
                        -colour
                    );
                    score *= -1.0;
                }
            }

            if score > alpha {
                alpha = score;
                best_move = Some(next_board);
            }

            if alpha >= beta {
                self.num_prunes += 1;
                break; // Beta cutoff
            }

            is_first_child = false;
        }
        // Check for terminal positions (checkmate or draw)
        if end {
            // Runs only if for loop didn't run
            return (None, board.evaluate(true) * (depth_remaining as f64) * colour);
        }
        (best_move, alpha)
    }
}
