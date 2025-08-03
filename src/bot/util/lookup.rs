use chess::{ Board, ChessMove };
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bot::include::map::OPENING_DB;
use crate::bot::include::types::OpeningEntry;
use crate::bot::util::moves::parse_uci_move;

pub fn lookup_opening_db(board: &Board) -> Option<ChessMove> {
    let board_hash = board.get_hash();
    let db = &OPENING_DB;

    if let Some(entries) = db.get(&board_hash) {
        let mut rng = thread_rng();
        if let Some(OpeningEntry(uci_str, _weight)) = entries.choose(&mut rng) {
            return parse_uci_move(uci_str, board);
        }
    }

    None
}
