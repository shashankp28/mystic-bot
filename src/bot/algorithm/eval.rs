use chess::{Board, Color, Piece, Square};
use crate::bot::{include::types::*, util::board::BoardExt};

const MAX_PHASE: i32 = 24;

#[inline(always)]
fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn   => PAWN_BASE,
        Piece::Knight => KNIGHT_BASE,
        Piece::Bishop => BISHOP_BASE,
        Piece::Rook   => ROOK_BASE,
        Piece::Queen  => QUEEN_BASE,
        Piece::King   => KING_BASE,
    }
}

#[inline(always)]
fn piece_phase(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn   => 0,
        Piece::Knight => 1,
        Piece::Bishop => 1,
        Piece::Rook   => 2,
        Piece::Queen  => 4,
        Piece::King   => 0,
    }
}

#[inline(always)]
fn pst_value(piece: Piece, sq: Square, color: Color, phase: i32) -> i32 {
    let rank = sq.get_rank().to_index();
    let file = sq.get_file().to_index();

    let row = match color {
        Color::White => rank,
        Color::Black => 7 - rank,
    };

    match piece {
        Piece::Pawn   => GlobalMap::PAWN_TABLE[row][file],
        Piece::Knight => GlobalMap::KNIGHT_TABLE[row][file],
        Piece::Bishop => GlobalMap::BISHOP_TABLE[row][file],
        Piece::Rook   => GlobalMap::ROOK_TABLE[row][file],
        Piece::Queen  => GlobalMap::QUEEN_TABLE[row][file],

        // TAPERED KING PST — NO SWITCHING
        Piece::King => {
            let mg = GlobalMap::KING_TABLE_START[row][file];
            let eg = GlobalMap::KING_TABLE_END[row][file];
            (mg * phase + eg * (MAX_PHASE - phase)) / MAX_PHASE
        }
    }
}

fn evaluate_doubled_pawns(board: &Board) -> i32 {
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
                let penalty = DOUBLED_PAWN_PENALTY * (count as i32 - 1);
                score += if color == White { -penalty } else { penalty };
            }
        }
    }

    score
}

pub fn evaluate_board(board: &Board) -> i32 {
    use chess::{Color::*, Piece::*};

    // 50-move rule
    if board.halfmove_clock() >= HALF_MOVE_DRAW_LIMIT {
        return 0;
    }

    let mut score = 0;
    let mut phase = 0;
    let mut white_bishops = 0;
    let mut black_bishops = 0;

    // SINGLE board scan
    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let color = board.color_on(sq).unwrap();
            let sign = if color == White { 1 } else { -1 };

            score += sign * piece_value(piece);
            phase += piece_phase(piece);

            score += sign * pst_value(piece, sq, color, phase.min(MAX_PHASE));

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

    // Bishop pair (small, stable)
    if white_bishops >= 2 {
        score += BISHOP_PAIR_BONUS;
    }
    if black_bishops >= 2 {
        score -= BISHOP_PAIR_BONUS;
    }

    // Minimal pawn structure
    score += evaluate_doubled_pawns(board);

    score
}
