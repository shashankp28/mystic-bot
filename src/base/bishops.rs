use crate::base::defs::Board;

impl Board {

    pub fn generate_bishop_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Bishop Moves

        // 1. [ ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 2. [ ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 3. [ ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 4. [ ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
        // 5. [ ] Take care to update castling bits if bishop captures opp. rook
        // 6. [ ] Take care of updating per move tickers like white/block move, half clock, full number
        println!( "Number of Legal Moves after Bishop: {}", legal_boards.len() );
    }

}