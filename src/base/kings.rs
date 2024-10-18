use crate::base::defs::{Board, CastleSide, PieceColour};

use super::defs::LegalMoveVec;

impl Board {
    pub fn generate_king_moves(&self, legal_boards: &mut LegalMoveVec) {
        // TODO: King Moves

        // 1. [ X ] All 8 squares around the king except EOB or obstruction including capture
        // 2. [ X ] Castling to the King-side
        // 3. [ X ] Castling to the Queen-side
        // 4. [ X ] Take care to update the castling bits ( King and Queenside ) on first king move
        // 5. [ X ] Take care to update castling bits if king captures opp. rook
        // 6. [ X ] Take care of updating per move tickers like white/block move, half clock, full number
        // 7. [ X ] Take care of removing En-passant on non-pawn move.
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };
        let king_positions: u64 = ( self.kings >> 64*is_black ) as u64;
        let pos: i8 = king_positions.trailing_zeros() as i8;
        let index: i8 = ( 63 - pos ) as i8;
        let x = index % 8;
        let y = index / 8;
        let curr_colour: PieceColour = match is_black {
            1 => PieceColour::Black,
            0 => PieceColour::White,
            _ => PieceColour::Any,
        };
        let opp_colour: PieceColour = match is_black {
            0 => PieceColour::Black,
            1 => PieceColour::White,
            _ => PieceColour::Any,
        };
        let directions: [[i8; 2]; 8] = [ [ 1, 1 ], [ 1, -1 ], [ -1, 1 ], [ -1, -1 ],
                                         [ 1, 0 ], [ 0, 1 ], [ -1, 0 ], [ 0, -1 ], ];

        // Standard King Moves
        for [ delta_x, delta_y ] in directions {
            let new_x = x+delta_x;
            let new_y = y+delta_y;

            if new_x < 0 || new_x > 7 || new_y < 0 || new_y > 7 {
                continue;
            }
            let new_index = (new_x + new_y * 8) as u8;
            let new_pos = (63 - new_index) as u8;

            // Break if hit a current coloured piece
            let current_piece_map = self.consolidated_piece_map( &curr_colour );
            if current_piece_map & ( 1 << new_pos ) != 0 {
                continue;
            }

            let mut new_board: Board = self.clone(); // Clone the board to modify it
            new_board.remove_piece(index as u8); // Remove current king position

            // If I removed opp. rook, I update their castling bits
            new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

            let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )

            new_board.kings |= 1 << 64 * is_black + new_pos; // Update new king position

            // Update Tickers
            new_board.update_tickers(piece_removed, is_black == 1);
            new_board.set_enpassant( None );

            // Remove castling bits as King is moved
            new_board.remove_castling_bits(CastleSide::King, &curr_colour);
            new_board.remove_castling_bits(CastleSide::Queen, &curr_colour);
            legal_boards.push(&mut new_board);
        }

        // King side castling
        // Check Castling bits
        if ( self.metadata >> ( 1+2*is_black ) ) & 1 != 0 {
            // I need not check whether king and rook are present,
            // Hopefully the code would have flipped the castling bits
            // if the king or the rook moved
            // Check for obstructions
            let all_piece_map = self.consolidated_piece_map(&PieceColour::Any);
            if ( all_piece_map >> 57-56*is_black & 3 ) == 0 {
                // Check no squres in between are under threat
                let target = if is_black==1 { [60, 61, 62] } else { [4, 5, 6] };
                let mut can_castle: bool = true;
                for index in target {
                    if self.can_attack(1-is_black, index) {
                        can_castle = false;
                        break;
                    }
                }
                if can_castle {
                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(target[0]); // Remove current king position
                    new_board.remove_piece(target[0]+3); // Remove kingside rook position

                    new_board.kings |= 1 << 64 * is_black + (61-target[0]); // Update new king position
                    new_board.rooks |= 1 << 64 * is_black + (62-target[0]); // Update new kingside rook position
                    
                    // Update Tickers
                    new_board.update_tickers(false, is_black == 1);
                    new_board.set_enpassant( None );

                    // Remove castling bits as King is moved
                    new_board.remove_castling_bits(CastleSide::King, &curr_colour);
                    new_board.remove_castling_bits(CastleSide::Queen, &curr_colour);
                    legal_boards.push(&mut new_board);
                }
            }

        }

        // Queen side castling
        // Check Castling bits
        if ( self.metadata >> ( 2*is_black ) ) & 1 != 0 {
            // I need not check whether king and rook are present,
            // Hopefully the code would have flipped the castling bits
            // if the king or the rook moved
            // Check for obstructions
            let all_piece_map = self.consolidated_piece_map(&PieceColour::Any);
            if ( all_piece_map >> 60-56*is_black & 7 ) == 0 {
                // Check no squres in between are under threat
                let target = if is_black==1 { [60, 59, 58] } else { [4, 3, 2] };
                let mut can_castle: bool = true;
                for index in target {
                    if self.can_attack(1-is_black, index) {
                        can_castle = false;
                        break;
                    }
                }
                if can_castle {
                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(target[0]); // Remove current king position
                    new_board.remove_piece(target[0]-4); // Remove kingside rook position

                    new_board.kings |= 1 << 64 * is_black + (65-target[0]); // Update new king position
                    new_board.rooks |= 1 << 64 * is_black + (64-target[0]); // Update new kingside rook position
                    
                    // Update Tickers
                    new_board.update_tickers(false, is_black == 1);
                    new_board.set_enpassant( None );

                    // Remove castling bits as King is moved
                    new_board.remove_castling_bits(CastleSide::King, &curr_colour);
                    new_board.remove_castling_bits(CastleSide::Queen, &curr_colour);
                    legal_boards.push(&mut new_board);
                }
            }

        }
    }

}

#[cfg(test)]
mod tests {
    use crate::base::defs::{Board, BoardHash, LegalMoveVec};
    use std::collections::HashSet;

    #[test]
    fn test_generate_king_moves() {
        let file_path = "sample/test/kings.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                board.generate_king_moves(&mut legal_boards);
                assert_eq!(legal_boards.len(), 6, "Expected 6 legal moves, but got {}", legal_boards.len());

                let mut board_hashes: HashSet<u64> = HashSet::new();
                let hashes = [
                    518375765602632387,
                    902887513263151204,
                    11268089721562349499,
                    11500744273320731964,
                    6918353593318135188,
                    7168607394553651974,
                ];
                for &hash in &hashes {
                    board_hashes.insert(hash);
                }
                let mut actual_board_hashes: HashSet<u64> = HashSet::new();
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
            Err(e) => {
                println!("Error loading board: {}", e);
            }
        }
    }

    #[test]
    fn test_generate_king_moves_2() {
        let file_path = "sample/test/kings2.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards:  LegalMoveVec = LegalMoveVec::new();
                board.generate_king_moves(&mut legal_boards);
                println!( "{:?}", legal_boards );
                assert_eq!(legal_boards.len(), 3, "Expected 3 legal moves, but got {}", legal_boards.len());

                let mut board_hashes: HashSet<BoardHash> = HashSet::new();
                let hashes = [
                    15388847677281830013,
                    2479904804525528622,
                    3108398517801993946,
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
            Err(e) => {
                println!("Error loading board: {}", e);
            }
        }
    }
}
