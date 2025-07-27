use chess::{ Board, ChessMove, MoveGen, Piece, Square };
use std::str::FromStr;

pub fn parse_uci_move(uci: &str, board: &Board) -> Option<ChessMove> {
    if uci.len() < 4 {
        return None;
    }

    let from = Square::from_str(&uci[0..2]).ok()?;
    let to = Square::from_str(&uci[2..4]).ok()?;

    let promotion = if uci.len() == 5 {
        match uci.chars().nth(4)? {
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            _ => None,
        }
    } else {
        None
    };

    let candidate = ChessMove::new(from, to, promotion);

    let legal = MoveGen::new_legal(board);
    legal.into_iter().find(|m| *m == candidate)
}
