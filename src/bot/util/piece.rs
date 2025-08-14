use chess::Piece;
use crate::bot::include::types::*;

pub fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => PAWN_BASE,
        Piece::Knight => KNIGHT_BASE,
        Piece::Bishop => BISHOP_BASE,
        Piece::Rook => ROOK_BASE,
        Piece::Queen => QUEEN_BASE,
        Piece::King => KING_BASE,
    }
}
