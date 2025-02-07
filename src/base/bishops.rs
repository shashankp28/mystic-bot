use crate::base::defs::{ Board, PieceColour };

use super::defs::{ GlobalMap, LegalMoveVec };

impl Board {
    pub fn get_bishop_move_bit_map(&self, pos: i8, is_black: u8) -> u64 {
        let index = 63 - pos;
        // Get the bitmap for only the diagonal
        let basic_bishop_map = GlobalMap::diagonal_map(index as u8, 0);
        let curr_colour: PieceColour = match is_black {
            0 => PieceColour::White,
            1 => PieceColour::Black,
            _ => PieceColour::Any,
        };
        let mut all_piece_map = self.consolidated_piece_map(&PieceColour::Any);
        let curr_piece_map = self.consolidated_piece_map(&curr_colour);
        all_piece_map &= !(1 << pos); // Remove the current bishop

        all_piece_map &= basic_bishop_map; // Only get the pieces present on the diagonal
        let mut final_bishop_bitmap = GlobalMap::diagonal_map(index as u8, all_piece_map);

        final_bishop_bitmap &= !curr_piece_map; // Remove positions of current coloured piece
        return final_bishop_bitmap;
    }

    pub fn generate_bishop_moves(&self) -> LegalMoveVec {
        // TODO: Bishop Moves

        // 1. [ X ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 2. [ X ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 3. [ X ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 4. [ X ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
        // 5. [ X ] Take care to update castling bits if bishop captures opp. rook
        // 6. [ X ] Take care of updating per move tickers like white/block move, half clock, full number
        // 7. [ X ] Take care of removing En-passant on non-pawn move.
        let mut legal_boards = LegalMoveVec::new();
        let is_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 0 } else { 1 };
        let mut bishop_positions: u64 = (self.bishops >> (64 * is_black)) as u64;
        let opp_colour: PieceColour = match is_black {
            0 => PieceColour::Black,
            1 => PieceColour::White,
            _ => PieceColour::Any,
        };

        while bishop_positions != 0 {
            // Legal moves for 1 bishop
            let pos: i8 = bishop_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let mut final_dir_bitmap = self.get_bishop_move_bit_map(pos, is_black);

            while final_dir_bitmap != 0 {
                let new_pos = final_dir_bitmap.trailing_zeros() as u8;
                let new_index = (63 - new_pos) as u8;

                let mut new_board: Board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index as u8); // Remove current bishop position

                // If I removed opp. rook, I update their castling bits
                new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.bishops |= 1 << (64 * is_black + new_pos); // Update new bishop position

                // Update Tickers
                new_board.update_tickers(piece_removed, is_black == 1);
                new_board.set_enpassant(None);
                new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                new_board.latest_move &= (1 << 12) - 1;
                legal_boards.push(&mut new_board);
                final_dir_bitmap &= !(1 << new_pos); // Flip the bit map position to 0
            }
            bishop_positions &= !(1 << pos); // Flip the bishop position to 0
        }
        legal_boards
    }
}

#[cfg(test)]
mod tests {
    use crate::base::defs::{ Board, BoardHash, GlobalMap, LegalMoveVec };
    use std::collections::HashSet;
    use std::time::Instant;

    #[test]
    fn test_generate_bishop_moves() {
        GlobalMap::init();
        let file_path = "sample/test/bishops.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                let iterations = 1000000;
                let num_boards = 14;
                let start_time = Instant::now();
                for _ in 0..iterations {
                    legal_boards.clear();
                    legal_boards = board.generate_bishop_moves();
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
                    16959283578777538976, 11799924303268582201, 14631441144274027917, 14579762446948299746,
                    851657687795069721, 15474899271530768492, 11497984583919045780, 12634702892263133124,
                    14064973253066716430, 3103288480787892444, 17810970411952272829, 2516209962426219720,
                    13821391325419034468, 7282050253790480613,
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
