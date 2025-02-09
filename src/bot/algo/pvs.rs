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
        // Check for terminal positions (checkmate or draw)
        let mut legal_moves = board.get_legal_moves();
        if legal_moves.len() == 0 {
            return (None, board.evaluate(true) * (depth_remaining as f64) * colour);
        }

        // If depth is zero or time runs out, evaluate the position
        if depth_remaining == 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (None, board.evaluate(false) * colour);
        }
        self.sort_legal_moves(&mut legal_moves, colour == -1.0);

        let mut is_first_child = true;
        let mut score: f64;
        let mut best_move: Option<Board> = None;
        for next_board in legal_moves {
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
        (best_move, alpha)
    }
}
