use crate::base::defs::Board;

impl Board {
    pub fn generate_pawn_moves(&self, legal_boards: &mut Vec<Board>) {
        // TODO: Pawn Moves

        // 1. [ ] Single step forward if unobstructing
        // 2. [ ] Double step forward if first move and unobstructing
        // 3. [ ] Left single diagonal capture, if opponent piece
        // 4. [ ] Right single diagonal capture if opponent piece
        // 5. [ ] Promote to Queen if on last file
        // 6. [ ] Promote to Rook if on last file
        // 7. [ ] Promote to Knight if on last file
        // 8. [ ] Promote to Bishop if on last file
        // 9. [ ] En-passant if conditions are right
        // 10. [ ] Take care to update castling bits if pawn captures opp. rook
        // 11. [ ] Take care of updating per move tickers like white/block move, half clock, full number
        // 12. [ ] Take care of updating En-passant conditions
        println!("Number of Legal Moves after Pawn: {}", legal_boards.len());

        
    }
}

#[cfg(test)]
mod tests {
    use crate::base::defs::Board;
    use std::collections::HashSet;

    #[test]
    fn test_generate_pawn_moves() {
        let file_path = "sample/test/pawns1.json";
        match Board::from_file(file_path) {
            Ok(board) => {
                println!("Successfully loaded board: {:?}", board);
                let mut legal_boards: Vec<Board> = Vec::new();
                board.generate_pawn_moves(&mut legal_boards);

                let mut board_hashes: HashSet<u32> = HashSet::new();
                let hashes = [
                    1384374154, 2111757897, 2673898309, 2366286041, 1727127921, 2573150337,
                    2536742162, 2680631764, 160663804, 1836417416, 1096515824, 1936876890,
                    3510469015
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
