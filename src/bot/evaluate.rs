use crate::base::defs::{ Board, GlobalMap, PieceType };

impl Board {
    pub fn get_piece_table(&self, piece_type: PieceType, is_end_game: bool) -> [[i32; 8]; 8] {
        match piece_type {
            PieceType::Pawn => GlobalMap::PAWN_TABLE,
            PieceType::Knight => GlobalMap::KNIGHT_TABLE,
            PieceType::Bishop => GlobalMap::BISHOP_TABLE,
            PieceType::Rook => GlobalMap::ROOK_TABLE,
            PieceType::Queen => GlobalMap::QUEEN_TABLE,
            PieceType::King => {
                if !is_end_game { GlobalMap::KING_TABLE_START } else { GlobalMap::KING_TABLE_END }
            }
        }
    }

    pub fn get_positional_scores(
        &self,
        piece_type: PieceType,
        piece_map: u64,
        is_black: u8,
        is_end_game: bool
    ) -> f64 {
        let piece_table = self.get_piece_table(piece_type, is_end_game);
        let mut curr_piece_map = piece_map;
        let mut score = 0.0;
        while curr_piece_map != 0 {
            let pos: i8 = curr_piece_map.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = (index % 8) as usize;
            let y = (index / 8) as usize;
            if is_black == 0 {
                score += piece_table[x][y] as f64;
            } else {
                score -= piece_table[7 - x][7 - y] as f64;
            }
            curr_piece_map &= !(1 << pos);
        }
        score
    }

    pub fn evaluate(&self, end: bool) -> f64 {
        // Half move clock has reached 100, is a draw
        if (self.metadata >> 9) & 0b1111111 >= 100 {
            return 0.0;
        }
        let is_black: u8 = if ((self.metadata >> 8) & 1) == 1 { 0 } else { 1 };
        let king_positions: u64 = (self.kings >> (64 * is_black)) as u64;
        if end {
            if self.can_attack(1 - is_black, king_positions) {
                return if is_black == 1 { 1000.0 } else { -1000.0 };
            } else {
                return 0.0;
            }
        }

        let mut white_score = 0.0;
        white_score += ((self.queens as u64).count_ones() * 900) as f64;
        white_score += ((self.rooks as u64).count_ones() * 500) as f64;
        white_score += ((self.knights as u64).count_ones() * 300) as f64;
        white_score += ((self.bishops as u64).count_ones() * 300) as f64;
        white_score += ((self.pawns as u64).count_ones() * 100) as f64;

        let mut black_score = 0.0;
        black_score -= ((self.queens >> (64 as u64)).count_ones() * 900) as f64;
        black_score -= ((self.rooks >> (64 as u64)).count_ones() * 500) as f64;
        black_score -= ((self.knights >> (64 as u64)).count_ones() * 300) as f64;
        black_score -= ((self.bishops >> (64 as u64)).count_ones() * 300) as f64;
        black_score -= ((self.pawns >> (64 as u64)).count_ones() * 100) as f64;

        let is_endgame = (white_score - black_score) / 2.0 < 1500.0;
        white_score += self.get_positional_scores(
            PieceType::King,
            self.kings as u64,
            0,
            is_endgame
        );
        white_score += self.get_positional_scores(
            PieceType::Queen,
            self.queens as u64,
            0,
            is_endgame
        );
        white_score += self.get_positional_scores(
            PieceType::Rook,
            self.rooks as u64,
            0,
            is_endgame
        );
        white_score += self.get_positional_scores(
            PieceType::Bishop,
            self.bishops as u64,
            0,
            is_endgame
        );
        white_score += self.get_positional_scores(
            PieceType::Knight,
            self.knights as u64,
            0,
            is_endgame
        );
        white_score += self.get_positional_scores(
            PieceType::Pawn,
            self.pawns as u64,
            0,
            is_endgame
        );

        black_score += self.get_positional_scores(
            PieceType::King,
            (self.kings >> 64) as u64,
            1,
            is_endgame
        );
        black_score += self.get_positional_scores(
            PieceType::Queen,
            (self.queens >> 64) as u64,
            1,
            is_endgame
        );
        black_score += self.get_positional_scores(
            PieceType::Rook,
            (self.rooks >> 64) as u64,
            1,
            is_endgame
        );
        black_score += self.get_positional_scores(
            PieceType::Bishop,
            (self.bishops >> 64) as u64,
            1,
            is_endgame
        );
        black_score += self.get_positional_scores(
            PieceType::Knight,
            (self.knights >> 64) as u64,
            1,
            is_endgame
        );
        black_score += self.get_positional_scores(
            PieceType::Pawn,
            (self.pawns >> 64) as u64,
            1,
            is_endgame
        );

        (white_score + black_score) / 100.0
    }
}
