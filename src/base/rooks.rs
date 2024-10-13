use crate::base::defs::Board;

impl Board {

    pub fn generate_rook_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Rook Moves

        // 1. [ ] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [ ] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [ ] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [ ] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [ ] Take care to update the castling bits ( King or Queenside ) on first rook move
        println!( "Number of Legal Moves after Rook: {}", legal_boards.len() );
    }

}