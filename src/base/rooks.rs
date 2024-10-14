use crate::base::defs::Board;

impl Board {
    pub fn generate_rook_moves(&mut self, legal_boards: &mut Vec<Board>) {
        // TODO: Rook Moves

        // 1. [X] Every Straight Up until EOB ( End of board ) or capture or obstruction
        // 2. [X] Every Straight Down until EOB ( End of board ) or capture or obstruction
        // 3. [X] Every Straight Right until EOB ( End of board ) or capture or obstruction
        // 4. [X] Every Straight Left until EOB ( End of board ) or capture or obstruction
        // 5. [ ] Take care to update the castling bits ( King or Queenside ) on first rook move
        // 6. [ ] Take care to update castling bits if rook captures opp. rook
        // 7. [ ] Take care of updating per move tickers like white/block move, half clock, full number
        println!("Generating Rook Moves...");
        let is_black: u8 = if (self.metadata >> 8) & 1 == 1 { 0 } else { 1 };
        let mut rook_positions: u64 = (self.rooks >> 64 * is_black) as u64;
        println!("Found Rook positions");
        while rook_positions != 0 {
            let pos: u8 = rook_positions.trailing_zeros() as u8;
            let index: u8 = (63 - pos) as u8;

            println!("index: {index}");

            let horizontal_pos: u8 = index % 8;
            let vertical_pos: u8 = index / 8;

            let mut incr: u8 = 1;
            loop {
                let new_index = (vertical_pos + incr) * 8 + horizontal_pos;
                if vertical_pos + incr >= 8 || self.is_same_color_piece_present(new_index, is_black)
                {
                    break;
                }
                let mut new_board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index); // Remove current rook position
                new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.rooks |= 1 << 64 * is_black + (63 - new_index); // Update new rook position
                legal_boards.push(new_board);

                if self.is_different_color_piece_present(new_index, is_black) {
                    break;
                }
                incr += 1;
                println!("increased vertically: {incr}");
            }

            let mut decr: u8 = 1;
            loop {
                if vertical_pos < decr {
                    break;
                }
                let new_index = (vertical_pos - decr) * 8 + horizontal_pos;
                if self.is_same_color_piece_present(new_index, is_black) {
                    break;
                }
                let mut new_board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index); // Remove current rook position
                new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.rooks |= 1 << 64 * is_black + (63 - new_index); // Update new rook position
                legal_boards.push(new_board);

                if self.is_different_color_piece_present(new_index, is_black) {
                    break;
                }
                decr += 1;
                println!("Decreased vertically: {decr}");
            }

            let mut incr: u8 = 1;
            loop {
                let new_index = (vertical_pos) * 8 + horizontal_pos + incr;
                if horizontal_pos + incr >= 8
                    || self.is_same_color_piece_present(new_index, is_black)
                {
                    break;
                }
                let mut new_board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index); // Remove current rook position
                new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.rooks |= 1 << 64 * is_black + (63 - new_index); // Update new rook position
                legal_boards.push(new_board);

                if self.is_different_color_piece_present(new_index, is_black) {
                    break;
                }
                incr += 1;
                println!("increased horizontally: {incr}");
            }

            let mut decr: u8 = 1;
            loop {
                if horizontal_pos < decr {
                    break;
                }
                let new_index = (vertical_pos) * 8 + horizontal_pos - decr;
                if self.is_same_color_piece_present(new_index, is_black) {
                    break;
                }
                let mut new_board = self.clone(); // Clone the board to modify it
                new_board.remove_piece(index); // Remove current rook position
                new_board.remove_piece(new_index); // Remove existing piece ( for capture )
                new_board.rooks |= 1 << 64 * is_black + (63 - new_index); // Update new rook position
                legal_boards.push(new_board);

                if self.is_different_color_piece_present(new_index, is_black) {
                    break;
                }
                decr += 1;
                println!("Decreased horizontally: {decr}");
            }

            rook_positions &= !(1 << pos); // Flip the knight position to 0
        }

        println!("Number of Legal Moves after Rook: {}", legal_boards.len());
    }
}
