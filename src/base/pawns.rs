use crate::base::defs::Board;
use crate::base::defs::PieceColour;

impl Board {
    // at the given pos, checks if the pawn is in the last file.
    // If pawn is in last file, promote it and add the boards.
    // Else add the current board to legal boards.
    // Marking the castling bits, move tickers should be taken care by the caller.
    fn check_and_add_promotion(&self, index: u8, is_black: u8, legal_boards: &mut Vec<Board>) {
        let pos = 63 - index;
        let y: u8 = index / 8;
        println!("For promotion: {y}");

        if (is_black == 1 && y == 0) || (is_black == 0 && y == 7) {
            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.rooks |= 1 << 64 * is_black + pos;
            legal_boards.push(new_board);

            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.bishops |= 1 << 64 * is_black + pos;
            legal_boards.push(new_board);

            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.knights |= 1 << 64 * is_black + pos;
            legal_boards.push(new_board);

            let mut new_board = self.clone();
            new_board.remove_piece(index as u8);
            new_board.queens |= 1 << 64 * is_black + pos;
            legal_boards.push(new_board);
            println!("Pushed promoted boards");
        } else {
            legal_boards.push(*self);
        }
    }

    pub fn generate_pawn_moves(&self, legal_boards: &mut Vec<Board>) {
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

        let is_black: u8 = if (self.metadata >> 8) & 1 == 1 { 0 } else { 1 };
        let opp_color = PieceColour::from_u8((is_black == 0) as u8);
        let incr_sign: i8 = 1 - 2 * (is_black as i8);
        let mut pawn_positions: u64 = (self.pawns >> 64 * is_black) as u64;
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
            if all_piece_map & (1 << new_pos) == 0 {
                let mut new_board: Board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index as u8); // Remove current pawn position
                new_board.pawns |= 1 << 64 * is_black + new_pos; // Update new pawn position
                new_board.update_tickers(true, is_black == 1); // Update Tickers
                new_board.set_enpassant( None );
                new_board.check_and_add_promotion(new_index, is_black, legal_boards);
            }

            // Double step if unobstructing. Note that we will never have a pawn at last index.
            // Take care of updating the En-passant condition.
            if (is_black == 1 && y == 6) || (is_black != 1 && y == 1) {
                let new_index = (x + (y + 2 * incr_sign) * 8) as u8;
                let new_pos = (63 - new_index) as u8;

                // If unobstructing, add to legal boards
                if all_piece_map & (1 << new_pos) == 0 {
                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(index as u8); // Remove current pawn position
                    new_board.pawns |= 1 << 64 * is_black + new_pos; // Update new pawn position
                    new_board.update_tickers(true, is_black == 1); // Update Tickers
                    new_board.set_enpassant( Some( x as u8 ) ); // mark en-passant possible at current x.
                    legal_boards.push(new_board);
                }
            }

            // Left and Right single diagonal capture, if opponent piece
            for delta_x in [-1, 1] {
                if x + delta_x < 0 || x + delta_x > 7 {
                    continue;
                }
                let new_index = ((x + delta_x) + (y + 1 * incr_sign) * 8) as u8;
                let new_pos = (63 - new_index) as u8;

                // if opposite piece present, capture it.
                if opp_piece_map & (1 << new_pos) != 0 {
                    let mut new_board: Board = self.clone(); // Clone the board to modify it
                    new_board.remove_piece(index as u8); // Remove current pawn position
                    new_board.remove_castling_for_rook(&opp_color, new_index as u64);
                    new_board.remove_piece(new_index as u8);
                    new_board.pawns |= 1 << 64 * is_black + new_pos; // Update new pawn position
                    new_board.update_tickers(true, is_black == 1); // Update Tickers
                    new_board.set_enpassant( None );
                    new_board.unmark_enpassant();
                    new_board.check_and_add_promotion(new_index, is_black, legal_boards);
                }
            }

            // En-passant implementation
            let en_passant_possible: Option<i8> = self.get_enpassant();
            if let Some(enpassant_x) = en_passant_possible {
                if (enpassant_x - x).abs() == 1
                    && ((is_black == 0 && y == 4) || (is_black == 1 && y == 3))
                {
                    let new_index = enpassant_x + (y + 1 * incr_sign) * 8;
                    let new_pos = (63 - new_index) as u8;

                    let pawn_to_remove_index = enpassant_x + y * 8;

                    let mut new_board: Board = self.clone(); // Clone the board to modify it

                    new_board.remove_piece(index as u8); // Remove current pawn position
                    new_board.remove_piece(pawn_to_remove_index as u8); // remove opp pawn
                    new_board.pawns |= 1 << 64 * is_black + new_pos; // Update new pawn position

                    new_board.update_tickers(true, is_black == 1); // Update Tickers
                    new_board.set_enpassant( None );
                    legal_boards.push(new_board);
                }
            }
            pawn_positions &= !(1 << pos); // Flip the pawn position to 0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::defs::Board;
    use std::collections::HashSet;

    #[test]
    fn test_generate_pawn_moves() {
        let file_path = "sample/test/pawns.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: Vec<Board> = Vec::new();
                board.generate_pawn_moves(&mut legal_boards);

                let mut board_hashes: HashSet<u32> = HashSet::new();
                let hashes = [
                    3376416924, 2938455243, 161785497, 2441280661, 2132923696, 3474119380,
                    2482819437, 815729389, 1800242416, 3995466229, 3050026149, 766271150,
                ];

                assert_eq!(hashes.len(), legal_boards.len());
                for &hash in &hashes {
                    board_hashes.insert(hash);
                }

                let mut actual_board_hashes: HashSet<u32> = HashSet::new();
                let mut i = 0;
                for board in &legal_boards {
                    let board_hash = board.hash();
                    if 2989022849 == board_hash {
                        println!("{}", i);
                    }
                    i += 1;

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
