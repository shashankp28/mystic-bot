use crate::base::defs::{Board, CastleSide, PieceColour};
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
            | self.pawns & removal_map)
            != 0;
        removal_map = !removal_map;
        self.rooks &= removal_map;
        self.knights &= removal_map;
        self.bishops &= removal_map;
        self.queens &= removal_map;
        // self.kings &= removal_map; IF KING SHOULD BE REMOVED, SOMETHING IS WRONG!!
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

    pub fn mark_enpassant_possible_at(&mut self, x: u8) {
        // 4 bits for castling, 3 bits for en-passant x, 1 bit for possible or not
        self.metadata |= (1 << 7) as u32;
        self.metadata |= (x << 4) as u32;
    }

    pub fn is_enpassant_possible(&self) -> u32 {
        self.metadata & (1 << 7)
    }

    pub fn get_enpassant_x(&self) -> i8 {
        ((self.metadata >> 4) & 0b111) as i8
    }

    pub fn unmark_enpassant(&mut self) {
        self.metadata &= !(0b11110000)
    }

    pub fn hash(&self) -> u32 {
        let mut hasher = DefaultHasher::new();
        self.rooks.hash(&mut hasher);
        self.knights.hash(&mut hasher);
        self.bishops.hash(&mut hasher);
        self.queens.hash(&mut hasher);
        self.kings.hash(&mut hasher);
        self.pawns.hash(&mut hasher);
        self.metadata.hash(&mut hasher);
        hasher.finish() as u32
    }

    // TODO: Some Global Rules to take care of:
    //
    // 1. [ ] A legal move should be discarded, if after making the move current king is under check!!
    // 2. [ ] Castling can be done only in the following cases
    //      a. [ ] King and the corresponding rook shouldn't have moved
    //      b. [ ] The king should not be in check
    //      c. [ ] The squares the king moves through during castling should not be in check
    //      d. [ ] There should be no pieces between the king and the corresponding rook
    // 3. [ ] En-Passant can only be done, `ONLY IMMEDIATELY` after the opponent moves double step pawn
    // 4. [ ] Check is when the king is directly under threat
    // 5. [ ] Repeating a sequence of moves 3 times draws
    // 6. [ ] Checkmate is when king is under check and there are no legal moves (win/lose)
    // 7. [ ] Stalemate is when there are no legal moves, but the king is not in check (draw)
    // 8. [ ] Keep track and update the Half Move Clock
    // 9. [ ] Keep track and update the Full Move Number

    pub fn get_legal_moves(self) -> Vec<Board> {
        let mut legal_boards = Vec::new();

        // Generate all possible legal moves
        // self.generate_rook_moves(&mut legal_boards);
        // self.generate_knight_moves(&mut legal_boards);
        // self.generate_bishop_moves(&mut legal_boards);
        // self.generate_queen_moves(&mut legal_boards);
        self.generate_pawn_moves(&mut legal_boards);
        // self.generate_king_moves(&mut legal_boards);

        // Remove moves in which the king is in check
        // self.prune_illegal_moves(&mut legal_boards);

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
