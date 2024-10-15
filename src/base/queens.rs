use crate::base::defs::{Board, PieceColour};

impl Board {

    pub fn generate_queen_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Queen Moves

        // 1. [ X ] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [ X ] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [ X ] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [ X ] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [ X ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 6. [ X ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 7. [ X ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 8. [ X ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
        // 9. [ X ] Take care to update castling bits if queen captures opp. rook
        // 10. [ X ] Take care of updating per move tickers like white/block move, half clock, full number
        // 11. [X] Take care of removing En-passant on non-pawn move.
        
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };

        let mut queen_positions: u64 = ( self.queens >> 64*is_black ) as u64;

        while queen_positions != 0 {
            // Legal moves for 1 queen
            let pos: i8 = queen_positions.trailing_zeros() as i8;
            let index: i8 = ( 63 - pos ) as i8;
            let x = index % 8;
            let y = index / 8;

            let directions: [[i8; 2]; 8] = [ [ 1, 1 ], [ 1, -1 ], [ -1, 1 ], [ -1, -1 ],
                                             [ 1, 0 ], [ 0, 1 ], [ -1, 0 ], [ 0, -1 ], ];

            for [delta_x, delta_y] in &directions {
                let mut new_x = x + delta_x;
                let mut new_y = y + delta_y;
                loop {
                    if new_x < 0 || new_x > 7 || new_y < 0 || new_y > 7 {
                        break;
                    }
                    let new_index = (new_x + new_y*8) as u8;
                    let new_pos = (63-new_index) as u8;
                    let curr_colour: PieceColour = match is_black {
                        1 => PieceColour::Black,
                        0 => PieceColour::White,
                        _ => PieceColour::Any,
                    };
                    let current_piece_map = self.consolidated_piece_map( &curr_colour );
                    
                    // Break if reached a current coloured piece
                    if current_piece_map & ( 1 << new_pos ) != 0 {
                        break;
                    }

                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece( index as u8 ); // Remove current queen position

                    // If I removed opp. rook, I update their castling bits
                    let opp_colour: PieceColour = match is_black {
                        0 => PieceColour::Black,
                        1 => PieceColour::White,
                        _ => PieceColour::Any,
                    };
                    new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                    let piece_removed = new_board.remove_piece( new_index ); // Remove existing piece ( for capture )
    
                    new_board.queens |= 1 << 64*is_black+new_pos; // Update new queen position

                    // Update Tickers
                    new_board.update_tickers( piece_removed, is_black==1 );
                    new_board.unmark_enpassant();
                    legal_boards.push( new_board );
                    // Break if we had reached an opposite coloured piece
                    if piece_removed {
                        break;
                    }

                    new_x += delta_x;
                    new_y += delta_y;
                }
            }
            queen_positions &= !( 1 << pos ); // Flip the queen position to 0 
        }
    }

}



#[cfg(test)]
mod tests {
    use crate::base::defs::Board;
    use std::collections::HashSet;

    #[test]
    fn test_generate_queen_moves() {
        let file_path = "sample/test/queens.json";
        match Board::from_file( file_path ) {
            Ok( board ) => {
                println!( "Successfully loaded board: {:?}", board );
                let mut legal_boards: Vec<Board> = Vec::new();
                board.generate_queen_moves( &mut legal_boards );
                assert_eq!(legal_boards.len(), 44, "Expected 44 legal moves, but got {}", legal_boards.len());

                let mut board_hashes: HashSet<u32> = HashSet::new();
                let hashes = [
                    1481746532,
                    2028220356,
                    1733110260,
                    3880039606,
                    590834368,
                    1340525763,
                    2930087644,
                    314367264,
                    3640646963,
                    4036555365,
                    3391048210,
                    1119545477,
                    3483815918,
                    2230492272,
                    4247970404,
                    3242191527,
                    2990893449,
                    969114333,
                    3681765855,
                    3021947795,
                    2896242605,
                    2025122889,
                    2423627468,
                    2959876819,
                    2208737318,
                    3965795674,
                    639665738,
                    1174105213,
                    4147033737,
                    33113689,
                    3352388842,
                    2282576045,
                    3855212428,
                    508652314,
                    1119136397,
                    3380708621,
                    3037908451,
                    420563079,
                    3531276333,
                    2595101837,
                    588429608,
                    168208811,
                    2031796871,
                    930302213,
                ];
                for &hash in &hashes {
                    board_hashes.insert(hash);
                }
                for board in &legal_boards {
                    let board_hash = board.hash();
                    assert!(
                        board_hashes.contains(&board_hash),
                        "Generated board hash {} for {:?} not found in the predefined hashes.",
                        board_hash,
                        board
                    );
                }

            }
            Err( e ) => {
                println!( "Error loading board: {}", e );
            }
        }
    }
}