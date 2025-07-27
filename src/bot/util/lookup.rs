use chess::{ Board, ChessMove };
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bot::include::types::GlobalMap;
use crate::bot::util::moves::parse_uci_move;

pub fn lookup_opening_db(board: &Board) -> Option<ChessMove> {
    let board_hash_str = board.get_hash().to_string();
    if let Some(opening_db) = GlobalMap::opening_db().as_object() {
        if let Some(entry_array) = opening_db.get(&board_hash_str).and_then(|v| v.as_array()) {
            let mut rng = thread_rng();
            if let Some(random_entry) = entry_array.choose(&mut rng) {
                if let Some(uci_str) = random_entry.get(0).and_then(|v| v.as_str()) {
                    if let Some(chess_move) = parse_uci_move(uci_str, board) {
                        println!("{}", chess_move);
                        return Some(chess_move);
                    }
                }
            }
        }
    }
    None
}
