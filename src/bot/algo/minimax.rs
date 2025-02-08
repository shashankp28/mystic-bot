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
        let legal_moves = board.get_legal_moves();
        if legal_moves.len() == 0 {
            return (Some(*board), board.evaluate(true) * (depth_remaining as f64) * colour);
        }

        // If depth is zero or time runs out, evaluate the position
        if depth_remaining == 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (Some(*board), board.evaluate(false) * colour);
        }

        let mut best_score = if colour == 1.0 { f64::NEG_INFINITY } else { f64::INFINITY };
        let mut best_move: Option<Board> = None;

        for child_board in legal_moves {
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
        (best_move, best_score)
    }
}
