use crate::base::defs::{Board, PieceColour};

impl Board {
    pub fn generate_rook_moves(&self, legal_boards: &mut Vec<Board>) {
        // TODO: Rook Moves

        // 1. [X] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [X] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [X] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [X] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [X] Take care to update the castling bits ( King or Queenside ) on first rook move
        // 6. [X] Take care to update castling bits if rook captures opp. rook
        // 7. [X] Take care of updating per move tickers like white/block move, half clock, full number
        println!("Generating Rook Moves...");
        let is_black: u8 = if (self.metadata >> 8) & 1 == 1 { 0 } else { 1 };

        let mut rook_positions: u64 = (self.rooks >> 64 * is_black) as u64;

        println!("Found Current Rook Positions..");
        while rook_positions != 0 {
            // Legal moves for 1 rook
            let pos: i8 = rook_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = index % 8;
            let y = index / 8;

            println!("Pos {pos}, Ind{index}");

            let directions: [[i8; 2]; 4] = [[0, 1], [0, -1], [-1, 0], [1, 0]];

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
                    new_board.remove_castling_for_rook(&curr_colour, index as u64); // remove castling for the rook since we are moving it.

                    new_board.remove_piece(index as u8); // Remove current rook position

                    // If I removed opp. rook, I update their castling bits
                    let opp_colour: PieceColour = match is_black {
                        0 => PieceColour::Black,
                        1 => PieceColour::White,
                        _ => PieceColour::Any,
                    };
                    new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                    let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )

                    new_board.rooks |= 1 << 64 * is_black + new_pos; // Update new rook position

                    // Update Tickers
                    new_board.update_tickers(piece_removed, is_black == 1);

                    println!("Board Hash: {}", new_board.hash());
                    legal_boards.push(new_board);
                    // Break if we had reached an opposite coloured piece
                    if piece_removed {
                        break;
                    }

                    new_x += delta_x;
                    new_y += delta_y;
                }
            }
            rook_positions &= !(1 << pos); // Flip the rook position to 0
        }
        println!("Number of Legal Moves after Rook: {}", legal_boards.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::base::defs::Board;
    use std::collections::HashSet;

    #[test]
    fn test_generate_rook_moves() {
        let file_path = "sample/test/rooks.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: Vec<Board> = Vec::new();
                board.generate_rook_moves(&mut legal_boards);

                let mut board_hashes: HashSet<u32> = HashSet::new();
                let hashes = [
                    3118420181, 3278978185, 1339305132, 4170517968, 3953619272, 2051754630,
                    3169314422, 318295110, 2298269325, 2778254618,
                ];
                for &hash in &hashes {
                    board_hashes.insert(hash);
                }

                let mut actual_board_hashes: HashSet<u32> = HashSet::new();
                for board in &legal_boards {
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
