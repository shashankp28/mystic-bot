use crate::base::defs::Board;

impl Board {

    pub fn generate_queen_moves( &self, legal_boards: &mut Vec<Board> ) {
        // TODO: Queen Moves

        // 1. [ ] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [ ] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [ ] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [ ] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [ ] Every NE ( North-East ) diagonal until EOB or Capture or obstruction
        // 6. [ ] Every SE ( South-East ) diagonal until EOB or Capture or obstruction
        // 7. [ ] Every SW ( South-West ) diagonal until EOB or Capture or obstruction
        // 8. [ ] Every NW ( North-West ) diagonal until EOB or Capture or obstruction
        println!( "Number of Legal Moves after Queen: {}", legal_boards.len() );
    }

}