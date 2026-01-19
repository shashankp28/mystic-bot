use chess::{ Board, ChessMove, MoveGen, Piece };
use crate::bot::{ include::types::ENDGAME_MATERIALS, util::piece::piece_value };

pub trait BoardExt {
    fn piece_moved(&self, mv: ChessMove) -> Option<Piece>;
    fn is_en_passant(&self, mv: ChessMove) -> bool;
    fn is_attack(&self, mv: ChessMove) -> bool;
    fn move_priority(&self, mv: ChessMove) -> i32;
    fn halfmove_clock(&self) -> u32;
    fn material_score(&self, color: chess::Color) -> i32;
    fn noiseness(&self) -> i32;
    fn is_endgame(&self) -> bool;
}

impl BoardExt for Board {
    fn piece_moved(&self, mv: ChessMove) -> Option<Piece> {
        self.piece_on(mv.get_source())
    }

    fn is_en_passant(&self, mv: ChessMove) -> bool {
        if self.piece_on(mv.get_source()) != Some(Piece::Pawn) {
            return false;
        }
        if self.piece_on(mv.get_dest()).is_some() {
            return false;
        }
        match self.en_passant() {
            Some(ep_sq) if ep_sq == mv.get_dest() => true,
            _ => false,
        }
    }

    fn is_attack(&self, mv: ChessMove) -> bool {
        let dest = mv.get_dest();
        match self.color_on(dest) {
            Some(color) if color != self.side_to_move() => true,
            _ => false,
        }
    }

    fn move_priority(&self, mv: ChessMove) -> i32 {
        let mut score = 0;
        let source_piece = self.piece_on(mv.get_source());
        let dest_piece = self.piece_on(mv.get_dest());

        if let Some(victim) = dest_piece {
            score +=
                10000 + piece_value(victim) - piece_value(source_piece.unwrap_or(Piece::Pawn)) / 10;
        }

        if self.is_en_passant(mv) {
            score += 10000 + piece_value(Piece::Pawn);
        }

        if let Some(promo) = mv.get_promotion() {
            score += 8000 + piece_value(promo);
        }

        if source_piece == Some(Piece::King) {
            let src_file = mv.get_source().get_file();
            let dst_file = mv.get_dest().get_file();
            if ((src_file as i32) - (dst_file as i32)).abs() > 1 {
                score += 500;
            }
        }

        score
    }

    fn noiseness(&self) -> i32 {
        MoveGen::new_legal(self)
            .map(|mv| self.move_priority(mv))
            .sum()
    }

    fn halfmove_clock(&self) -> u32 {
        self.to_string()
            .split_whitespace()
            .nth(4)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0)
    }

    fn material_score(&self, color: chess::Color) -> i32 {
        let mut score = 0;

        for sq in chess::ALL_SQUARES {
            if self.color_on(sq) == Some(color) {
                if let Some(piece) = self.piece_on(sq) {
                    score += piece_value(piece);
                }
            }
        }

        score
    }

    fn is_endgame(&self) -> bool {
        let white_material = self.material_score(chess::Color::White);
        let black_material = self.material_score(chess::Color::Black);
        return white_material + black_material < ENDGAME_MATERIALS;
    }
}
