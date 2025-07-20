use chess::{ Board, Piece };
use crate::bot::include::types::GlobalMap;

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

    // Check for checkmate
    if board.status() == chess::BoardStatus::Checkmate {
        return if board.side_to_move() == White { -10_000 } else { 10_000 };
    }

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

            // Use table from GlobalMap
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
                score += value;
            } else {
                score -= value;
            }
        }
    }

    score
}
