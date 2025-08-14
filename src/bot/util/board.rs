use std::collections::HashSet;
use chess::{ Board, ChessMove, File, MoveGen, Piece };
use crate::bot::{
    include::types::{
        SpecialMove,
        CAPTURE_BONUS,
        CASTLING_BONUS,
        CHECK_BONUS,
        ENDGAME_MATERIALS,
        PROMOTION_BONUS,
    },
    util::piece::piece_value,
};

pub trait BoardExt {
    fn classify_move(&self, mv: ChessMove) -> HashSet<SpecialMove>;
    fn piece_moved(&self, mv: ChessMove) -> Option<Piece>;
    fn is_en_passant(&self, mv: ChessMove) -> bool;
    fn is_attack(&self, mv: ChessMove) -> bool;
    fn move_priority(&self, mv: ChessMove) -> i32;
    fn halfmove_clock(&self) -> u32;
    fn capture_pieces(&self, mv: ChessMove) -> Option<(Piece, Piece)>;
    fn material_score(&self, color: chess::Color) -> i32;
    fn noiseness(&self) -> i32;
    fn is_endgame(&self) -> bool;
}

impl BoardExt for Board {
    fn classify_move(&self, mv: ChessMove) -> HashSet<SpecialMove> {
        let mut result = HashSet::new();
        let new_board = self.make_move_new(mv);

        if new_board.checkers().popcnt() > 0 {
            result.insert(SpecialMove::Check);
        }

        if mv.get_promotion().is_some() {
            result.insert(SpecialMove::Promotion);
        }

        if self.piece_on(mv.get_dest()).is_some() {
            result.insert(SpecialMove::Capture);
        } else if self.is_en_passant(mv) {
            result.insert(SpecialMove::Capture);
            result.insert(SpecialMove::EnPassant);
        }

        if self.is_attack(mv) {
            result.insert(SpecialMove::Attack);
        }

        if let Some(piece) = self.piece_on(mv.get_source()) {
            if piece == Piece::King {
                let src_file = mv.get_source().get_file();
                let dst_file = mv.get_dest().get_file();
                // Assuming standard chess coordinates (file e to g or c)
                if src_file == File::E && dst_file == File::G {
                    result.insert(SpecialMove::CastleKingside);
                } else if src_file == File::E && dst_file == File::C {
                    result.insert(SpecialMove::CastleQueenside);
                }
            }
        }

        result
    }

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
        let mut has_check = false;
        let mut has_promotion = false;
        let mut has_capture = false;
        let mut has_castling = false;
        let mut capture_value_sum = 0;

        let tags = self.classify_move(mv);
        if tags.contains(&SpecialMove::Check) {
            has_check = true;
        }
        if tags.contains(&SpecialMove::Promotion) {
            has_promotion = true;
        }
        if tags.contains(&SpecialMove::Capture) {
            has_capture = true;
            let captured_value = if self.is_en_passant(mv) {
                piece_value(Piece::Pawn)
            } else {
                self.piece_on(mv.get_dest()).map(piece_value).unwrap_or(0)
            };
            capture_value_sum += captured_value;
        }
        if
            tags.contains(&SpecialMove::CastleKingside) ||
            tags.contains(&SpecialMove::CastleQueenside)
        {
            has_castling = true;
        }

        let mut tactical_bonus = 0;
        if has_check {
            tactical_bonus += CHECK_BONUS;
        }
        if has_promotion {
            tactical_bonus += PROMOTION_BONUS;
        }
        if has_capture {
            tactical_bonus += CAPTURE_BONUS;
        }
        if has_castling {
            tactical_bonus += CASTLING_BONUS;
        }

        tactical_bonus + capture_value_sum
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

    fn capture_pieces(&self, mv: ChessMove) -> Option<(Piece, Piece)> {
        if !self.classify_move(mv).contains(&SpecialMove::Capture) {
            return None;
        }

        let attacker = self.piece_on(mv.get_source())?;

        let victim = if self.is_en_passant(mv) {
            // For en passant, the victim is always one rank behind the destination
            let dest = mv.get_dest();
            let victim_sq = match self.side_to_move() {
                chess::Color::White => dest.down()?, // One rank below
                chess::Color::Black => dest.up()?, // One rank above
            };
            self.piece_on(victim_sq)?
        } else {
            self.piece_on(mv.get_dest())?
        };

        Some((attacker, victim))
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
