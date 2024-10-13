use mystic_bot::base::defs::Board;

fn main() {
    let file_path = "sample/start.json";
    match Board::from_file( file_path ) {
        Ok( mut board ) => {
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
    println!( "Hello, world!" );
}
