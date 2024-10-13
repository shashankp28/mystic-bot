use crate::base::defs::Board;
use crate::base::defs::PieceColour;

impl Board {

    pub fn generate_knight_moves( &mut self, legal_boards: &mut Vec<Board> ) {
        // TODO: Knight Moves
        // 1. [ X ] All 8 L shape moves around it ( Unless EOB or obstruction ) including capture
        println!( "Generating Knight Moves..." );
        let basic_knight_map: u64 = 21617444997;  // Through Experimentation
        let left_half_board_map: u64 = 17361641481138401520;  // Through Experimentation
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };
        let mut knight_positions: u64 = ( self.knights >> 64*is_black ) as u64;
        println!( "Found Current Knight Positions.." );
        while knight_positions != 0 {
            // Legal moves for 1 knight
            let pos: u8 = knight_positions.trailing_zeros() as u8;
            let index: u8 = ( 63 - pos ) as u8;
            let mut new_knight_map: u64 = 0;

            // Through Experimentation
            if index + 17 <= 63 {
                new_knight_map |= basic_knight_map << ( 63 - ( index + 17 ) );
            } else {
                new_knight_map |= basic_knight_map >> ( ( index + 17 ) - 63 )
            }
            if index%8 < 2 {
                new_knight_map &= left_half_board_map
            }
            if index%8 > 5 {
                new_knight_map &= !left_half_board_map
            }

            // Remove all bits where the knight jumps on same coloured piece
            let curr_colour: PieceColour = match is_black {
                1 => PieceColour::Black,
                0 => PieceColour::White,
                _ => PieceColour::Any,
            };
            new_knight_map &= !self.consolidated_piece_map( curr_colour );
            
            while new_knight_map != 0 {
                // Update the legal move in the vector
                let new_pos: u8 = new_knight_map.trailing_zeros() as u8;
                let new_index: u8 = ( 63 - new_pos ) as u8;

                let mut new_board = self.clone(); // Clone the board to modify it
                new_board.remove_piece( index ); // Remove current knight position
                new_board.remove_piece( new_index ); // Remove existing piece ( for capture )
                new_board.knights |= 1 << 64*is_black+new_pos; // Update new knight position
                legal_boards.push( new_board );

                new_knight_map &= !( 1 << new_pos ); // Flip the knight position to 0 
            }

            knight_positions &= !( 1 << pos ); // Flip the knight position to 0 
        }
    }

}