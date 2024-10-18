use crate::base::defs::{Board, PieceType};



impl Board {

    const PAWN_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [5, 10, 10, -20, -20, 10, 10, 5],
        [5, -5, -10, 0, 0, -10, -5, 5],
        [0, 0, 0, 20, 20, 0, 0, 0],
        [5, 5, 10, 25, 25, 10, 5, 5],
        [10, 10, 20, 30, 30, 20, 10, 10],
        [50, 50, 50, 50, 50, 50, 50, 50],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const KNIGHT_TABLE: [[i32; 8]; 8] = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 0, 5, 5, 0, -20, -40],
        [-30, 5, 10, 15, 15, 10, 5, -30],
        [-30, 0, 15, 20, 20, 15, 0, -30],
        [-30, 5, 15, 20, 20, 15, 5, -30],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];

    const BISHOP_TABLE: [[i32; 8]; 8] = [
        [-20, -10, -10, -10, -10, -10, -10, -20],
        [-10, 5, 0, 0, 0, 0, 5, -10],
        [-10, 10, 10, 10, 10, 10, 10, -10],
        [-10, 0, 10, 10, 10, 10, 0, -10],
        [-10, 5, 5, 10, 10, 5, 5, -10],
        [-10, 0, 5, 10, 10, 5, 0, -10],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-20, -10, -10, -10, -10, -10, -10, -20],
    ];

    const ROOK_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 5, 5, 0, 0, 0],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [5, 10, 10, 10, 10, 10, 10, 5],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const QUEEN_TABLE: [[i32; 8]; 8] = [
        [-20, -10, -10, -5, -5, -10, -10, -20],
        [-10, 0, 5, 0, 0, 0, 0, -10],
        [-10, 5, 5, 5, 5, 5, 0, -10],
        [0, 0, 5, 5, 5, 5, 0, -5],
        [-5, 0, 5, 5, 5, 5, 0, -5],
        [-10, 0, 5, 5, 5, 5, 0, -10],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-20, -10, -10, -5, -5, -10, -10, -20],
    ];

    const KING_TABLE_START: [[i32; 8]; 8] = [
        [20, 30, 10, 0, 0, 10, 30, 20],
        [20, 20, 0, 0, 0, 0, 20, 20],
        [-10, -20, -20, -20, -20, -20, -20, -10],
        [-20, -30, -30, -40, -40, -30, -30, -20],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
    ];

    const KING_TABLE_END: [[i32; 8]; 8] = [
        [-50, -30, -30, -30, -30, -30, -30, -50],
        [-30, -30, 0, 0, 0, 0, -30, -30],
        [-30, -10, 20, 30, 30, 20, -10, -30],
        [-30, -10, 30, 40, 40, 30, -10, -30],
        [-30, -10, 30, 40, 40, 30, -10, -30],
        [-30, -10, 20, 30, 30, 20, -10, -30],
        [-30, -20, -10, 0, 0, -10, -20, -30],
        [-50, -40, -30, -20, -20, -30, -40, -50],
    ];

    pub fn get_piece_table(&self, piece_type: PieceType, is_end_game: bool) -> [[i32; 8]; 8] {
        match piece_type {
            PieceType::Pawn => Self::PAWN_TABLE,
            PieceType::Knight => Self::KNIGHT_TABLE,
            PieceType::Bishop => Self::BISHOP_TABLE,
            PieceType::Rook => Self::ROOK_TABLE,
            PieceType::Queen => Self::QUEEN_TABLE,
            PieceType::King => {
                if !is_end_game { Self::KING_TABLE_START }
                else { Self::KING_TABLE_END }
            },
        }
    }

    pub fn get_positional_scores(&self, piece_type: PieceType, piece_map: u64, is_black: u8, is_end_game: bool) -> f64 {
        let piece_table = self.get_piece_table(piece_type, is_end_game);
        let mut curr_piece_map = piece_map;
        let mut score = 0.0;
        while curr_piece_map != 0 {
            let pos: i8 = curr_piece_map.trailing_zeros() as i8;
            let index: i8 = (63 - pos) as i8;
            let x = (index % 8) as usize;
            let y = (index / 8) as usize;
            if is_black == 0 {
                score += piece_table[ x ][ y ] as f64;
            } else {
                score -= piece_table[ 7-x ][ 7-y ] as f64;
            }
            curr_piece_map &= !(1 << pos);
        }
        score
    }

    pub fn evaluate( &self ) -> f64 {

        let mut white_score = 0.0;
        white_score += ( ( self.queens as u64).count_ones()*900 ) as f64 ;
        white_score += ( ( self.rooks as u64).count_ones()*500 ) as f64 ;
        white_score += ( ( self.knights as u64).count_ones()*300 ) as f64 ;
        white_score += ( ( self.bishops as u64).count_ones()*300 ) as f64 ;
        white_score += ( ( self.pawns as u64).count_ones()*100 ) as f64 ;
        
        let mut black_score = 0.0;
        black_score -= ( ( self.queens >> 64 as u64).count_ones()*900 ) as f64 ;
        black_score -= ( ( self.rooks >> 64 as u64).count_ones()*500 ) as f64 ;
        black_score -= ( ( self.knights >> 64 as u64).count_ones()*300 ) as f64 ;
        black_score -= ( ( self.bishops >> 64 as u64).count_ones()*300 ) as f64 ;
        black_score -= ( ( self.pawns >> 64 as u64).count_ones()*100 ) as f64 ;

        let is_endgame = ( white_score-black_score )/2.0 < 1500.0;
        white_score += self.get_positional_scores(PieceType::King, self.kings as u64, 0, is_endgame);
        white_score += self.get_positional_scores(PieceType::Queen, self.queens as u64, 0, is_endgame);
        white_score += self.get_positional_scores(PieceType::Rook, self.rooks as u64, 0, is_endgame);
        white_score += self.get_positional_scores(PieceType::Bishop, self.bishops as u64, 0, is_endgame);
        white_score += self.get_positional_scores(PieceType::Knight, self.knights as u64, 0, is_endgame);
        white_score += self.get_positional_scores(PieceType::Pawn, self.pawns as u64, 0, is_endgame);

        black_score += self.get_positional_scores(PieceType::King, (self.kings >> 64) as u64, 1, is_endgame);
        black_score += self.get_positional_scores(PieceType::Queen, (self.queens >> 64) as u64, 1, is_endgame);
        black_score += self.get_positional_scores(PieceType::Rook, (self.rooks >> 64) as u64, 1, is_endgame);
        black_score += self.get_positional_scores(PieceType::Bishop, (self.bishops >> 64) as u64, 1, is_endgame);
        black_score += self.get_positional_scores(PieceType::Knight, (self.knights >> 64) as u64, 1, is_endgame);
        black_score += self.get_positional_scores(PieceType::Pawn, (self.pawns >> 64) as u64, 1, is_endgame);

        white_score + black_score
    }

}