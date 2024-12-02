use crate::base::defs::LegalMoveVec;
use crate::base::defs::{Board, GameState, Search};
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::time::{Duration, Instant};

pub fn generate_game_tree(curr_board: Board, max_depth: u32, num_nodes: &mut u64) {
    *num_nodes += 1;
    if max_depth == 0 {
        return;
    }
    for board in curr_board.get_legal_moves() {
        generate_game_tree(board, max_depth - 1, num_nodes);
    }
}

impl Search {
    fn sort_legal_moves(&mut self, legal_moves: &mut LegalMoveVec, is_black: bool) {
        legal_moves.data.sort_by(|a, b| {
            let a_evaluation = a.evaluate();
            let b_evaluation = b.evaluate();

            let order = a_evaluation
                .partial_cmp(&b_evaluation)
                .unwrap_or(Ordering::Equal);

            if is_black {
                order // Ascending for black
            } else {
                order.reverse() // Descending for white
            }
        });
    }

    pub fn alpha_beta_pruning(
        &mut self,
        board: &Board,
        mut alpha: f64,
        mut beta: f64,
        depth: u32,
        maximizing_player: bool,
        time_limit: Duration,
        start_time: &Instant,
    ) -> f64 {

        //  If depth is 0 or time is up evaluate and return
        if depth == 0 || Instant::now().duration_since(*start_time) > time_limit {
            let eval = board.evaluate();
            return eval;
        }

        // If checkmate or draw return appropriate score
        let mut legal_moves = board.get_legal_moves();
        let is_black: u8 = if (board.metadata >> 8) & 1 == 1 { 0 } else { 1 };
        let game_state = if legal_moves.len() == 0 {
            let king_positions: u64 = (board.kings >> (64 * is_black)) as u64;
            let pos: i8 = king_positions.trailing_zeros() as i8;
            let index: i8 = 63 - pos;
            if board.can_attack(1 - is_black, index as u8) {
                GameState::Checkmate
            } else {
                GameState::Stalemate
            }
        } else {
            GameState::Playable
        };
        match game_state {
            GameState::Checkmate => {
                if is_black == 1 {
                    return 100000.0*( 100.0 - ( self.max_depth - depth ) as f64 );
                } else {
                    return -100000.0*( 100.0 - ( self.max_depth - depth ) as f64 );
                }
            }
            GameState::Stalemate => return 0.0,
            GameState::Playable => {}
        }

        self.sort_legal_moves(&mut legal_moves, is_black == 1);

        if maximizing_player {
            let mut max_eval = f64::NEG_INFINITY;
            for next_board in legal_moves {
                self.num_nodes += 1;
                let eval = self.alpha_beta_pruning(&next_board, alpha, beta, depth - 1, false, time_limit, start_time);
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
                self.num_nodes += 1;
                let eval = self.alpha_beta_pruning(&next_board, alpha, beta, depth - 1, true, time_limit, start_time);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }

    pub fn best_next_board(&mut self, time_limit: Duration, start_time: &Instant) -> Option<Board> {
        let local_time = Instant::now();
        let is_black: i32 = if (self.board.metadata >> 8) & 1 == 1 { 0 } else { 1 };

        let mut best_move: Option<Board> = None;
        let mut best_eval = if is_black == 0 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };

        let legal_moves = self.board.get_legal_moves();

        // Depth search loop
        while self.max_depth <= 15 && Instant::now().duration_since(*start_time) < time_limit {
            let mut local_best_move: Option<Board> = None;
            let mut local_best_eval = if is_black == 0 {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            };

            // Iterate over legal moves
            for next_board in legal_moves.iter() {
                self.num_nodes += 1;

                // Perform alpha-beta pruning
                let eval = self.alpha_beta_pruning(
                    next_board,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    self.max_depth - 1,
                    is_black == 1,
                    time_limit,
                    start_time,
                );

                // Update local best move and eval
                if is_black == 0 && eval > local_best_eval {
                    local_best_eval = eval;
                    local_best_move = Some(next_board.clone());
                } else if is_black == 1 && eval < local_best_eval {
                    local_best_eval = eval;
                    local_best_move = Some(next_board.clone());
                }
            }

            // Stop if time limit is exceeded
            if Instant::now().duration_since(*start_time) > time_limit {
                break;
            }

            // Update global best move as Higher depth ==> Better result
            best_eval = local_best_eval;
            best_move = local_best_move;

            self.max_depth += 1; // Increment depth for the next iteration
        }

        // Calculate elapsed time
        let elapsed_time = local_time.elapsed();

        // Output evaluation details
        println!("Evaluation Function: {}", best_eval);
        println!("Number of Nodes Explored: {}", self.num_nodes);
        println!("Depth Explored: {}", self.max_depth-1); // Depth adjusted to last successful iteration
        println!("Time Taken: {:?}", elapsed_time);
        println!(
            "Explored Nodes per second: {:.2}",
            self.num_nodes as f64 / elapsed_time.as_secs_f64()
        );

        best_move
    }

    pub fn random_next_board(&self) -> Option<Board> {
        let legal_moves = self.board.get_legal_moves();
        let len = legal_moves.len();

        if len == 0 {
            return None;
        }

        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..len);

        legal_moves.choose(random_index).cloned()
    }
}
