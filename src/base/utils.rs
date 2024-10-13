use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::to_writer_pretty;
use crate::base::defs::Board;
use crate::base::defs::PieceColour;

impl Board {

    pub fn consolidated_piece_map( &self, colour: PieceColour ) -> u64 {
        let all_piece_map: u128 = self.rooks | self.knights | self.bishops | self.queens | self.kings | self.pawns;
        match colour {
            PieceColour::Black => ( all_piece_map >> 64 ) as u64,
            PieceColour::White => all_piece_map as u64,
            PieceColour::Any => {
                ( all_piece_map >> 64 ) as u64 | all_piece_map as u64
            },
        }
    }

    pub fn remove_piece( &mut self, index: u8 ) {
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

    pub fn get_legal_moves( &mut self ) -> Vec<Board> {
        let mut legal_boards = Vec::new();

        // Generate all possible legal moves
        self.generate_rook_moves( &mut legal_boards );
        self.generate_knight_moves( &mut legal_boards );
        self.generate_bishop_moves( &mut legal_boards );
        self.generate_queen_moves( &mut legal_boards );
        self.generate_pawn_moves( &mut legal_boards );
        self.generate_king_moves( &mut legal_boards );

        // Remove moves in which the king is in check
        self.prune_illegal_moves( &mut legal_boards );
    
        legal_boards
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