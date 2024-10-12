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

    // Some Global Rules to take care of:
    // 
    // 1. A legal move should be discarded, if after making the move current king is under check!!
    // 2. Castling can be done only in the following cases
    //      a. King and the corresponding rook shouldn't have moved
    //      b. The king should not be in check
    //      c. The squares the king moves through during castling should not be in check
    //      d. There should be no pieces between the king and the corresponding rook
    // 3. En-Passant can only be done, `ONLY IMMIDEATELY` after the opponent moves double step pawn

    fn generate_pawn_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // Placeholder: implement logic to generate pawn moves based on color
        // Example of how you'd modify the state and push it into legal_boards

        // 1. Single step forward if unobstructing
        // 2. Double step forward if first move and un-obstructing
        // 3. Left diagonal capture, if opponent piece
        // 4. Right diagonal capture if opponent piece
        // 5. Promote to Queen on last file
        // 6. Promote to Rook on last file
        // 7. Promote to Knight on last file
        // 8. Promote to Bishop on last file
        // 9. Promote to Bishop on last file
        // 10. En-passant if conditions are right
    }

    fn generate_rook_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. Every Straight Up until EOB ( End of board ) or capture
        // 2. Every Straight Down until EOB ( End of board ) or capture
        // 3. Every Straight Right until EOB ( End of board ) or capture
        // 4. Every Straight Left until EOB ( End of board ) or capture
    }

    fn generate_knight_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. All 8 L shape moves around it ( Unless EOB ) including capture
    }

    fn generate_bishop_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. Every NE ( North-East ) diagonal until EOB or Capture
        // 2. Every SE ( South-East ) diagonal until EOB or Capture
        // 3. Every SW ( Sount-West ) diagonal until EOB or Capture
        // 4. Every NW ( North-West ) diagonal until EOB or Capture
    }

    fn generate_king_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. All 8 squares around the king except EOB
        //  Castling to the King-side
        //  Castling to the Queen-side
    }

    fn generate_queen_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. Every Straight Up until EOB ( End of board ) or capture
        // 2. Every Straight Down until EOB ( End of board ) or capture
        // 3. Every Straight Right until EOB ( End of board ) or capture
        // 4. Every Straight Left until EOB ( End of board ) or capture
        // 5. Every NE ( North-East ) diagonal until EOB or Capture
        // 6. Every SE ( South-East ) diagonal until EOB or Capture
        // 7. Every SW ( Sount-West ) diagonal until EOB or Capture
        // 8. Every NW ( North-West ) diagonal until EOB or Capture
    }

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
