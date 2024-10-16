use crate::base::defs::Board;
use std::fs;

pub fn generate_hash_for_boards() -> std::io::Result<()> {
    let path = "./sample/test/king_test/";

    for entry in fs::read_dir(path)? {
        match entry {
            Ok(file_path) => match Board::from_file(file_path.path()) {
                Ok(board) => {
                    let hash = board.hash();
                    println!("{hash},")
                }
                Err(e) => {
                    eprintln!("Error loading the board {}", e)
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}

pub fn generate_game_tree( curr_board: Board, max_depth: u32, num_nodes: &mut u64 ) {
    *num_nodes += 1;
    if max_depth == 0 {
        return;
    }
    for board in curr_board.get_legal_moves() {
        generate_game_tree(board, max_depth-1, num_nodes);
    }
}
