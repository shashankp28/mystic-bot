use crate::base::defs::{ Board, PieceColour };

use super::defs::{ LegalMoveVec, PieceType };

impl Board {
    pub fn generate_rook_moves(&self, legal_boards: &mut LegalMoveVec) {
        // TODO: Rook Moves

        // 1. [X] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [X] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [X] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [X] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [X] Take care to update the castling bits ( King or Queenside ) on first rook move
        // 6. [X] Take care to update castling bits if rook captures opp. rook
        // 7. [X] Take care of updating per move tickers like white/block move, half clock, full number
        // 8. [X] Take care of removing En-passant on non-pawn move.

        let is_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 0 } else { 1 };
        let mut rook_positions: u64 = (self.rooks >> (64 * is_black)) as u64;
        let opp_colour: PieceColour = match is_black {
            0 => PieceColour::Black,
            1 => PieceColour::White,
            _ => PieceColour::Any,
        };
        let curr_colour: PieceColour = match is_black {
            1 => PieceColour::Black,
            0 => PieceColour::White,
            _ => PieceColour::Any,
        };

        while rook_positions != 0 {
            // Legal moves for 1 rook
            let pos: i8 = rook_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let mut final_dir_bitmap = self.get_directional_bit_map(pos, PieceType::Rook);

            while final_dir_bitmap != 0 {
                let new_pos = final_dir_bitmap.trailing_zeros() as u8;
                let new_index = (63 - new_pos) as u8;

                let mut new_board: Board = self.clone(); // Clone the board to modify it

                new_board.remove_castling_for_rook(&curr_colour, index as u64); // remove castling for the rook since we are moving it.
                new_board.remove_piece(index as u8); // Remove current rook position
                // If I removed opp. rook, I update their castling bits
                new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.rooks |= 1 << (64 * is_black + new_pos); // Update new rook position

                // Update Tickers
                new_board.update_tickers(piece_removed, is_black == 1);
                new_board.set_enpassant(None);
                new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                new_board.latest_move &= (1 << 12) - 1;
                legal_boards.push(&mut new_board);
                final_dir_bitmap &= !(1 << new_pos); // Flip the bit map position to 0
            }
            rook_positions &= !(1 << pos); // Flip the rook position to 0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::defs::{ Board, BoardHash, LegalMoveVec };
    use std::collections::HashSet;
    use std::time::Instant;

    #[test]
    fn test_generate_rook_moves() {
        let file_path = "sample/test/rooks.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                let iterations = 1000000;
                let num_boards = 10;
                let start_time = Instant::now();
                for _ in 0..iterations {
                    legal_boards.clear();
                    board.generate_rook_moves(&mut legal_boards);
                }
                let elapsed_time_ns = start_time.elapsed().as_micros() * 1000;
                let average_time_per_iteration = (elapsed_time_ns as f64) / (iterations as f64);
                println!(
                    "Average time per move generation over {} moves: {:.2} ns",
                    iterations * num_boards,
                    average_time_per_iteration / (num_boards as f64)
                );
                assert_eq!(
                    legal_boards.len(),
                    num_boards,
                    "Expected {} legal moves, but got {}",
                    num_boards,
                    legal_boards.len()
                );

                let mut board_hashes: HashSet<BoardHash> = HashSet::new();
                let hashes = [
                    7091049467665180278, 647243355553057862, 13314191749323641045, 17145172113589615753,
                    6321615291756588204, 5841539080194952656, 13567278288467709581, 316115115972194632,
                    10565316067822619270, 15896617303301213466,
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
