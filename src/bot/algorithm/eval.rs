use chess::{ Board, ChessMove, Color, File, Piece, Rank, Square };
use crate::bot::{
    include::types::{ BoundType, EngineState, GlobalMap, TTEntry },
    util::{ board::BoardExt, piece::piece_value },
};

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
        // Closer kings in endgame is generally good
        return 14 - proximity;
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

                [-1, 0, 1].iter().any(|&dr| {
                    let r = (rank as isize) + dr;
                    if r < 0 || r > 7 {
                        return false;
                    }

                    let adj_sq = Square::make_square(
                        Rank::from_index(r as usize),
                        File::from_index(f as usize)
                    );
                    board.piece_on(adj_sq) == Some(Pawn) && board.color_on(adj_sq) == Some(color)
                })
            });

            if connected {
                score += if color == White { 5 } else { -5 };
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
        let opponent_color = if color == White { Black } else { White };

        for sq in pawns {
            let rank_idx = sq.get_rank().to_index();
            let file_idx = sq.get_file().to_index();

            let file_range = file_idx.saturating_sub(1)..=(file_idx + 1).min(7);

            let is_passed = file_range.clone().all(|f| {
                let file = File::from_index(f);
                match color {
                    White =>
                        (rank_idx + 1..=7).all(|r| {
                            let sq = Square::make_square(Rank::from_index(r), file);
                            board.piece_on(sq) != Some(Pawn) ||
                                board.color_on(sq) != Some(opponent_color)
                        }),
                    Black =>
                        (0..rank_idx).all(|r| {
                            let sq = Square::make_square(Rank::from_index(r), file);
                            board.piece_on(sq) != Some(Pawn) ||
                                board.color_on(sq) != Some(opponent_color)
                        }),
                }
            });

            if is_passed {
                let bonus = match color {
                    White => 5 + 5 * (rank_idx as i32),
                    Black => -5 - 5 * ((7 - rank_idx) as i32),
                };
                score += bonus;
            }
        }
    }

    score
}

pub fn evaluate_board(board: &Board) -> i32 {
    use chess::{ Piece::*, Color::* };

    if board.halfmove_clock() >= 100 {
        return 0;
    }

    let white_material = board.material_score(White);
    let black_material = board.material_score(Black);

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
                    if piece == Bishop {
                        white_bishops += 1;
                    } else if piece == Knight {
                        white_knights += 1;
                    }
                }
                Black => {
                    black_total += 1;
                    if piece == Bishop {
                        black_bishops += 1;
                    } else if piece == Knight {
                        black_knights += 1;
                    }
                }
            }
        }
    }

    let is_minor_or_lone = |total: usize, bishops: usize, knights: usize| {
        match total {
            1 => true,
            2 if bishops == 1 => true,
            2 if knights == 1 => true,
            3 if knights == 2 => true,
            _ => false,
        }
    };

    if
        is_minor_or_lone(white_total, white_bishops, white_knights) &&
        is_minor_or_lone(black_total, black_bishops, black_knights)
    {
        return 0;
    }

    let mut score = 0;
    let is_endgame = white_material + black_material < 1600;

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

    score += evaluate_connected_pawns(board);
    score += evaluate_passed_pawns(board);
    score += evaluate_king_proximity(board, is_endgame);

    score
}

pub fn is_terminal(
    board: &Board,
    board_hash: u64,
    depth: u8,
    current_depth: u8,
    color: i32,
    repetition_count: u32,
    engine_state: &mut EngineState
) -> Option<(Option<ChessMove>, i32)> {
    match board.status() {
        chess::BoardStatus::Checkmate => {
            let base_score = 1_000_000 - (current_depth as i32);
            let mate_score = if board.side_to_move() == chess::Color::White {
                -base_score
            } else {
                base_score
            };
            let score = mate_score * color;
            engine_state.transposition_table.put((board_hash, depth), TTEntry {
                value: score,
                depth,
                flag: BoundType::Exact,
                best_move: None,
            });
            Some((None, score))
        }
        chess::BoardStatus::Stalemate => {
            engine_state.transposition_table.put((board_hash, depth), TTEntry {
                value: 0,
                depth,
                flag: BoundType::Exact,
                best_move: None,
            });
            Some((None, 0))
        }
        _ if repetition_count >= 3 => {
            engine_state.transposition_table.put((board_hash, depth), TTEntry {
                value: 0,
                depth,
                flag: BoundType::Exact,
                best_move: None,
            });
            Some((None, 0))
        }
        _ => None,
    }
}
