use crate::base::defs::{Board, GameState, Search};
use std::cmp::Ordering;
use std::time::Instant;
use rand::thread_rng;
use rand::Rng;

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
    pub fn alpha_beta_pruning(
        &mut self,
        board: Board,
        mut alpha: f64,
        mut beta: f64,
        depth: u32,
        maximizing_player: bool,
    ) -> f64 {
        //  Get cached eval score
        let board_hash = board.hash();
        if self.memory.contains_key(&board_hash) {
            return *self.memory.get(&board_hash).unwrap();
        }

        //  If depth is 0 evaluate and return
        if depth == 0 {
            let eval = board.evaluate();
            self.memory.insert(board.hash(), eval);
            return eval;
        }

        // If checkmate or draw return appropriate score
        let mut legal_moves = board.get_legal_moves();
        let is_black: u8 = if ( board.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };
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
                    return 100000.0;
                } else {
                    return -100000.0;
                }
            }
            GameState::Stalemate => return 0.0,
            GameState::Playable => {}
        }

        legal_moves.data.sort_by(|a, b| {
            let a_evaluation;
            let b_evaluation;

            if self.memory.contains_key(&a.hash()) {
                a_evaluation = *self.memory.get(&a.hash()).unwrap();
            } else {
                a_evaluation = board.evaluate();
                self.memory.insert(a.hash(), a_evaluation);
            }

            if self.memory.contains_key(&b.hash()) {
                b_evaluation = *self.memory.get(&b.hash()).unwrap();
            } else {
                b_evaluation = board.evaluate();
                self.memory.insert(b.hash(), b_evaluation);
            }

            let order = a_evaluation
                .partial_cmp(&b_evaluation)
                .unwrap_or(Ordering::Equal);

            if is_black == 1 {
                order // Ascending for black
            } else {
                order.reverse() // Descending for white
            }
        });

        if maximizing_player {
            let mut max_eval = f64::NEG_INFINITY;
            for next_board in legal_moves {
                self.num_nodes += 1;
                let eval = self.alpha_beta_pruning(next_board, alpha, beta, depth - 1, false);
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
                let eval = self.alpha_beta_pruning(next_board, alpha, beta, depth - 1, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }

    pub fn best_next_board(&mut self) -> Option<Board> {
        let start_time = Instant::now();
        let is_black: i32 = if (self.board.metadata >> 8) & 1 == 1 {
            0
        } else {
            1
        };
        let mut best_move = None;
        let mut best_eval = if is_black == 0 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
        let depth = 15;
        for next_board in self.board.get_legal_moves() {
            self.num_nodes += 1;
            let eval = self.alpha_beta_pruning(
                next_board,
                f64::NEG_INFINITY,
                f64::INFINITY,
                depth - 1,
                is_black == 1,
            );

            if is_black == 0 && eval > best_eval {
                best_eval = eval;
                best_move = Some(next_board);
            } else if is_black == 1 && eval < best_eval {
                best_eval = eval;
                best_move = Some(next_board);
            }
        }

        let elapsed_time = start_time.elapsed();
        println!("Evaluation Function: {}", best_eval);
        println!("Number of Nodes Explored: {}", self.num_nodes);
        println!("Time Taken: {:?}", elapsed_time);
        println!(
            "Explored Nodes per second: {}",
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
