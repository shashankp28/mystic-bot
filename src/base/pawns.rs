use crate::base::defs::Board;

impl Board {

    pub fn generate_pawn_moves( &self, legal_boards: &mut Vec<Board> ) {
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
        println!( "Number of Legal Moves after Pawn: {}", legal_boards.len() );
    }

}