use crate::base::defs::{Board, PieceColour};

impl Board {
    pub fn generate_bishop_moves(&self, legal_boards: &mut Vec<Board>) {
        // TODO: Bishop Moves

        // 1. [ X ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 2. [ X ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 3. [ X ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 4. [ X ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
        // 5. [ X ] Take care to update castling bits if bishop captures opp. rook
        // 6. [ X ] Take care of updating per move tickers like white/block move, half clock, full number
        // 7. [X] Take care of removing En-passant on non-pawn move.

        let is_black: u8 = if (self.metadata >> 8) & 1 == 1 { 0 } else { 1 };

        let mut bishop_positions: u64 = (self.bishops >> 64 * is_black) as u64;

        while bishop_positions != 0 {
            // Legal moves for 1 bishop
            let pos: i8 = bishop_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = index % 8;
            let y = index / 8;

            let directions: [[i8; 2]; 4] = [[1, 1], [1, -1], [-1, 1], [-1, -1]];

            for [delta_x, delta_y] in &directions {
                let mut new_x = x + delta_x;
                let mut new_y = y + delta_y;
                loop {
                    if new_x < 0 || new_x > 7 || new_y < 0 || new_y > 7 {
                        break;
                    }
                    let new_index = (new_x + new_y * 8) as u8;
                    let new_pos = (63 - new_index) as u8;
                    let curr_colour: PieceColour = match is_black {
                        1 => PieceColour::Black,
                        0 => PieceColour::White,
                        _ => PieceColour::Any,
                    };
                    let current_piece_map = self.consolidated_piece_map(&curr_colour);

                    // Break if reached a current coloured piece
                    if current_piece_map & (1 << new_pos) != 0 {
                        break;
                    }

                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(index as u8); // Remove current bishop position

                    // If I removed opp. rook, I update their castling bits
                    let opp_colour: PieceColour = match is_black {
                        0 => PieceColour::Black,
                        1 => PieceColour::White,
                        _ => PieceColour::Any,
                    };
                    new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                    let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )

                    new_board.bishops |= 1 << 64 * is_black + new_pos; // Update new bishop position

                    // Update Tickers
                    new_board.update_tickers(piece_removed, is_black == 1);
                    new_board.unmark_enpassant();
                    legal_boards.push(new_board);
                    // Break if we had reached an opposite coloured piece
                    if piece_removed {
                        break;
                    }

                    new_x += delta_x;
                    new_y += delta_y;
                }
            }
            bishop_positions &= !(1 << pos); // Flip the bishop position to 0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::defs::Board;
    use std::collections::HashSet;

    #[test]
    fn test_generate_bishop_moves() {
        let file_path = "sample/test/bishops.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: Vec<Board> = Vec::new();
                board.generate_bishop_moves(&mut legal_boards);
                assert_eq!(
                    legal_boards.len(),
                    14,
                    "Expected 14 legal moves, but got {}",
                    legal_boards.len()
                );

                let mut board_hashes: HashSet<u32> = HashSet::new();
                let hashes = [
                    2101630820, 515292360, 1701416084, 880608653, 3354513348, 3622761442,
                    2087286030, 493385149, 2384892320, 3733645420, 1827440860, 3048835301,
                    2563937081, 2671866649,
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
                        board,
                    );
                }
            }
            Err(e) => {
                println!("Error loading board: {}", e);
            }
        }
    }
}
