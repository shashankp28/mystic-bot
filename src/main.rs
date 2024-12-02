use mystic_bot::base::defs::{Board, Search};
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mystic_bot <JSON File to Update> [Time Limit in ms]");
        return;
    }

    let file_path = &args[1];

    // Optional time limit in milliseconds
    let time_limit = if args.len() > 2 {
        args[2].parse::<u64>().ok().map(Duration::from_millis)
    } else {
        Some(Duration::from_secs(5)) // Default to 5 seconds
    }.unwrap();

    match Board::from_file(file_path) {
        Ok(board) => {
            let memory: HashMap<u64, f64> = HashMap::new();
            let mut search: Search = Search {
                board,
                memory,
                num_nodes: 0,
            };

            // Start timing if a limit is provided
            let start_time = Instant::now();

            // Compute the best next board, respecting the time limit
            let next_board = search.best_next_board(time_limit, &start_time);

            if let Some(next) = next_board {
                next.save_board(file_path);
            }
        }
        Err(e) => {
            println!("Error loading board: {}", e);
        }
    }
}
