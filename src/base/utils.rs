use crate::base::defs::{Board, CastleSide, PieceColour, LegalMoveVec};
use serde_json::to_writer_pretty;
use std::fs::File;
use std::hash::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;

impl Board {
    pub fn get_number_pieces(&self) -> u32 {
        let rooks_count = self.rooks.count_ones();
        let knights_count = self.knights.count_ones();
        let bishops_count = self.bishops.count_ones();
        let queens_count = self.queens.count_ones();
        let kings_count = self.kings.count_ones();
        let pawns_count = self.pawns.count_ones();
        rooks_count + knights_count + bishops_count + queens_count + kings_count + pawns_count
    }

    pub fn consolidated_piece_map(&self, colour: &PieceColour) -> u64 {
        let all_piece_map: u128 =
            self.rooks | self.knights | self.bishops | self.queens | self.kings | self.pawns;
        match colour {
            PieceColour::Black => (all_piece_map >> 64) as u64,
            PieceColour::White => all_piece_map as u64,
            PieceColour::Any => (all_piece_map >> 64) as u64 | all_piece_map as u64,
        }
    }

    pub fn remove_piece(&mut self, index: u8) -> bool {
        // Remove piece from bitMap if any piece exists at that index,
        // The logic of colour / legality of the move must be taken care
        // from the caller's side. Return True if a piece was actually removed
        let mut removal_map: u128 = 0;
        removal_map |= (1 << (63 - index)) | (1 << (127 - index));
        let piece_removed: bool = (self.rooks & removal_map
            | self.knights & removal_map
            | self.bishops & removal_map
            | self.queens & removal_map
            | self.kings & removal_map
            | self.pawns & removal_map)
            != 0;
        removal_map = !removal_map;
        self.rooks &= removal_map;
        self.knights &= removal_map;
        self.bishops &= removal_map;
        self.queens &= removal_map;
        self.kings &= removal_map;
        self.pawns &= removal_map;
        piece_removed
    }

    pub fn update_tickers(&mut self, half_reset: bool, is_black: bool) {
        let mut current_half_clock = (self.metadata >> 9) & 127;
        let mut current_full_number = self.metadata >> 16;
        current_half_clock = if half_reset {
            0
        } else {
            current_half_clock + 1
        };
        if is_black {
            current_full_number += 1;
        }
        self.metadata &= !(127 << 9);
        self.metadata &= !(65535 << 16);

        self.metadata |= current_half_clock << 9;
        self.metadata |= current_full_number << 16;

        self.metadata ^= 1 << 8; // Updating black / white move
    }

    pub fn remove_castling_for_rook(&mut self, colour: &PieceColour, index: u64) {
        // Removes Castling bit for a rook at index if it is present.
        match colour {
            PieceColour::Black => {
                if self.rooks >> 64 as u64 & 1 << (63 - index) != 0 {
                    if index == 56 {
                        self.remove_castling_bits(CastleSide::Queen, colour);
                    } else if index == 63 {
                        self.remove_castling_bits(CastleSide::King, colour);
                    }
                }
            }
            PieceColour::White => {
                if self.rooks as u64 & 1 << (63 - index) != 0 {
                    if index == 0 {
                        self.remove_castling_bits(CastleSide::Queen, colour);
                    } else if index == 7 {
                        self.remove_castling_bits(CastleSide::King, colour);
                    }
                }
            }
            PieceColour::Any => {}
        }
    }

    pub fn remove_castling_bits(&mut self, side: CastleSide, colour: &PieceColour) {
        match colour {
            PieceColour::White => match side {
                CastleSide::Queen => self.metadata &= !(1 << 0),
                CastleSide::King => self.metadata &= !(1 << 1),
            },
            PieceColour::Black => match side {
                CastleSide::Queen => self.metadata &= !(1 << 2),
                CastleSide::King => self.metadata &= !(1 << 3),
            },
            PieceColour::Any => {}
        }
    }

    pub fn set_enpassant(&mut self, x: Option<u8>) {
        // Clear the en-passant bits (bits 4-7)
        self.metadata &= !(0b11110000);

        if let Some(pos) = x {
            // Set the en-passant bit (bit 7) and the position (bits 4-6)
            self.metadata |= (1 << 7) as u32; // Mark en-passant as possible
            self.metadata |= (pos << 4) as u32; // Set the en-passant column
        }
    }

    pub fn get_enpassant(&self) -> Option<i8> {
        if self.metadata & (1 << 7) != 0 {
            Some(((self.metadata >> 4) & 0b111) as i8)
        } else {
            None
        }
    }

    pub fn can_attack( &self, is_black: u8, target: u8 ) -> bool {
        // Check if target is same colour
        let curr_colour: PieceColour = match is_black {
                1 => PieceColour::Black,
                0 => PieceColour::White,
                _ => PieceColour::Any,
        };
        let curr_position_map = self.consolidated_piece_map(&curr_colour);
        if curr_position_map & ( 1 << (63-target) ) != 0 {
            return false;
        }
        let all_piece_map = self.consolidated_piece_map(&PieceColour::Any);

        // Can Bishop+Queen Attack Diagonally?
        let bishop_positions: u64 = (self.bishops >> 64 * is_black) as u64;
        let queen_positions: u64 = (self.queens >> 64 * is_black) as u64;
        let mut bishop_queen_positions: u64 = bishop_positions | queen_positions;
        let target_x = (target%8) as i8;
        let target_y = (target/8) as i8;
        while bishop_queen_positions != 0 {
            // Pick a direction and go until one step behind the target
            // To check if there are no obstructions
            let pos: i8 = bishop_queen_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = index % 8;
            let y = index / 8;
            if (target_x-x).abs() == (target_y-y).abs() {
                let delta_x = (target_x-x)/(target_x-x).abs();
                let delta_y = (target_y-y)/(target_y-y).abs();
                let steps = (target_x-x).abs();
                let mut attacks = true;
                for i in 1..steps {
                    let new_index = x+i*delta_x + 8*(y+i*delta_y);
                    if all_piece_map & ( 1 << (63-new_index) ) != 0 {
                        attacks = false;
                        break;
                    }
                }
                if attacks {
                    return true;
                }
            }
            
            bishop_queen_positions &= !(1 << pos);            
        }
        
        // Can Rook+Queen Attack in a straight line?
        let rook_positions: u64 = (self.rooks >> 64 * is_black) as u64;
        let mut rook_queen_positions: u64 = rook_positions | queen_positions;
        while rook_queen_positions != 0 {
            // Pick a direction and go until one step behind the target
            // To check if there are no obstructions
            let pos: i8 = rook_queen_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = index % 8;
            let y = index / 8;
            if (target_x-x).abs() == 0 || (target_y-y).abs() == 0 {
                let delta_x = if target_x != x { (target_x-x)/(target_x-x).abs() } else { 0 };
                let delta_y = if target_y != y { (target_y-y)/(target_y-y).abs() } else { 0 };
                let steps = if target_x != x { (target_x-x).abs() } else { (target_y-y).abs() };
                let mut attacks = true;
                for i in 1..steps {
                    let new_index = x+i*delta_x + 8*(y+i*delta_y);
                    if all_piece_map & ( 1 << (63 - new_index) ) != 0 {
                        attacks = false;
                        break;
                    }
                }
                if attacks {
                    return true;
                }
            }
            rook_queen_positions &= !(1 << pos);            
        }
        
        // Can Knight Attack?
        let mut knight_positions: u64 = (self.knights >> 64 * is_black) as u64;
        while knight_positions != 0 {
            // Check the delta vector is of the form (+-2, +-1) or (+-1, +-2)
            let pos: i8 = knight_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = index % 8;
            let y = index / 8;
            let delta_x = (target_x-x).abs();
            let delta_y = (target_y-y).abs();
            if (delta_x==2 && delta_y==1) || (delta_x==1 && delta_y==2) {
                return true;
            }
            knight_positions &= !(1 << pos);            
        }
        
        // Can Pawn Attack?
        let mut pawn_positions: u64 = (self.pawns >> 64 * is_black) as u64;
        while pawn_positions != 0 {
            // Check if delta_x absolute value value is 1
            // Check delta_y is -1 for black and +1 for white
            let pos: i8 = pawn_positions.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = index % 8;
            let y = index / 8;
            let delta_x = target_x-x;
            let delta_y = target_y-y;
            if delta_x.abs() == 1 {
                if delta_y == (1-2*is_black as i8) {
                    return true;
                }
            }
            pawn_positions &= !(1 << pos);            
        }

        // Can King Attack?
        let king_position: u64 = (self.kings >> 64 * is_black) as u64;
        let pos: i8 = king_position.trailing_zeros() as i8;
        let index: i8 = (63 - pos) as i8;
        let x = index % 8;
        let y = index / 8;
        let delta_x = (target_x-x).abs();
        let delta_y = (target_y-y).abs();
        if delta_x <= 1 && delta_y <= 1 {
            return true;
        }

        return false;
    }

    pub fn is_legal( &self ) -> bool {
        let prev_was_black: u8 = if (self.metadata >> 8) & 1 == 1 { 1 } else { 0 };
        let king_position: u64 = (self.kings >> 64 * prev_was_black) as u64;
        let pos: i8 = king_position.trailing_zeros() as i8;
        let index: u8 = (63 - pos) as u8;
        return !self.can_attack(1-prev_was_black, index);
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.rooks.hash(&mut hasher);
        self.knights.hash(&mut hasher);
        self.bishops.hash(&mut hasher);
        self.queens.hash(&mut hasher);
        self.kings.hash(&mut hasher);
        self.pawns.hash(&mut hasher);
        self.metadata.hash(&mut hasher);
        hasher.finish() as u64
    }

    // TODO: Some Global Rules to take care of:
    //
    // 1. [ X ] A legal move should be discarded, if after making the move current king is under check!!
    // 2. [ X ] Castling can be done only in the following cases
    //      a. [ X ] King and the corresponding rook shouldn't have moved
    //      b. [ X ] The king should not be in check
    //      c. [ X ] The squares the king moves through during castling should not be in check
    //      d. [ X ] There should be no pieces between the king and the corresponding rook
    // 3. [ X ] En-Passant can only be done, `ONLY IMMEDIATELY` after the opponent moves double step pawn
    // 4. [ X ] Check is when the king is directly under threat
    // 5. [ X ] Repeating a sequence of moves 3 times draws
    // 6. [ X ] Checkmate is when king is under check and there are no legal moves (win/lose)
    // 7. [ X ] Stalemate is when there are no legal moves, but the king is not in check (draw)
    // 8. [ X ] Keep track and update the Half Move Clock
    // 9. [ X ] Keep track and update the Full Move Number

    pub fn get_legal_moves(self) -> LegalMoveVec {
        let mut legal_boards = LegalMoveVec::new();

        // Generate all possible legal moves
        self.generate_rook_moves(&mut legal_boards);
        self.generate_knight_moves(&mut legal_boards);
        self.generate_bishop_moves(&mut legal_boards);
        self.generate_queen_moves(&mut legal_boards);
        self.generate_pawn_moves(&mut legal_boards);
        self.generate_king_moves(&mut legal_boards);

        legal_boards
    }

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let board: Board = serde_json::from_str(&contents)?;
        Ok(board)
    }

    pub fn save_board(&self, file_name: &str) {
        let file = File::create(file_name).expect("Unable to create file");
        match to_writer_pretty(&file, &self) {
            Ok(_) => {
                println!("Board saved successfully to {}", file_name);
            }
            Err(e) => {
                println!("Error serializing board: {}", e);
            }
        }
    }
}

impl PieceColour {
    pub fn from_u8(is_black: u8) -> Self {
        match is_black {
            0 => PieceColour::White,
            1 => PieceColour::Black,
            _ => PieceColour::Any,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::base::defs::Board;
    use std::time::Instant;
    use crate::bot::search::generate_game_tree;

    #[test]
    fn test_perft() {
        let file_path = "sample/default.json";
        let mut curr_board: Option<Board> = Option::None;
        match Board::from_file( file_path ) {
            Ok( board ) => {
                curr_board = Some( board );
            }
            Err( e ) => {
                println!( "Error loading board: {}", e );
            }
        }

        let correct_num_nodes = [ 1, 20, 400, 8902, 197281, 4865609, 119060324 ];
        let mut curr_nodes = 0;

        if let Some(board) = curr_board {
            for max_depth in 0..7 {
                let mut num_nodes: u64 = 0;

                let start_time = Instant::now();
                generate_game_tree(board, max_depth, &mut num_nodes);
                let duration = start_time.elapsed();
                let duration_secs = duration.as_secs_f64();

                println!("Depth: {}", max_depth);
                println!("Number of Nodes Traversed: {}", num_nodes);
                println!("Time Taken: {:.2} seconds", duration_secs);
                println!("Nodes per second: {:.2}\n", num_nodes as f64 / duration_secs);

                curr_nodes += correct_num_nodes[ max_depth as usize ];
                assert_eq!( num_nodes, curr_nodes, "Correct Number of Nodes: {}, But Found: {}, for Depth: {}", num_nodes, curr_nodes, max_depth );
            }
        } else {
            println!("Failed to load the board, exiting.");
        }
    }
}

impl LegalMoveVec {
    pub fn new() -> Self {
        LegalMoveVec { data: Vec::new() }
    }

    pub fn push(&mut self, board: &mut Board) {
        if board.is_legal() {
            self.data.push(*board);
        }
    }

    pub fn len(&mut self) -> usize {
        self.data.len()
    }
}

impl Iterator for LegalMoveVec {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() {
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}