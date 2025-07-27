use chess::{ Board, Piece };
use crate::bot::{ include::types::GlobalMap, util::board::BoardExt };

fn is_endgame(board: &Board) -> bool {
    // Simple heuristic: endgame if total material (excluding kings) is low
    let mut non_king_material = 0;

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            if piece == Piece::King {
                continue;
            }

            non_king_material += match piece {
                Piece::Pawn => 100,
                Piece::Knight => 320,
                Piece::Bishop => 330,
                Piece::Rook => 500,
                Piece::Queen => 900,
                _ => 0,
            };
        }
    }

    non_king_material < 1600 // adjust threshold as needed
}

pub fn evaluate_board(board: &Board) -> i32 {
    use chess::{ Piece::*, Color::* };

    // Fifty-move rule draw
    if board.halfmove_clock() >= 50 {
        return 0;
    }

    // Check for checkmate
    if board.status() == chess::BoardStatus::Checkmate {
        return if board.side_to_move() == White { -10_000 } else { 10_000 };
    }

    let mut white_total = 0;
    let mut black_total = 0;
    let mut white_bishops = 0;
    let mut black_bishops = 0;
    let mut white_knights = 0;
    let mut black_knights = 0;

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let color = board.color_on(sq).unwrap();

            match color {
                White => {
                    white_total += 1;
                    match piece {
                        Piece::Bishop => {
                            white_bishops += 1;
                        }
                        Piece::Knight => {
                            white_knights += 1;
                        }
                        _ => {}
                    }
                }
                Black => {
                    black_total += 1;
                    match piece {
                        Piece::Bishop => {
                            black_bishops += 1;
                        }
                        Piece::Knight => {
                            black_knights += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let is_minor_or_lone = |total: usize, bishops: usize, knights: usize| {
        match total {
            1 => true, // Only king
            2 if bishops == 1 => true, // King + bishop
            2 if knights == 1 => true, // King + knight
            3 if knights == 2 => true, // King + 2 knights
            _ => false,
        }
    };

    // Case 1: Both sides have only king/(bishop|knight|2 knights) → draw
    if
        is_minor_or_lone(white_total, white_bishops, white_knights) &&
        is_minor_or_lone(black_total, black_bishops, black_knights)
    {
        return 0;
    }

    // Case 2: One side has only king+bishop or king+knight; ignore its score
    let mut score: i32 = 0;
    let is_endgame = is_endgame(board);

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let color = board.color_on(sq).unwrap();
            let (rank, file) = (sq.get_rank().to_index(), sq.get_file().to_index());
            let (row, col) = match color {
                White => (rank, file),
                Black => (7 - rank, file),
            };

            let base = match piece {
                Pawn => 100,
                Knight => 320,
                Bishop => 330,
                Rook => 500,
                Queen => 900,
                King => 0,
            };

            let positional = match piece {
                Pawn => GlobalMap::PAWN_TABLE[row][col],
                Knight => GlobalMap::KNIGHT_TABLE[row][col],
                Bishop => GlobalMap::BISHOP_TABLE[row][col],
                Rook => GlobalMap::ROOK_TABLE[row][col],
                Queen => GlobalMap::QUEEN_TABLE[row][col],
                King => {
                    if is_endgame {
                        GlobalMap::KING_TABLE_END[row][col]
                    } else {
                        GlobalMap::KING_TABLE_START[row][col]
                    }
                }
            };

            let value = base + positional;

            if color == White {
                if !is_minor_or_lone(white_total, white_bishops, white_knights) {
                    score += value;
                }
            } else {
                if !is_minor_or_lone(black_total, black_bishops, black_knights) {
                    score -= value;
                }
            }
        }
    }

    score
}
