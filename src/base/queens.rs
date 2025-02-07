use crate::base::defs::{ Board, PieceColour };

use super::defs::LegalMoveVec;

impl Board {
    pub fn get_queen_move_bit_map(&self, pos: i8, is_black: u8) -> u64 {
        self.get_bishop_move_bit_map(pos, is_black) | self.get_rook_move_bit_map(pos, is_black)
    }

    pub fn generate_queen_moves(&self) -> LegalMoveVec {
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
        // 11. [ X ] Take care of removing En-passant on non-pawn move.
        let mut legal_boards = LegalMoveVec::new();
        let is_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 0 } else { 1 };
        let mut queen_positions: u64 = (self.queens >> (64 * is_black)) as u64;
        let opp_colour: PieceColour = match is_black {
            0 => PieceColour::Black,
            1 => PieceColour::White,
            _ => PieceColour::Any,
        };

        while queen_positions != 0 {
            // Legal moves for 1 queen
            let pos: i8 = queen_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let mut final_dir_bitmap = self.get_queen_move_bit_map(pos, is_black);

            while final_dir_bitmap != 0 {
                let new_pos = final_dir_bitmap.trailing_zeros() as u8;
                let new_index = (63 - new_pos) as u8;

                let mut new_board: Board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index as u8); // Remove current queen position

                // If I removed opp. rook, I update their castling bits
                new_board.remove_castling_for_rook(&opp_colour, new_index as u64);

                let piece_removed = new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.queens |= 1 << (64 * is_black + new_pos); // Update new queen position

                // Update Tickers
                new_board.update_tickers(piece_removed, is_black == 1);
                new_board.set_enpassant(None);
                new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                new_board.latest_move &= (1 << 12) - 1;
                legal_boards.push(&mut new_board);
                final_dir_bitmap &= !(1 << new_pos); // Flip the bit map position to 0
            }
            queen_positions &= !(1 << pos); // Flip the queen position to 0
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
    fn test_generate_queen_moves() {
        GlobalMap::init();
        let file_path = "sample/test/queens.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                let iterations = 1000000;
                let num_boards = 44;
                let start_time = Instant::now();
                for _ in 0..iterations {
                    legal_boards.clear();
                    legal_boards = board.generate_queen_moves();
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
                    1342122215706044516, 382088654981705668, 18125960016145754612, 3127957963239896246,
                    2637972985423030976, 7942556423377308867, 3454327136929027804, 5602117116560727328,
                    5051334933369314611, 10689619840762180197, 6380778275037202962, 11054537091290163333,
                    18212121232267464686, 13988209341383220336, 1861602228070179428, 13227717758378044071,
                    12271173355272759689, 13703204505884263133, 3049012822008875487, 5339922469792727955,
                    15644594742818318253, 17307495599514643529, 12713155747508494028, 14573237342990770899,
                    5733019969221208102, 13385004155880422746, 9483777171666273866, 1575166165921787005,
                    8962617884902602377, 16565004358882248281, 1613191025400046826, 8386448209258305709,
                    3404699087101616012, 15068900313372125978, 6267553524569385613, 10999009690469369101,
                    7584023421471870435, 17042760075846961287, 9564582805228945453, 12343237563164531853,
                    332781063184234792, 17061353156522715563, 2821133804232163975, 7236432944459630853,
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
