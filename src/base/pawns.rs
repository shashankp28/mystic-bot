use crate::base::defs::{ Board, PieceColour };

use super::defs::{ GlobalMap, LegalMoveVec };

impl Board {
    pub fn get_pawn_attack_bit_map(&self, pos: i8, is_black: u8) -> u64 {
        let index: i8 = (63 - pos) as i8;
        let x = index / 8;
        let y = index % 8;
        let curr_colour: PieceColour = match is_black {
            1 => PieceColour::Black,
            0 => PieceColour::White,
            _ => PieceColour::Any,
        };
        let mut final_bit_map = GlobalMap::PAWN_MAP[is_black as usize][x as usize][y as usize];
        let friend_pieces = self.consolidated_piece_map(&curr_colour);
        final_bit_map &= !friend_pieces;
        return final_bit_map;
    }

    fn check_and_add_promotion(
        &mut self,
        index: u8,
        is_black: u8,
        legal_boards: &mut LegalMoveVec
    ) {
        // at the given pos, checks if the pawn is in the last file.
        // If pawn is in last file, promote it and add the boards.
        // Else add the current board to legal boards.
        // Marking the castling bits, move tickers should be taken care by the caller.
        let pos = 63 - index;
        let y: u8 = index / 8;

        if (is_black == 1 && y == 0) || (is_black == 0 && y == 7) {
            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.queens |= 1 << (64 * is_black + pos);
            new_board.latest_move |= 4 << 12; // 1 00
            legal_boards.push(&mut new_board);

            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.rooks |= 1 << (64 * is_black + pos);
            new_board.latest_move |= 5 << 12; // 1 01
            legal_boards.push(&mut new_board);

            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.bishops |= 1 << (64 * is_black + pos);
            new_board.latest_move |= 6 << 12; // 1 10
            legal_boards.push(&mut new_board);

            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.knights |= 1 << (64 * is_black + pos);
            new_board.latest_move |= 7 << 12; // 1 11
            legal_boards.push(&mut new_board);
        } else {
            legal_boards.push(&mut *self);
        }
    }

    pub fn generate_pawn_moves(&self) -> LegalMoveVec {
        // TODO: Pawn Moves

        // 1. [X] Single step forward if unobstructing
        // 2. [X] Double step forward if first move and unobstructing
        // 3. [X] Left single diagonal capture, if opponent piece
        // 4. [X] Right single diagonal capture if opponent piece
        // 5. [X] Promote to Queen if on last file
        // 6. [X] Promote to Rook if on last file
        // 7. [X] Promote to Knight if on last file
        // 8. [X] Promote to Bishop if on last file
        // 9. [X] En-passant if conditions are right
        // 10. [X] Take care to update castling bits if pawn captures opp. rook
        // 11. [X] Take care of updating per move tickers like white/block move, half clock, full number
        // 12. [X] Take care of updating En-passant conditions on Double step forward
        // 13. [X] Take care of removing En-passant on non Double step move.
        let mut legal_boards = LegalMoveVec::new();
        let is_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 0 } else { 1 };
        let opp_color = PieceColour::from_u8((is_black == 0) as u8);
        let incr_sign: i8 = 1 - 2 * (is_black as i8);
        let mut pawn_positions: u64 = (self.pawns >> (64 * is_black)) as u64;
        let all_piece_map = self.consolidated_piece_map(&PieceColour::Any);
        let opp_piece_map = self.consolidated_piece_map(&opp_color);

        while pawn_positions != 0 {
            // Legal moves for 1 pawn
            let pos: i8 = pawn_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x: i8 = index % 8;
            let y: i8 = index / 8;

            // Single step if unobstructing. Note that we will never have a pawn at last index.
            let new_index = (x + (y + 1 * incr_sign) * 8) as u8;
            let new_pos = (63 - new_index) as u8;

            // If unobstructing, add to legal boards
            if (all_piece_map & (1 << new_pos)) == 0 {
                let mut new_board: Board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index as u8); // Remove current pawn position
                new_board.pawns |= 1 << (64 * is_black + new_pos); // Update new pawn position
                new_board.update_tickers(true, is_black == 1); // Update Tickers
                new_board.set_enpassant(None);
                new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                new_board.latest_move &= (1 << 12) - 1;
                new_board.check_and_add_promotion(new_index, is_black, &mut legal_boards);
            }

            // Double step if unobstructing. Note that we will never have a pawn at last index.
            // Take care of updating the En-passant condition.
            if (is_black == 1 && y == 6) || (is_black != 1 && y == 1) {
                let new_index = (x + (y + 2 * incr_sign) * 8) as u8;
                let new_pos = (63 - new_index) as u8;
                let obstruction_map = (2155872256 << (16 * (1 - is_black))) >> x;

                // If unobstructing, add to legal boards
                if (all_piece_map & obstruction_map) == 0 {
                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(index as u8); // Remove current pawn position
                    new_board.pawns |= 1 << (64 * is_black + new_pos); // Update new pawn position
                    new_board.update_tickers(true, is_black == 1); // Update Tickers
                    new_board.set_enpassant(Some(x as u8)); // mark en-passant possible at current x.
                    new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                    new_board.latest_move &= (1 << 12) - 1;
                    legal_boards.push(&mut new_board);
                }
            }

            // Left and Right single diagonal capture, if opponent piece
            for delta_x in [-1, 1] {
                if x + delta_x < 0 || x + delta_x > 7 {
                    continue;
                }
                let new_index = (x + delta_x + (y + 1 * incr_sign) * 8) as u8;
                let new_pos = (63 - new_index) as u8;

                // if opposite piece present, capture it.
                if (opp_piece_map & (1 << new_pos)) != 0 {
                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(index as u8); // Remove current pawn position
                    new_board.remove_castling_for_rook(&opp_color, new_index as u64);
                    new_board.remove_piece(new_index as u8);
                    new_board.pawns |= 1 << (64 * is_black + new_pos); // Update new pawn position
                    new_board.update_tickers(true, is_black == 1); // Update Tickers
                    new_board.set_enpassant(None);
                    new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                    new_board.latest_move &= (1 << 12) - 1;
                    new_board.check_and_add_promotion(new_index, is_black, &mut legal_boards);
                }
            }

            // En-passant implementation
            let en_passant_possible: Option<i8> = self.get_enpassant();
            if let Some(enpassant_x) = en_passant_possible {
                if
                    (enpassant_x - x).abs() == 1 &&
                    ((is_black == 0 && y == 4) || (is_black == 1 && y == 3))
                {
                    let new_index = enpassant_x + (y + 1 * incr_sign) * 8;
                    let new_pos = (63 - new_index) as u8;

                    let pawn_to_remove_index = enpassant_x + y * 8;

                    let mut new_board: Board = self.clone(); // Clone the board to modify it

                    new_board.remove_piece(index as u8); // Remove current pawn position
                    new_board.remove_piece(pawn_to_remove_index as u8); // remove opp pawn
                    new_board.pawns |= 1 << (64 * is_black + new_pos); // Update new pawn position

                    new_board.update_tickers(true, is_black == 1); // Update Tickers
                    new_board.set_enpassant(None);
                    new_board.latest_move = (((index as u16) << 6) | (new_index as u16)) as u16;
                    new_board.latest_move &= (1 << 12) - 1;
                    legal_boards.push(&mut new_board);
                }
            }
            pawn_positions &= !(1 << pos); // Flip the pawn position to 0
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
    fn test_generate_pawn_moves() {
        GlobalMap::init();
        let fen = String::from(
            "rn2k1nr/p2bbpP1/6q1/Pp1pp1Pp/2pNP3/7P/1PPP4/RNBQKB1R w KQkq b6 0 13"
        );
        match Board::from_fen(&fen) {
            Some(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: LegalMoveVec = LegalMoveVec::new();
                let iterations = 1000000;
                let num_boards = 12;
                let start_time = Instant::now();
                for _ in 0..iterations {
                    legal_boards.clear();
                    legal_boards = board.generate_pawn_moves();
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
                    10182643459555330204, 13663192652653610187, 400160999323510425, 17836208204372180117,
                    15042658352580646192, 14264939703325350612, 3672314799976141165, 13327077293154044653,
                    7923809918087169264, 15917505594494679541, 16548048423720039589, 18040599868043189934,
                ];

                assert_eq!(hashes.len(), legal_boards.len());
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
