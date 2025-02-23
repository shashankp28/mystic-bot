use std::time::{ Duration, Instant };

use crate::base::defs::{ Board, Search };

impl Search {
    pub fn minimax(
        &mut self,
        board: &Board,
        depth_remaining: u32,
        time_limit: Duration,
        start_time: &Instant,
        colour: f64
    ) -> (Option<Board>, f64) {
        self.num_nodes += 1;

        // If depth is zero or time runs out, evaluate the position
        if depth_remaining == 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (None, board.eval * colour);
        }

        let mut best_score = if colour == 1.0 { f64::NEG_INFINITY } else { f64::INFINITY };
        let mut best_move: Option<Board> = None;
        let mut end = true;
        for child_board in board.get_legal_moves() {
            end &= false;
            let (_, score) = self.minimax(
                &child_board,
                depth_remaining - 1,
                time_limit,
                start_time,
                -colour
            );
            if (colour == 1.0 && score > best_score) || (colour == -1.0 && score < best_score) {
                best_score = score;
                if depth_remaining == self.max_depth {
                    best_move = Some(child_board);
                }
            }
        }
        // Check for terminal positions (checkmate or draw)
        if end {
            // Runs only if for loop didn't run
            return (None, board.evaluate(true) * (depth_remaining as f64) * colour);
        }
        (best_move, best_score)
    }
}
