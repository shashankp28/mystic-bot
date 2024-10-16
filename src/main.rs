use mystic_bot::base::defs::Board;
use std::time::Instant;
// use std::fs;

fn generate_game_tree( curr_board: Board, max_depth: u32, num_nodes: &mut u64 ) {

    *num_nodes += 1;
    if max_depth == 0 {
        return;
    }
    for board in curr_board.get_legal_moves() {
        generate_game_tree(board, max_depth-1, num_nodes);
    }
}

fn main() {
    let file_path = "sample/default.json";
    let mut curr_board: Option<Board> = Option::None;
    match Board::from_file( file_path ) {
        Ok( board ) => {
            curr_board = Some( board );
            println!( "Successfully loaded board: {:?}", board );
            let legal_moves: Vec<Board> = board.get_legal_moves();
            for (i, new_board) in legal_moves.iter().enumerate() {
                let filename = format!("sample/{}.json", i);
                new_board.save_board(&filename);
            }
        }
        Err( e ) => {
            println!( "Error loading board: {}", e );
        }
    }

    if let Some(board) = curr_board {
        let max_depth = 3;
        let mut num_nodes: u64 = 0;

        let start_time = Instant::now();
        generate_game_tree(board, max_depth, &mut num_nodes);
        let duration = start_time.elapsed();
        let duration_secs = duration.as_secs_f64();

        println!("Number of Nodes Traversed: {}", num_nodes);
        println!("Time Taken: {:.2} seconds", duration_secs);
        println!("Nodes per second: {:.2}", num_nodes as f64 / duration_secs);
    } else {
        println!("Failed to load the board, exiting.");
    }
}
