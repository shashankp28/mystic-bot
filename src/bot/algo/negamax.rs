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
        // Check for terminal positions (checkmate or draw)
        self.num_nodes += 1;
        let mut legal_moves = board.get_legal_moves();
        if legal_moves.len() == 0 {
            return (Some(*board), board.evaluate(true) * (depth_remaining as f64) * colour);
        }

        // If depth is zero or time runs out, evaluate the position
        if depth_remaining == 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (Some(*board), board.evaluate(false) * colour);
        }
        self.sort_legal_moves(&mut legal_moves, colour == -1.0);

        let mut best_score = f64::NEG_INFINITY;
        let mut best_move: Option<Board> = None;
        for new_board in legal_moves {
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
        (best_move, best_score)
    }
}
