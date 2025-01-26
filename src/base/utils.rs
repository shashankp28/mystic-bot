use crate::base::defs::{ Board, CastleSide, PieceColour, LegalMoveVec };
use fen::{ Color, PieceKind };
use serde_json::to_writer_pretty;
use std::fs::File;
use std::hash::DefaultHasher;
use std::hash::{ Hash, Hasher };
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
            PieceColour::Any => ((all_piece_map >> 64) as u64) | (all_piece_map as u64),
        }
    }

    pub fn remove_piece(&mut self, index: u8) -> bool {
        // Remove piece from bitMap if any piece exists at that index,
        // The logic of colour / legality of the move must be taken care
        // from the caller's side. Return True if a piece was actually removed
        let mut removal_map: u128 = 0;
        removal_map |= (1 << (63 - index)) | (1 << (127 - index));
        let piece_removed: bool =
            ((self.rooks & removal_map) |
                (self.knights & removal_map) |
                (self.bishops & removal_map) |
                (self.queens & removal_map) |
                (self.kings & removal_map) |
                (self.pawns & removal_map)) != 0;
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
        current_half_clock = if half_reset { 0 } else { current_half_clock + 1 };
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
                if ((self.rooks >> (64 as u64)) & (1 << (63 - index))) != 0 {
                    if index == 56 {
                        self.remove_castling_bits(CastleSide::Queen, colour);
                    } else if index == 63 {
                        self.remove_castling_bits(CastleSide::King, colour);
                    }
                }
            }
            PieceColour::White => {
                if ((self.rooks as u64) & (1 << (63 - index))) != 0 {
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
            PieceColour::White =>
                match side {
                    CastleSide::Queen => {
                        self.metadata &= !(1 << 0);
                    }
                    CastleSide::King => {
                        self.metadata &= !(1 << 1);
                    }
                }
            PieceColour::Black =>
                match side {
                    CastleSide::Queen => {
                        self.metadata &= !(1 << 2);
                    }
                    CastleSide::King => {
                        self.metadata &= !(1 << 3);
                    }
                }
            PieceColour::Any => {}
        }
    }

    pub fn set_enpassant(&mut self, x: Option<u8>) {
        // Clear the en-passant bits (bits 4-7)
        self.metadata &= !0b11110000;

        if let Some(pos) = x {
            // Set the en-passant bit (bit 7) and the position (bits 4-6)
            self.metadata |= (1 << 7) as u32; // Mark en-passant as possible
            self.metadata |= (pos << 4) as u32; // Set the en-passant column
        }
    }

    pub fn get_enpassant(&self) -> Option<i8> {
        if (self.metadata & (1 << 7)) != 0 {
            Some(((self.metadata >> 4) & 0b111) as i8)
        } else {
            None
        }
    }

    pub fn can_attack(&self, is_black: u8, mut targets: u64) -> bool {
        // is_black here is the opponent's colour
        while targets != 0 {
            // Can any opponent Bishop Attack target?
            // Equivalent: What if target was my bishop, can I reach any opp bishop?
            let pos = targets.trailing_zeros() as i8;
            let pseudo_bishop_map = self.get_bishop_move_bit_map(pos, 1 - is_black); // My Pseudo bishop map
            let opp_bishop_map = (self.bishops >> (64 * is_black)) as u64; // Actual opponent bishop map
            if (opp_bishop_map & pseudo_bishop_map) != 0 {
                return true;
            }

            // Similar logic applies queens, kings, knights, rooks, pawns (yes, even for pawns)
            // Queens
            let pseudo_queen_map = self.get_queen_move_bit_map(pos, 1 - is_black); // My Pseudo queen map
            let opp_queen_map = (self.queens >> (64 * is_black)) as u64; // Actual opponent queen map
            if (opp_queen_map & pseudo_queen_map) != 0 {
                return true;
            }

            // Knights
            let pseudo_knight_map = self.get_knight_move_bit_map(pos, 1 - is_black); // My Pseudo knight map
            let opp_knights_map = (self.knights >> (64 * is_black)) as u64; // Actual opponent knight map
            if (opp_knights_map & pseudo_knight_map) != 0 {
                return true;
            }

            // Rooks
            let pseudo_rook_map = self.get_rook_move_bit_map(pos, 1 - is_black); // My Pseudo rook map
            let opp_rooks_map = (self.rooks >> (64 * is_black)) as u64; // Actual opponent rook map
            if (opp_rooks_map & pseudo_rook_map) != 0 {
                return true;
            }

            // Kings
            let pseudo_king_map = self.get_king_attack_bit_map(pos, 1 - is_black); // My Pseudo king map
            let opp_kings_map = (self.kings >> (64 * is_black)) as u64; // Actual opponent king map
            if (opp_kings_map & pseudo_king_map) != 0 {
                return true;
            }

            // Pawns
            let pseudo_pawn_map = self.get_pawn_attack_bit_map(pos, 1 - is_black); // My Pseudo pawn map
            let opp_pawns_map = (self.pawns >> (64 * is_black)) as u64; // Actual opponent pawn map
            if (opp_pawns_map & pseudo_pawn_map) != 0 {
                return true;
            }
            targets &= !(1 << pos);
        }

        return false;
    }

    pub fn is_legal(&self) -> bool {
        let prev_was_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 1 } else { 0 };
        let king_position: u64 = (self.kings >> (64 * prev_was_black)) as u64;
        return !self.can_attack(1 - prev_was_black, king_position);
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
        let mut combined_moves = LegalMoveVec::new();

        // Generate moves for each piece type sequentially
        combined_moves.extend(self.generate_rook_moves());
        combined_moves.extend(self.generate_knight_moves());
        combined_moves.extend(self.generate_bishop_moves());
        combined_moves.extend(self.generate_queen_moves());
        combined_moves.extend(self.generate_pawn_moves());
        combined_moves.extend(self.generate_king_moves());

        combined_moves
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

    pub fn from_fen(fen_string: &String) -> Option<Board> {
        let result = fen::BoardState::from_fen(fen_string);
        match result {
            Ok(fen_board) => {
                let mut board = Board {
                    rooks: 0,
                    knights: 0,
                    bishops: 0,
                    queens: 0,
                    kings: 0,
                    pawns: 0,
                    metadata: 0,
                    latest_move: 0,
                };
                for index in 0..64 {
                    if let Some(piece) = fen_board.pieces.get(index).unwrap() {
                        let piece_black = if piece.color == Color::Black { 1 } else { 0 };
                        let offset = 63 - index + piece_black * 63;
                        match piece.kind {
                            PieceKind::King => {
                                board.kings |= 1 << offset;
                            }
                            PieceKind::Queen => {
                                board.queens |= 1 << offset;
                            }
                            PieceKind::Rook => {
                                board.rooks |= 1 << offset;
                            }
                            PieceKind::Knight => {
                                board.knights |= 1 << offset;
                            }
                            PieceKind::Bishop => {
                                board.bishops |= 1 << offset;
                            }
                            PieceKind::Pawn => {
                                board.pawns |= 1 << offset;
                            }
                        }
                    }
                }
                let is_white_move = if fen_board.side_to_play == Color::White { 1 } else { 0 };
                let white_ooo = if fen_board.white_can_ooo { 1 } else { 0 };
                let white_oo = if fen_board.white_can_oo { 1 } else { 0 };
                let black_ooo = if fen_board.black_can_ooo { 1 } else { 0 };
                let black_oo = if fen_board.black_can_oo { 1 } else { 0 };

                board.metadata |= white_ooo << 0;
                board.metadata |= white_oo << 1;
                board.metadata |= black_ooo << 2;
                board.metadata |= black_oo << 3;
                if let Some(ep_square) = fen_board.en_passant_square {
                    board.metadata |= ((ep_square as u32) & 7) << 4;
                    board.metadata |= 1 << 7;
                }
                board.metadata |= is_white_move << 8;
                board.metadata |= ((fen_board.halfmove_clock as u32) & 127) << 9;
                board.metadata |= (fen_board.fullmove_number as u32) << 16;
                Some(board)
            }
            Err(error) => {
                println!("Error Parsing fen: {:?}", error);
                None
            }
        }
    }
}

pub fn uci_to_uint(uci: &str) -> u16 {
    let mut result: u16 = 0;

    result |= ((uci.chars().nth(1).unwrap().to_digit(10).unwrap() as u16) - 1) << 9;
    result |= ((uci.chars().nth(0).unwrap() as u16) - ('a' as u16)) << 6;
    result |= ((uci.chars().nth(3).unwrap().to_digit(10).unwrap() as u16) - 1) << 3;
    result |= (uci.chars().nth(2).unwrap() as u16) - ('a' as u16);
    result &= (1 << 12) - 1;

    // Handle promotion piece if present (uci[4])
    if uci.len() == 5 {
        match uci.chars().nth(4).unwrap() {
            'Q' => {
                result |= 4 << 12;
            } // Queen promotion
            'R' => {
                result |= 5 << 12;
            } // Rook promotion
            'B' => {
                result |= 6 << 12;
            } // Bishop promotion
            'N' => {
                result |= 7 << 12;
            } // Knight promotion
            _ => {}
        }
    }
    result
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

impl LegalMoveVec {
    pub fn new() -> Self {
        LegalMoveVec { data: Vec::new() }
    }

    pub fn push(&mut self, board: &mut Board) {
        if board.is_legal() {
            self.data.push(*board);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn choose(&self, index: usize) -> Option<&Board> {
        self.data.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<Board> {
        self.data.iter()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn extend(&mut self, move_vec: LegalMoveVec) {
        self.data.extend(move_vec.data);
    }
}

impl Iterator for LegalMoveVec {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() { Some(self.data.remove(0)) } else { None }
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
        match Board::from_file(file_path) {
            Ok(board) => {
                curr_board = Some(board);
            }
            Err(e) => {
                println!("Error loading board: {}", e);
            }
        }

        let correct_num_nodes = [1, 20, 400, 8902, 197281, 4865609, 119060324];
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
                println!("Nodes per second: {:.2}\n", (num_nodes as f64) / duration_secs);

                curr_nodes += correct_num_nodes[max_depth as usize];
                assert_eq!(
                    num_nodes,
                    curr_nodes,
                    "Correct Number of Nodes: {}, But Found: {}, for Depth: {}",
                    num_nodes,
                    curr_nodes,
                    max_depth
                );
            }
        } else {
            println!("Failed to load the board, exiting.");
        }
    }
}
