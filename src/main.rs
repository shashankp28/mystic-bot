use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{ Serialize, Deserialize };
use serde_json::to_writer_pretty;

// Debugging Purpose Start

// fn sum_bits(num: u64) -> u64 {
//     let mut count = 0;
//     let mut n = num;

//     while n > 0 {
//         count += n & 1; // Add the least significant bit
//         n >>= 1;        // Shift right to process the next bit
//     }

//     count
// }

// Debugging Purpose End

enum PieceColour {
    Black,
    White,
    Any,
}

#[derive( Copy, Clone, Debug, Serialize, Deserialize )]
pub struct Board {

    // Flattended Matrix representation of 8x8 Chess Board, with `a1` at the Top-Left
    // Bit is 1 if the corresponding piece is at corresponding index else 0
    // The black and white parts of the boards are concatenated in 64+64 = 128 bits
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

    // Uility Functions Start
    fn consolidated_piece_map( &self, colour: PieceColour ) -> u64 {
        let all_piece_map: u128 = self.rooks | self.knights | self.bishops | self.queens | self.kings | self.pawns;
        match colour {
            PieceColour::Black => ( all_piece_map >> 64 ) as u64,
            PieceColour::White => all_piece_map as u64,
            PieceColour::Any => {
                ( all_piece_map >> 64 ) as u64 | all_piece_map as u64
            },
        }
    }

    fn remove_piece( &mut self, index: u8 ) {
        // Remove piece from bitMap if any piece exists at that index,
        // The logic of colour / legality of the move must be taken care
        // from the caller's side
        let mut removal_map: u128 = 0;
        removal_map |= ( 1 << ( 63-index ) ) | ( 1 << ( 127-index ) );
        removal_map = !removal_map;
        self.rooks &= removal_map;
        self.knights &= removal_map;
        self.bishops &= removal_map;
        self.queens &= removal_map;
        // self.kings &= removal_map; IF KING SHOULD BE REMOVED, SOMETHING IS WRONG!!
        self.pawns &= removal_map;
    }
    // Utility Functions End

    pub fn get_legal_moves( &mut self ) -> Vec<Board> {
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
        // 5. [ ] Take care to update the castling bits ( King or Queenside ) on first rook move
        println!( "Number of Legal Moves after Rook: {}", legal_boards.len() );
    }

    fn generate_knight_moves( &mut self, legal_boards: &mut Vec<Board> ) {
        // TODO: Knight Moves
        // 1. [ X ] All 8 L shape moves around it ( Unless EOB or obstruction ) including capture
        println!( "Generating Knight Moves..." );
        let basic_knight_map: u64 = 21617444997;  // Through Experimentation
        let left_half_board_map: u64 = 17361641481138401520;  // Through Experimentation
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };
        let mut knight_positions: u64 = ( self.knights >> 64*is_black ) as u64;
        println!( "Found Current Knight Positions.." );
        while knight_positions != 0 {
            // Legal moves for 1 knight
            let pos: u8 = knight_positions.trailing_zeros() as u8;
            let index: u8 = ( 63 - pos ) as u8;
            let mut new_knight_map: u64 = 0;

            // Through Experimentation
            if index + 17 <= 63 {
                new_knight_map |= basic_knight_map << ( 63 - ( index + 17 ) );
            } else {
                new_knight_map |= basic_knight_map >> ( ( index + 17 ) - 63 )
            }
            if index%8 < 2 {
                new_knight_map &= left_half_board_map
            }
            if index%8 > 5 {
                new_knight_map &= !left_half_board_map
            }

            // Remove all bits where the knight jumps on same coloured piece
            let curr_colour: PieceColour = match is_black {
                1 => PieceColour::Black,
                0 => PieceColour::White,
                _ => PieceColour::Any,
            };
            new_knight_map &= !self.consolidated_piece_map( curr_colour );
            
            while new_knight_map != 0 {
                // Update the legal move in the vector
                let new_pos: u8 = new_knight_map.trailing_zeros() as u8;
                let new_index: u8 = ( 63 - new_pos ) as u8;

                let mut new_board = self.clone(); // Clone the board to modify it
                new_board.remove_piece( index ); // Remove current knight position
                new_board.remove_piece( new_index ); // Remove existing piece ( for capture )
                new_board.knights |= 1 << 64*is_black+new_pos; // Update new knight position
                legal_boards.push( new_board );

                new_knight_map &= !( 1 << new_pos ); // Flip the knight position to 0 
            }

            knight_positions &= !( 1 << pos ); // Flip the knight position to 0 
        }
    }

    fn generate_bishop_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Bishop Moves

        // 1. [ ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 2. [ ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 3. [ ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 4. [ ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
        println!( "Number of Legal Moves after Bishop: {}", legal_boards.len() );
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
        println!( "Number of Legal Moves after Queen: {}", legal_boards.len() );
    }

    fn generate_king_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: King Moves

        // 1. [ ] All 8 squares around the king except EOB or obstruction including capture
        // 2. [ ] Castling to the King-side
        // 3. [ ] Castling to the Queen-side
        // 4. [ ] Take care to update the castling bits ( King and Queenside ) on first king move
        println!( "Number of Legal Moves after King: {}", legal_boards.len() );
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
        println!( "Number of Legal Moves after Pawn: {}", legal_boards.len() );
    }

    fn prune_illegal_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Pin / Check analysis
        // 1. [ ] Remove moves in which the king is in check
        println!( "Number of Legal Moves after Pruning Illegal: {}", legal_boards.len() );
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
