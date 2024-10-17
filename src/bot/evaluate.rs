use crate::base::defs::{Board, GameState, PieceType};



impl Board {

    const PAWN_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [50, 50, 50, 50, 50, 50, 50, 50],
        [10, 10, 20, 30, 30, 20, 10, 10],
        [5, 5, 10, 25, 25, 10, 5, 5],
        [0, 0, 0, 20, 20, 0, 0, 0],
        [5, -5, -10, 0, 0, -10, -5, 5],
        [5, 10, 10, -20, -20, 10, 10, 5],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const KNIGHT_TABLE: [[i32; 8]; 8] = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-30, 5, 15, 20, 20, 15, 5, -30],
        [-30, 0, 15, 20, 20, 15, 0, -30],
        [-30, 5, 10, 15, 15, 10, 5, -30],
        [-40, -20, 0, 5, 5, 0, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];

    const BISHOP_TABLE: [[i32; 8]; 8] = [
        [-20, -10, -10, -10, -10, -10, -10, -20],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-10, 0, 5, 10, 10, 5, 0, -10],
        [-10, 5, 5, 10, 10, 5, 5, -10],
        [-10, 0, 10, 10, 10, 10, 0, -10],
        [-10, 10, 10, 10, 10, 10, 10, -10],
        [-10, 5, 0, 0, 0, 0, 5, -10],
        [-20, -10, -10, -10, -10, -10, -10, -20],
    ];

    const ROOK_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [5, 10, 10, 10, 10, 10, 10, 5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [0, 0, 0, 5, 5, 0, 0, 0],
    ];

    const QUEEN_TABLE: [[i32; 8]; 8] = [
        [-20, -10, -10, -5, -5, -10, -10, -20],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-10, 0, 5, 5, 5, 5, 0, -10],
        [-5, 0, 5, 5, 5, 5, 0, -5],
        [0, 0, 5, 5, 5, 5, 0, -5],
        [-10, 5, 5, 5, 5, 5, 0, -10],
        [-10, 0, 5, 0, 0, 0, 0, -10],
        [-20, -10, -10, -5, -5, -10, -10, -20],
    ];

    const KING_TABLE_START: [[i32; 8]; 8] = [
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-20, -30, -30, -40, -40, -30, -30, -20],
        [-10, -20, -20, -20, -20, -20, -20, -10],
        [20, 20, 0, 0, 0, 0, 20, 20],
        [20, 30, 10, 0, 0, 10, 30, 20],
    ];

    const KING_TABLE_END: [[i32; 8]; 8] = [
        [-50, -40, -30, -20, -20, -30, -40, -50],
        [-30, -20, -10, 0, 0, -10, -20, -30],
        [-30, -10, 20, 30, 30, 20, -10, -30],
        [-30, -10, 30, 40, 40, 30, -10, -30],
        [-30, -10, 30, 40, 40, 30, -10, -30],
        [-30, -10, 20, 30, 30, 20, -10, -30],
        [-30, -30, 0, 0, 0, 0, -30, -30],
        [-50, -30, -30, -30, -30, -30, -30, -50],
    ];

    pub fn get_piece_table(&self, piece_type: PieceType) -> [[i32; 8]; 8] {
        let is_end_game = false;
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

    pub fn evaluate( &self ) -> f64 {
        let mut score: f64 = 0.0;
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };

        match self.get_game_state() {
            GameState::Checkmate => {
                if is_black==1 {
                    return 20000.0;
                } else {
                    return -20000.0;
                }
            },
            GameState::Stalemate => return 0.0,
            GameState::Playable => {}
        }

        score += ( ( self.queens as u64).count_ones()*900 ) as f64 ;
        score += ( ( self.rooks as u64).count_ones()*500 ) as f64 ;
        score += ( ( self.knights as u64).count_ones()*300 ) as f64 ;
        score += ( ( self.bishops as u64).count_ones()*300 ) as f64 ;
        score += ( ( self.pawns as u64).count_ones()*100 ) as f64 ;

        score -= ( ( self.queens >> 64 as u64).count_ones()*900 ) as f64 ;
        score -= ( ( self.rooks >> 64 as u64).count_ones()*500 ) as f64 ;
        score -= ( ( self.knights >> 64 as u64).count_ones()*300 ) as f64 ;
        score -= ( ( self.bishops >> 64 as u64).count_ones()*300 ) as f64 ;
        score -= ( ( self.pawns >> 64 as u64).count_ones()*100 ) as f64 ;

        score as f64
    }

}