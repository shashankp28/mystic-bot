use mystic_bot::base::defs::{Board, Search};
use std::collections::HashMap;

fn main() {
    let file_path: &str = "sample/position.json";
    match Board::from_file( file_path ) {
        Ok( board ) => {
            let memory: HashMap<u64, f64> = HashMap::new();
            let search: Search = Search {
                board,
                memory,
            };
            let next_board = search.best_next_board();

            next_board.save_board("./sample/position.json");
        }
        Err( e ) => {
            println!( "Error loading board: {}", e );
        }
    }
}
