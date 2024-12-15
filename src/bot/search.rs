extern crate random_choice;
use crate::base::defs::LegalMoveVec;
use crate::base::defs::{Board, GameState, Search};
use crate::base::utils::uci_to_uint;
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::time::{Duration, Instant};
use serde_json::Value;
use self::random_choice::random_choice;

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

    /// principal variation search
    /// 
    /// https://en.wikipedia.org/wiki/Principal_variation_search
    pub fn pvs(
        &mut self,
        board: &Board,
        mut alpha: f64,
        beta: f64,
        depth: u32,
        maximizing_player: bool,
        time_limit: Duration,
        start_time: &Instant,
    ) -> f64 {
        let color = if maximizing_player { 1.0 } else { -1.0 };
        //  If depth is 0 or time is up evaluate and return
        if depth == 0 || Instant::now().duration_since(*start_time) > time_limit {
            let eval = board.evaluate();
            return eval * color;
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
                    return 100000.0 * (depth as f64) * color; // white won
                } else {
                    return -100000.0 * (depth as f64) * color;
                }
            }
            GameState::Stalemate => return 0.0,
            GameState::Playable => {}
        }

        self.sort_legal_moves(&mut legal_moves, is_black == 1);

        let mut is_first_child = true;
        let mut score: f64;
        for next_board in legal_moves {
            self.num_nodes += 1;

            if is_first_child {
                score = -self.pvs(
                    &next_board,
                    -beta,
                    -alpha,
                    depth - 1,
                    !maximizing_player,
                    time_limit,
                    start_time,
                );
            } else {
                score = -self.pvs(
                    &next_board,
                    -alpha - 1.0,
                    -alpha,
                    depth - 1,
                    !maximizing_player,
                    time_limit,
                    start_time,
                );
                if alpha < score && score < beta {
                    score = -self.pvs(
                        &next_board,
                        -beta,
                        -alpha,
                        depth - 1,
                        !maximizing_player,
                        time_limit,
                        start_time,
                    )
                }
            }
            alpha = alpha.max(score);
            if alpha >= beta {
                self.num_prunes += 1;
                break;
            }
            is_first_child = false;
        }
        alpha
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
                    self.num_prunes += depth;
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
                    self.num_prunes += depth;
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


                let color = if is_black == 1 { 1.0 } else { -1.0 };
                let eval = self.pvs(
                    next_board,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    self.max_depth - 1,
                    is_black == 1,
                    time_limit,
                    start_time,
                ) * color;

                // // Perform alpha-beta pruning
                // let eval = self.alpha_beta_pruning(
                //     next_board,
                //     f64::NEG_INFINITY,
                //     f64::INFINITY,
                //     depth - 1,
                //     is_black == 1,
                //     time_limit,
                //     start_time,
                // );

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
        println!("Number of Nodes Pruned: {}", self.num_prunes);
        println!("Time Taken: {:?}", elapsed_time.as_millis());
        println!(
            "Explored Nodes per second: {:.2}",
            self.num_nodes as f64 / elapsed_time.as_secs_f64()
        );

        best_move
    }

    pub fn search_opening_db(&self) -> Option<Board> {
        // Check if board.hash() exists in the opening_db
        let board_hash = self.board.hash();
        // let is_black: u8 = if (self.board.metadata >> 8) & 1 == 1 { 0 } else { 1 };
        // let curr_colour = if is_black==1 { "black" } else { "white" };
        // let opp_colour = if is_black==1 { "white" } else { "black" };
        if let Some(entry) = self.opening_db.get(&board_hash.to_string()) {
            println!("Found hash in opening database: {:?}", board_hash);

            // Parse the entry to extract values
            if let Value::Object(moves) = entry {
                let mut db_moves = Vec::new();
                let mut scores = Vec::new();
    
                for (move_str, stats) in moves {
                    if let Value::Object(stats_map) = stats {
                        // let win = stats_map.get(curr_colour).and_then(|v| v.as_f64()).unwrap_or(0.0);
                        // let lose = stats_map.get(opp_colour).and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let total = stats_map.get("total").and_then(|v| v.as_f64()).unwrap_or(1.0);
                        // let draw = total - ( win.abs() + lose.abs() );

                        // let mut score = ( win.abs() + draw.abs()/2.0 ) / total;
                        // if score == 0.0 { score += 0.01 }; // Even though bad, maybe good who knows?
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
                    let selected_move = uci_to_uint( choice );
                    for next_board in legal_moves {
                        if next_board.latest_move == selected_move {
                            return Some( next_board );
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
