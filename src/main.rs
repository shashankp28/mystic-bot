use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{ Serialize, Deserialize };
use serde_json::to_writer_pretty;

#[derive( Debug, Serialize, Deserialize )]
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
    // ( [ X bits full move number ], [ 7 bits Half move clock ], is_white_move, en_passant_warn,
    //   [ 3 bits en_passant_column  ], Black o-o, Black o-o-o, White o-o, White o-o-o )
    //   --> 16 + fullmove_nuber / 32 bits used
    pub metadata: u32,
}

impl Board {

    pub fn get_legal_moves( &self ) -> Vec<Board> {
        let mut legal_boards = Vec::new();

        // Generate all possible legal moves
        self.generate_rook_moves( &mut legal_boards );
        self.generate_knight_moves( &mut legal_boards );
        self.generate_bishop_moves( &mut legal_boards );
        self.generate_queen_moves( &mut legal_boards );
        self.generate_king_moves( &mut legal_boards );
        self.generate_pawn_moves( &mut legal_boards );

        // Remove moves in which the king is in check
        self.prune_illegal_moves( &mut legal_boards );
    
        legal_boards
    }

    // TODO: Some Global Rules to take care of:
    // 
    // 1. [ ] A legal move should be discarded, if after making the move current king is under check!!
    // 2. [ ] Castling can be done only in the following cases
    //      a. [ ] King and the corresponding rook shouldn't have moved
    //      b. [ ] The king should not be in check
    //      c. [ ] The squares the king moves through during castling should not be in check
    //      d. [ ] There should be no pieces between the king and the corresponding rook
    // 3. [ ] En-Passant can only be done, `ONLY IMMEDIATELY` after the opponent moves double step pawn
    // 4. [ ] Check is when the king is directly under threat
    // 5. [ ] Repeating a sequence of moves 3 times draws
    // 6. [ ] Checkmate is when king is under check and there are no legal moves (win/lose)
    // 7. [ ] Stalemate is when there are no legal moves, but the king is not in check (draw)
    // 8. [ ] Keep track and update the Half Move Clock
    // 9. [ ] Keep track and update the Full Move Number

    fn generate_rook_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Rook Moves

        // 1. [ ] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [ ] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [ ] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [ ] Every Straight Left until EOB ( End of board ) or capture or obstruction
    }

    fn generate_knight_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Knight Moves
        // 1. [ ] All 8 L shape moves around it ( Unless EOB or obstruction ) including capture
    }

    fn generate_bishop_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Bishop Moves

        // 1. [ ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 2. [ ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 3. [ ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 4. [ ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
    }

    fn generate_queen_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Queen Moves

        // 1. [ ] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [ ] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [ ] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [ ] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [ ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 6. [ ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 7. [ ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 8. [ ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
    }

    fn generate_king_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: King Moves

        // 1. [ ] All 8 squares around the king except EOB or obstruction including capture
        // 2. [ ] Castling to the King-side
        // 3. [ ] Castling to the Queen-side
    }

    fn generate_pawn_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Pawn Moves

        // 1. [ ] Single step forward if unobstructing
        // 2. [ ] Double step forward if first move and unobstructing
        // 3. [ ] Left single diagonal capture, if opponent piece
        // 4. [ ] Right single diagonal capture if opponent piece
        // 5. [ ] Promote to Queen if on last file
        // 6. [ ] Promote to Rook if on last file
        // 7. [ ] Promote to Knight if on last file
        // 8. [ ] Promote to Bishop if on last file
        // 9. [ ] En-passant if conditions are right
    }

    fn prune_illegal_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Pin / Check analysis
        // 1. [ ] Remove moves in which the king is in check
    }

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let board: Board = serde_json::from_str(&contents)?;
        Ok(board)
    }

    pub fn save_board( &self, file_name: &str ) {
        let file = File::create( file_name ).expect( "Unable to create file" );
        match to_writer_pretty( &file, &self ) {
            Ok(_) => {
                println!( "Board saved successfully to {}", file_name );
            }
            Err(e) => {
                println!( "Error serializing board: {}", e );
            }
        }
    }
}

fn main() {
    let file_path = "sample/start.json";
    match Board::from_file( file_path ) {
        Ok( board ) => {
            println!( "Successfully loaded board: {:?}", board );
            board.save_board( "sample/rust_board.json" );
        }
        Err( e ) => {
            println!( "Error loading board: {}", e );
        }
    }
    println!( "Hello, world!" );
}
