use std::collections::HashSet;
use chess::{ Board, ChessMove, MoveGen, Piece };
use crate::bot::{ include::types::SpecialMove, util::piece::piece_value };

pub trait BoardExt {
    fn classify_move(&self, mv: ChessMove) -> HashSet<SpecialMove>;
    fn piece_moved(&self, mv: ChessMove) -> Option<Piece>;
    fn is_en_passant(&self, mv: ChessMove) -> bool;
    fn is_attack(&self, mv: ChessMove) -> bool;
    fn is_quiet_position(&self) -> bool;
    fn move_priority(&self, mv: ChessMove) -> i32;
    fn halfmove_clock(&self) -> u32;
    fn capture_pieces(&self, mv: ChessMove) -> Option<(Piece, Piece)>;
}

pub fn is_noisy(classification: &HashSet<SpecialMove>) -> bool {
    classification.contains(&SpecialMove::Check) ||
        classification.contains(&SpecialMove::Capture) ||
        classification.contains(&SpecialMove::Promotion) ||
        classification.contains(&SpecialMove::EnPassant)
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

    fn is_quiet_position(&self) -> bool {
        for mv in MoveGen::new_legal(self) {
            let tags = self.classify_move(mv);
            if is_noisy(&tags) {
                return false;
            }
        }
        true
    }

    fn move_priority(&self, mv: ChessMove) -> i32 {
        let mut has_check = false;
        let mut has_promotion = false;
        let mut has_capture = false;
        let mut is_noisy_flag = false;
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
        if is_noisy(&tags) {
            is_noisy_flag = true;
        }

        let mut tactical_bonus = 0;
        if is_noisy_flag {
            tactical_bonus += 50;
        }
        if has_check {
            tactical_bonus += 30;
        }
        if has_promotion {
            tactical_bonus += 80;
        }
        if has_capture {
            tactical_bonus += 40;
        }

        return tactical_bonus + capture_value_sum / 10;
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
}
