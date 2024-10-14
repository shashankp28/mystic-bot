use crate::base::defs::Board;

impl Board {

    pub fn generate_king_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: King Moves

        // 1. [ ] All 8 squares around the king except EOB or obstruction including capture
        // 2. [ ] Castling to the King-side
        // 3. [ ] Castling to the Queen-side
        // 4. [ ] Take care to update the castling bits ( King and Queenside ) on first king move
        // 5. [  ] Take care to update castling bits if king captures opp. rook
        println!( "Number of Legal Moves after King: {}", legal_boards.len() );
    }

    pub fn prune_illegal_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Pin / Check analysis
        // 1. [ ] Remove moves in which the king is in check
        println!( "Number of Legal Moves after Pruning Illegal: {}", legal_boards.len() );
    }

}