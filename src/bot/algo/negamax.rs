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

        // 3-fold repetition draw
        if let Some(&count) = self.memory.get(&board.static_hash()) {
            if count >= 3 {
                return (None, 0.0);
            }
        }

        // Half move clock has reached 100, is a draw
        if (board.metadata >> 9) & 0b1111111 >= 100 {
            return (None, 0.0);
        }

        if depth_remaining <= 0 || Instant::now().duration_since(*start_time) > time_limit {
            return (None, board.eval * colour);
        }

        let mut best_score = f64::NEG_INFINITY;
        let mut best_move: Option<Board> = None;

        let mut end = true;
        for new_board in board.get_legal_moves() {
            end &= false;
            let hash = new_board.static_hash();
            *self.memory.entry(hash).or_insert(0) += 1;
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

            if let Some(count) = self.memory.get_mut(&(hash as u128)) {
                *count -= 1;
                if *count == 0 {
                    self.memory.remove(&(hash as u128));
                }
            }

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
