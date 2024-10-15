use crate::base::defs::Board;
use std::fs;

pub fn generate_hash_for_boards() -> std::io::Result<()> {
    let path = "./sample/test/rook_test/";

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
