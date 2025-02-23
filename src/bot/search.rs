extern crate random_choice;
use crate::base::defs::GlobalMap;
use crate::base::defs::{ Board, Search };
use crate::base::utils::uci_to_uint;
use rand::thread_rng;
use rand::Rng;
use core::f64;
use std::time::{ Duration, Instant };
use serde_json::Value;
use self::random_choice::random_choice;

impl Search {
    /// Returns the best move using iterative deepening and PVS
    pub fn best_next_board(&mut self, time_limit: Duration) -> Option<Board> {
        let start_time: Instant = Instant::now();
        let is_black = ((self.board.metadata >> 8) & 1) == 0;
        let colour = if is_black { -1.0 } else { 1.0 };
        let mut best_move: Option<Board> = None;
        let mut best_eval = if is_black { f64::NEG_INFINITY } else { f64::INFINITY };

        // If only 1 legal move available, return that directly
        let legal_moves = self.board.get_legal_moves();
        if legal_moves.len() == 1 {
            best_move = Some(legal_moves.data[0]);
            best_eval = legal_moves.data[0].eval;
            self.max_depth = u32::MAX;
        }
        // Iterative deepening loop
        while self.max_depth <= 15 && Instant::now().duration_since(start_time) < time_limit {
            let board = self.board.clone();

            // Mini-Max Algorithm
            // let (local_best_move, mut local_best_eval) = self.minimax(
            //     &board,
            //     self.max_depth,
            //     time_limit,
            //     &start_time,
            //     -colour
            // );

            // Negamax algorithm (Alpha-Beta Pruning)
            let (local_best_move, mut local_best_eval) = self.nega_max(
                &board,
                f64::NEG_INFINITY,
                f64::INFINITY,
                self.max_depth,
                time_limit,
                &start_time,
                colour
            );
            local_best_eval *= colour;

            // PVS algorithm
            // let (local_best_move, mut local_best_eval) = self.pvs(
            //     &board,
            //     f64::NEG_INFINITY,
            //     f64::INFINITY,
            //     self.max_depth,
            //     time_limit,
            //     &start_time,
            //     colour
            // );
            // local_best_eval *= colour;

            // Stop if time limit is exceeded (don't update half exploration)
            if Instant::now().duration_since(start_time) > time_limit {
                break;
            }

            // Update global best move (deeper searches provide better results)
            best_eval = local_best_eval;
            best_move = local_best_move;

            self.max_depth += 1; // Increment depth for the next iteration
        }

        // Calculate elapsed time
        let elapsed_time = start_time.elapsed();

        // Output evaluation details
        println!("Evaluation Function: {}", best_eval);
        println!("Number of Nodes Explored: {}", self.num_nodes);
        println!("Depth Explored: {}", self.max_depth - 1);
        println!("Number of Nodes Pruned: {}", self.num_prunes);
        println!("Time Taken: {:?} ms", elapsed_time.as_millis());
        println!(
            "Explored Nodes per second: {:.2}",
            (self.num_nodes as f64) / elapsed_time.as_secs_f64()
        );

        best_move
    }

    pub fn search_opening_db(&self) -> Option<Board> {
        // Check if board.hash() exists in the opening_db
        let board_hash = self.board.hash();
        if let Some(entry) = GlobalMap::opening_db().get(&board_hash.to_string()) {
            println!("Found hash in opening database: {:?}", board_hash);

            // Parse the entry to extract values
            if let Value::Object(moves) = entry {
                let mut db_moves = Vec::new();
                let mut scores = Vec::new();

                for (move_str, stats) in moves {
                    if let Value::Object(stats_map) = stats {
                        let total = stats_map
                            .get("total")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(1.0);
                        db_moves.push(move_str.clone());
                        scores.push(total);
                    }
                }

                if scores.is_empty() {
                    return None;
                }

                // Randomly select a move based on weighted scores
                let number_choices = 1;
                let choices = random_choice().random_choice_f64(&db_moves, &scores, number_choices);
                assert!(choices.len() == 1);

                for choice in choices {
                    println!("Move selected using Opening DB: {}", choice);
                    let legal_moves = self.board.get_legal_moves();
                    let selected_move = uci_to_uint(choice);
                    for next_board in legal_moves {
                        if next_board.latest_move == selected_move {
                            return Some(next_board);
                        }
                    }
                }

                None
            } else {
                println!("No moves found for hash: {:?}", board_hash);
                return None;
            }
        } else {
            println!("Hash not found in the database: {:?}", board_hash);
            return None;
        }
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

#[cfg(test)]
mod tests {
    use crate::base::defs::{ Board, Search };
    use std::{ collections::HashMap, time::Duration };

    #[test]
    fn test_complex_search() {
        let fen = String::from("r1b1kb1r/ppp2ppp/2n2n2/4P3/2B5/4PN2/PP3PPP/RNBqK2R w KQkq - 0 1");
        match Board::from_fen(&fen) {
            Some(board) => {
                let memory: HashMap<u64, f64> = HashMap::new();
                let mut search = Search {
                    board,
                    memory,
                    num_nodes: 0,
                    max_depth: 3,
                    num_prunes: 0,
                };
                let best_move = search.best_next_board(Duration::from_millis(5000));
                match best_move {
                    Some(new_board) => {
                        println!("Best Move Found: {}", new_board.get_next_uci());
                    }
                    None => {
                        println!("Best move not found!!");
                    }
                }
            }
            None => {
                println!("Error loading board: {}", fen);
            }
        }
    }
}
