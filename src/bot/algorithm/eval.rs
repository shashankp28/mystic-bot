use chess::{Board, Color, Piece, Square};
use crate::bot::{include::types::*, util::piece::{phase_value, piece_value}};

#[inline(always)]
fn pst_value(piece: Piece, sq: Square, color: Color, phase: i32) -> i32 {
    let rank = sq.get_rank().to_index();
    let file = sq.get_file().to_index();
    let row = if color == Color::White { rank } else { 7 - rank };

    match piece {
        Piece::Pawn   => GlobalMap::PAWN_TABLE[row][file],
        Piece::Knight => GlobalMap::KNIGHT_TABLE[row][file],
        Piece::Bishop => GlobalMap::BISHOP_TABLE[row][file],
        Piece::Rook   => GlobalMap::ROOK_TABLE[row][file],
        Piece::Queen  => GlobalMap::QUEEN_TABLE[row][file],

        Piece::King => {
            let mg = GlobalMap::KING_TABLE_START[row][file];
            let eg = GlobalMap::KING_TABLE_END[row][file];
            (mg * phase + eg * (MAX_PHASE - phase)) / MAX_PHASE
        }
    }
}

fn doubled_pawn_penalty(board: &Board) -> i32 {
    use chess::{Color::*, Piece::Pawn};

    let mut score = 0;

    for &color in &[White, Black] {
        let pawns = board.pieces(Pawn) & board.color_combined(color);
        let mut files = [0u8; 8];

        for sq in pawns {
            files[sq.get_file().to_index()] += 1;
        }

        for &count in &files {
            if count > 1 {
                let penalty = 12 * (count as i32 - 1);
                score += if color == White { -penalty } else { penalty };
            }
        }
    }

    score
}

pub fn evaluate_board(board: &Board) -> i32 {
    use chess::{Color::*, Piece::*};

    let mut score = 0;
    let mut phase = 0;

    let mut white_bishops = 0;
    let mut black_bishops = 0;

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let color = board.color_on(sq).unwrap();
            let sign = if color == White { 1 } else { -1 };

            score += sign * piece_value(piece);
            phase += phase_value(piece);

            if piece == Bishop {
                if color == White {
                    white_bishops += 1;
                } else {
                    black_bishops += 1;
                }
            }
        }
    }

    phase = phase.min(MAX_PHASE);

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let color = board.color_on(sq).unwrap();
            let sign = if color == White { 1 } else { -1 };
            score += sign * pst_value(piece, sq, color, phase);
        }
    }

    if white_bishops >= 2 {
        score += 30;
    }
    if black_bishops >= 2 {
        score -= 30;
    }

    score += doubled_pawn_penalty(board);

    score
}
