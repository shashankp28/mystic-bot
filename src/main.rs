use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct Board {

    // Flattended Matrix representation of 8x8 Chess Board, with `a1` at the Top-Left
    // Bit is 1 if the corresponding piece is at corresponding index else 0
    // The white and black parts of the boards are concatenated in 64+64 = 128 bits
    // The MSB part corresponds to black and LSB part corresponds to white
    // The below representation based on
    // Video: https://www.youtube.com/watch?v=w4FFX_otR-4&pp=ygUSbWFraW5nIGEgY2hlc3MgYm90
    pub rooks: u128,
    pub knights: u128,
    pub bishops: u128,
    pub queens: u128,
    pub kings: u128,
    pub pawns: u128,

    // 1 bit, whether the board has an en-passant
    // It is not possible for a board to have multiple en-passants at the same time!
    // ( is_white_move, en_passant_warn, [ 3 bits en_passant_column  ],
    //   Black o-o, Black o-o-o, White o-o, White o-o-o )  --> 9 / 16 bits used
    pub metadata: u16,
}

impl Board {
    pub fn get_legal_moves(&self, is_white_turn: bool) -> Vec<Board> {
        let mut legal_boards = Vec::new();

        // Generate all possible legal moves
        self.generate_rook_moves(is_white_turn, &mut legal_boards);
        self.generate_knight_moves(is_white_turn, &mut legal_boards);
        self.generate_bishop_moves(is_white_turn, &mut legal_boards);
        self.generate_queen_moves(is_white_turn, &mut legal_boards);
        self.generate_king_moves(is_white_turn, &mut legal_boards);
        self.generate_pawn_moves(is_white_turn, &mut legal_boards);

        // Remove moves in which the king is in check
        self.prune_illegal_moves(is_white_turn, &mut legal_boards);
    
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

    fn prune_illegal_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // Remove moves in which the king is in check
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

    fn generate_king_moves(&self, is_white: bool, legal_boards: &mut Vec<Board>) {
        // 1. All 8 squares around the king except EOB or obstruction including capture
        //  Castling to the King-side
        //  Castling to the Queen-side
    }

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
        rooks: 2388925415139424862208,
        knights: 1222240910071333650432,
        bishops: 666676860038909263872,
        queens: 296300826683959672832,
        kings: 148150413341979836416,
        pawns: 1204203524907878590709760,
        metadata: 271
    };

    // Save the board to a file named "board.json"
    board.save_board("gen/rust_board.json");
    println!("Hello, world!");
}
