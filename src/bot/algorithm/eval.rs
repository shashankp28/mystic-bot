use chess::{ Board, Color, File, Piece, Rank, Square };
use crate::bot::{ include::types::GlobalMap, util::{ board::BoardExt, piece::piece_value } };

fn distance_between(a: Square, b: Square) -> u8 {
    let file_distance = ((a.get_file().to_index() as i8) - (b.get_file().to_index() as i8)).abs();
    let rank_distance = ((a.get_rank().to_index() as i8) - (b.get_rank().to_index() as i8)).abs();
    (file_distance + rank_distance) as u8
}

fn evaluate_king_proximity(board: &Board, is_endgame: bool) -> i32 {
    if is_endgame {
        let white_king_sq = (board.pieces(Piece::King) & board.color_combined(Color::White))
            .into_iter()
            .next()
            .unwrap();
        let black_king_sq = (board.pieces(Piece::King) & board.color_combined(Color::Black))
            .into_iter()
            .next()
            .unwrap();

        let proximity = distance_between(white_king_sq, black_king_sq) as i32;
        let score = 11 + (14 - proximity);

        return score;
    }

    0
}

fn evaluate_connected_pawns(board: &Board) -> i32 {
    use chess::{ Color::*, Piece::Pawn };

    let mut score = 0;

    for &color in &[White, Black] {
        let pawns = board.pieces(Pawn) & board.color_combined(color);

        for sq in pawns {
            let rank = sq.get_rank().to_index();
            let file = sq.get_file().to_index();

            let connected = [-1, 1].iter().any(|&df| {
                let f = (file as isize) + df;
                if f < 0 || f > 7 {
                    return false;
                }

                // Check same rank, one ahead, and one behind
                [-1, 0, 1].iter().any(|&dr| {
                    let r = (rank as isize) + dr;
                    if r < 0 || r > 7 {
                        return false;
                    }

                    let adj_sq = chess::Square::make_square(
                        chess::Rank::from_index(r as usize),
                        chess::File::from_index(f as usize)
                    );
                    board.piece_on(adj_sq) == Some(Pawn) && board.color_on(adj_sq) == Some(color)
                })
            });

            if connected {
                score += match color {
                    White => 15,
                    Black => -15,
                };
            }
        }
    }

    score
}

pub fn evaluate_passed_pawns(board: &Board) -> i32 {
    use Color::{ White, Black };
    use Piece::Pawn;

    let mut score = 0;

    for &color in &[White, Black] {
        let pawns = board.pieces(Pawn) & board.color_combined(color);
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        for sq in pawns {
            let rank_idx = sq.get_rank().to_index();
            let file_idx = sq.get_file().to_index();

            // Check files: current, left, right
            let file_range = file_idx.saturating_sub(1)..=(file_idx + 1).min(7);

            let is_passed = file_range.clone().all(|f| {
                let file = File::from_index(f);
                match color {
                    White => {
                        // Check ahead of current rank
                        (rank_idx + 1..=7).all(|r| {
                            let sq = Square::make_square(Rank::from_index(r), file);
                            board.piece_on(sq) != Some(Pawn) ||
                                board.color_on(sq) != Some(opponent_color)
                        })
                    }
                    Black => {
                        // Check behind current rank
                        (0..rank_idx).all(|r| {
                            let sq = Square::make_square(Rank::from_index(r), file);
                            board.piece_on(sq) != Some(Pawn) ||
                                board.color_on(sq) != Some(opponent_color)
                        })
                    }
                }
            });

            if is_passed {
                score += match color {
                    White => 15,
                    Black => -15,
                };
            }
        }
    }

    score
}

pub fn evaluate_board(board: &Board) -> i32 {
    use chess::{ Piece::*, Color::* };

    // Fifty-move rule draw
    if board.halfmove_clock() >= 100 {
        return 0;
    }

    // Check for checkmate
    if board.status() == chess::BoardStatus::Checkmate {
        return if board.side_to_move() == White { -10_000 } else { 10_000 };
    } else if board.status() == chess::BoardStatus::Stalemate {
        return 0;
    }

    let white_base: i32 = board.material_score(chess::Color::White);
    let black_base = board.material_score(chess::Color::Black);

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

    let mut score: i32 = 0;
    let is_endgame = white_base + black_base < 1600;

    for sq in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            let color = board.color_on(sq).unwrap();
            let (rank, file) = (sq.get_rank().to_index(), sq.get_file().to_index());
            let (row, col) = match color {
                White => (rank, file),
                Black => (7 - rank, file),
            };

            let base = piece_value(piece);
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

            // Case 2: One side has only king+bishop or king+knight; ignore its base score
            if color == White {
                if !is_minor_or_lone(white_total, white_bishops, white_knights) {
                    score += base;
                }
                score += positional;
            } else {
                if !is_minor_or_lone(black_total, black_bishops, black_knights) {
                    score -= base;
                }
                score -= positional;
            }
        }
    }

    let proximity_score = evaluate_king_proximity(board, is_endgame);
    score += evaluate_connected_pawns(board);
    score += evaluate_passed_pawns(board);
    if white_base > black_base {
        score += proximity_score;
    } else {
        score -= proximity_score;
    }

    score
}
