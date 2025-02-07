use crate::base::defs::{ Board, PieceColour };

use super::defs::{GlobalMap, LegalMoveVec};

impl Board {
    pub fn get_knight_move_bit_map(&self, pos: i8, is_black: u8) -> u64 {
        let index: i8 = (63 - pos) as i8;
        let x = index / 8;
        let y = index % 8;
        let curr_colour: PieceColour = match is_black {
            1 => PieceColour::Black,
            0 => PieceColour::White,
            _ => PieceColour::Any,
        };
        let mut final_bit_map = GlobalMap::KNIGHT_MAP[x as usize][y as usize];
        let friend_pieces = self.consolidated_piece_map(&curr_colour);
        final_bit_map &= !friend_pieces;
        return final_bit_map;
    }

    pub fn generate_knight_moves(&self) -> LegalMoveVec {
        // TODO: Knight Moves
        // 1. [ X ] All 8 L shape moves around it ( Unless EOB or obstruction ) including capture
        // 2. [ X ] Take care to update castling bits if knight captures opp. rook
        // 3. [ X ] Take care of removing En-passant on non-pawn move.
        let mut legal_boards = LegalMoveVec::new();
        let is_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 0 } else { 1 };
        let mut knight_positions: u64 = (self.knights >> (64 * is_black)) as u64;
        while knight_positions != 0 {
            // Legal moves for 1 knight
            let pos: u8 = knight_positions.trailing_zeros() as u8;
            let index: u8 = (63 - pos) as u8;
            let mut knight_map = self.get_knight_move_bit_map(pos as i8, is_black);

            while knight_map != 0 {
                // Update the legal move in the vector
                let new_pos: u8 = knight_map.trailing_zeros() as u8;
                let new_index: u8 = (63 - new_pos) as u8;

                let mut new_board: Board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index); // Remove current knight position

                // If I removed opp. rook, I update their castling bits
                let opp_colour: PieceColour = match is_black {
                    1 => PieceColour::White,
                    0 => PieceColour::Black,
                    _ => PieceColour::Any,
                };
                new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.knights |= 1 << (64 * is_black + new_pos); // Update new knight position

                // Update Half & Full move clocks & toggle black / white move
                new_board.update_tickers(piece_removed, is_black == 1);
                new_board.set_enpassant(None);
                new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                new_board.latest_move &= (1 << 12) - 1;
                legal_boards.push(&mut new_board);

                knight_map &= !(1 << new_pos); // Flip the knight position to 0
            }

            knight_positions &= !(1 << pos); // Flip the knight position to 0
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
    fn test_generate_knight_moves() {
        GlobalMap::init();
        let fen = String::from("r1bqkb1r/pppp1ppp/8/2N1pN2/4P3/1n4nP/PPPP1PP1/R1BQKB1R b KQkq - 0 8");
        match Board::from_fen(&fen) {
            Some(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                let iterations = 1000000;
                let num_boards = 12;
                let start_time = Instant::now();
                for _ in 0..iterations {
                    legal_boards.clear();
                    legal_boards = board.generate_knight_moves();
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
                    5820322162005059073, 7248332367856294888, 3343382540968063460, 13734315367842045572,
                    15671716411796014867, 14442241140689718116, 2629788050489721178, 5902975122334121358,
                    3148796746828343536, 6017073384496795541, 8083710378863667238, 2302243076185838823,
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
            None => {
                panic!("Error loading board: {}", fen);
            }
        }
    }
}
