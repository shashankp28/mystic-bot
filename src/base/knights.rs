use crate::base::defs::{Board, PieceColour};

use super::defs::LegalMoveVec;

impl Board {

    pub fn generate_knight_moves( &self, legal_boards: &mut LegalMoveVec ) {
        // TODO: Knight Moves
        // 1. [ X ] All 8 L shape moves around it ( Unless EOB or obstruction ) including capture
        // 2. [ X ] Take care to update castling bits if knight captures opp. rook
        // 3. [ X ] Take care of removing En-passant on non-pawn move.
        let basic_knight_map: u64 = 21617444997;  // Through Experimentation
        let left_half_board_map: u64 = 17361641481138401520;  // Through Experimentation
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };
        let mut knight_positions: u64 = ( self.knights >> 64*is_black ) as u64;
        let curr_colour: PieceColour = match is_black {
            1 => PieceColour::Black,
            0 => PieceColour::White,
            _ => PieceColour::Any,
        };
        let same_piece_map = self.consolidated_piece_map( &curr_colour );
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
            new_knight_map &= !same_piece_map;
            
            while new_knight_map != 0 {
                // Update the legal move in the vector
                let new_pos: u8 = new_knight_map.trailing_zeros() as u8;
                let new_index: u8 = ( 63 - new_pos ) as u8;

                let mut new_board: Board = self.clone(); // Clone the board to modify it
                new_board.remove_piece( index ); // Remove current knight position

                // If I removed opp. rook, I update their castling bits
                let opp_colour: PieceColour = match is_black {
                    1 => PieceColour::White,
                    0 => PieceColour::Black,
                    _ => PieceColour::Any,
                };
                new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                let piece_removed = new_board.remove_piece( new_index ); // Remove existing piece ( for capture )
                new_board.knights |= 1 << 64*is_black+new_pos; // Update new knight position

                // Update Half & Full move clocks & toggle black / white move
                new_board.update_tickers( piece_removed, is_black==1 );
                new_board.set_enpassant( None );
                new_board.latest_move = ( (index as u16) << 6 | new_index as u16) as u16;
                new_board.latest_move &= ( 1 << 12 ) - 1;
                legal_boards.push(&mut new_board);

                new_knight_map &= !( 1 << new_pos ); // Flip the knight position to 0 
            }

            knight_positions &= !( 1 << pos ); // Flip the knight position to 0 
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::base::defs::{Board, BoardHash, LegalMoveVec};
    use std::collections::HashSet;

    #[test]
    fn test_generate_knight_moves() {
        let file_path = "sample/test/knights.json";
        match Board::from_file( file_path ) {
            Ok( board ) => {
                println!( "Successfully loaded board: {:?}", board );
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                board.generate_knight_moves( &mut legal_boards );
                assert_eq!(legal_boards.len(), 12, "Expected 12 legal moves, but got {}", legal_boards.len());

                let mut board_hashes: HashSet<BoardHash> = HashSet::new();
                let hashes = [
                    5820322162005059073,
                    7248332367856294888,
                    3343382540968063460,
                    13734315367842045572,
                    15671716411796014867,
                    14442241140689718116,
                    2629788050489721178,
                    5902975122334121358,
                    3148796746828343536,
                    6017073384496795541,
                    8083710378863667238,
                    2302243076185838823,
                ];
                for &hash in &hashes {
                    board_hashes.insert(hash);
                }
                let mut actual_board_hashes: HashSet<BoardHash> = HashSet::new();
                for board in legal_boards {
                    let board_hash = board.hash();
                    actual_board_hashes.insert(board_hash);
                    assert!(
                        board_hashes.contains(&board_hash),
                        "Generated board hash {} not found in the predefined hashes.",
                        board_hash
                    );
                }

                for &hash in &hashes {
                    assert!(
                        actual_board_hashes.contains(&hash),
                        "Predefined board hash {} not found in the generated hashes.",
                        hash
                    );
                }

            }
            Err( e ) => {
                println!( "Error loading board: {}", e );
            }
        }
    }
}