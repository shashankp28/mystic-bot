use crate::base::defs::{Board, GameState};


impl Board {

    pub fn evaluate( &self ) -> f64 {
        let mut score: f64 = 0.0;
        let is_black: u8 = if ( self.metadata >> 8 ) & 1 == 1 { 0 } else { 1 };

        match self.get_game_state() {
            GameState::Checkmate => {
                if is_black==1 {
                    return 1000.0;
                } else {
                    return -1000.0;
                }
            },
            GameState::Stalemate => return 0.0,
            GameState::Playable => {}
        }

        score += ( ( self.queens as u64).count_ones()*9 ) as f64 ;
        score += ( ( self.rooks as u64).count_ones()*5 ) as f64 ;
        score += ( ( self.knights as u64).count_ones()*3 ) as f64 ;
        score += ( ( self.bishops as u64).count_ones()*3 ) as f64 ;
        score += ( ( self.pawns as u64).count_ones() ) as f64 ;

        score -= ( ( self.queens >> 64 as u64).count_ones()*9 ) as f64 ;
        score -= ( ( self.rooks >> 64 as u64).count_ones()*5 ) as f64 ;
        score -= ( ( self.knights >> 64 as u64).count_ones()*3 ) as f64 ;
        score -= ( ( self.bishops >> 64 as u64).count_ones()*3 ) as f64 ;
        score -= ( ( self.pawns >> 64 as u64).count_ones() ) as f64 ;

        score as f64
    }

}