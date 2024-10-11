use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct Board {
    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub white_pawns: u64,

    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queens: u64,
    pub black_king: u64,
    pub black_pawns: u64,

    pub en_passant: u16,
    pub castling_rights: u8,
}

impl Board {
    pub fn get_legal_moves(&self, is_white_turn: bool) -> Vec<Board> {
        let mut legal_boards = Vec::new();

        // Example: Generate all possible pawn moves for the current player
        if is_white_turn {
            self.generate_pawn_moves(true, &mut legal_boards);
        } else {
            self.generate_pawn_moves(false, &mut legal_boards);
        }

        // Add similar calls for other pieces (rooks, knights, bishops, etc.)
        // self.generate_rook_moves(is_white_turn, &mut legal_boards);
        // self.generate_knight_moves(is_white_turn, &mut legal_boards);
        // ...

        // Return the list of legal boards
        legal_boards
    }

    fn generate_pawn_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // Placeholder: implement logic to generate pawn moves based on color
        // Example of how you'd modify the state and push it into legal_boards
    }

    // Additional helper methods for generating moves for other pieces (rooks, knights, bishops, etc.)
    // fn generate_rook_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) { ... }
    // fn generate_knight_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) { ... }
    // fn generate_bishop_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) { ... }
    // fn generate_castling_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) { ... }

    pub fn save_board(&self, file_name: &str) {
        // Serialize the Board struct into JSON format
        match serde_json::to_string(&self) {
            Ok(json) => {
                // Open a file in write mode
                let mut file = File::create(file_name).expect("Unable to create file");
                // Write the JSON data to the file
                file.write_all(json.as_bytes()).expect("Unable to write data");
            }
            Err(e) => {
                println!("Error serializing board: {}", e);
            }
        }
    }
}

fn main() {
    let board = Board {
        white_rooks: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_king: 0,
        white_pawns: 0,
        black_rooks: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_king: 0,
        black_pawns: 0,
        en_passant: 0,
        castling_rights: 0,
    };

    // Save the board to a file named "board.json"
    board.save_board("board.json");
    println!("Hello, world!");
}
