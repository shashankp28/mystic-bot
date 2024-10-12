use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct Board {

    // Flattended Matrix representation of 8x8 Chess Board, with `a1` at the Top-Left
    // Bit is 1 if the corresponding piece is at corresponding index else 0
    // The below representation based on
    // Video: https://www.youtube.com/watch?v=w4FFX_otR-4&pp=ygUSbWFraW5nIGEgY2hlc3MgYm90
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

    // ( 8 bits for each black pawn, ||ly 8 bits for each white pawn that moved double step )
    pub en_passant: u16,

    // ( X, X, BlackKingMoved, BlackQueenRookMoved, BlackKingRookMoved, ||ly 3 for White )
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
    // 4. Check is when the king is directly under threat
    // 5. Repeating a sequence of moves 3 times draws
    // 6. Checkmate is when king is under check and tehre are no legal moves (win/lose)
    // 7. Stalemate is when there are no legal moves, bu tthe king is not in check (draw)

    fn generate_pawn_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // Placeholder: implement logic to generate pawn moves based on color
        // Example of how you'd modify the state and push it into legal_boards

        // 1. Single step forward if unobstructing
        // 2. Double step forward if first move and un-obstructing
        // 3. Left single diagonal capture, if opponent piece
        // 4. Right single diagonal capture if opponent piece
        // 5. Promote to Queen if on last file
        // 6. Promote to Rook if on last file
        // 7. Promote to Knight if on last file
        // 8. Promote to Bishop if on last file
        // 9. En-passant if conditions are right
    }

    fn generate_rook_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. Every Straight Left until EOB ( End of board ) or capture or obstruction
    }

    fn generate_knight_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. All 8 L shape moves around it ( Unless EOB or obstruction ) including capture
    }

    fn generate_bishop_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 2. Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 3. Every SW ( Sount-West ) diagonal until EOB or Capture or obstruction
        // 4. Every NW ( North-West ) diagonal until EOB or Capture or obstruction
    }

    fn generate_king_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. All 8 squares around the king except EOB or obstruction including capture
        //  Castling to the King-side
        //  Castling to the Queen-side
    }

    fn generate_queen_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 6. Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 7. Every SW ( Sount-West ) diagonal until EOB or Capture or obstruction
        // 8. Every NW ( North-West ) diagonal until EOB or Capture or obstruction
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
        white_rooks: 129,
        white_knights: 66,
        white_bishops: 36,
        white_queens: 16,
        white_king: 8,
        white_pawns: 65280,
        black_rooks: 9295429630892703744,
        black_knights: 4755801206503243776,
        black_bishops: 2594073385365405696,
        black_queens: 1152921504606846976,
        black_king: 576460752303423488,
        black_pawns: 71776119061217280,
        en_passant: 0,
        castling_rights: 63
    };

    // Save the board to a file named "board.json"
    board.save_board("gen/rust_board.json");
    println!("Hello, world!");
}
