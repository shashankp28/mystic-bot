use mystic_bot::base::defs::{Board, Search};
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mystic_bot < JSON File to Update >");
        return;
    }

    let file_path = &args[1];
    match Board::from_file(file_path) {
        Ok(board) => {
            let memory: HashMap<u64, f64> = HashMap::new();
            let mut search: Search = Search {
                board,
                memory,
                num_nodes: 0,
            };
            let next_board = search.best_next_board();

            if let Some(next) = next_board {
                next.save_board(file_path);
            }
        }
        Err(e) => {
            println!("Error loading board: {}", e);
        }
    }
}