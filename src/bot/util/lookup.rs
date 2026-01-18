use chess::{ Board, ChessMove };
use lru::LruCache;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::bot::include::map::OPENING_DB;
use crate::bot::include::types::{
    OpeningEntry,
    RepetitionHistory,
    SearchContext,
    TTBound,
    TTEntry,
    TranspositionTable,
};
use crate::bot::util::moves::parse_uci_move;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

impl RepetitionHistory {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, hash: u64) -> u32 {
        *self.inner.get(&hash).unwrap_or(&0)
    }

    pub fn set(&mut self, hash: u64, count: u32) {
        self.inner.insert(hash, count);
    }

    pub fn increment(&mut self, hash: u64) {
        *self.inner.entry(hash).or_insert(0) += 1;
    }

    pub fn decrement(&mut self, hash: u64) {
        if let Some(count) = self.inner.get_mut(&hash) {
            *count -= 1;
            if *count == 0 {
                self.inner.remove(&hash);
            }
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(LruCache::new(std::num::NonZeroUsize::new(size).unwrap()))),
        }
    }

    pub fn probe(&self, hash: u64, depth: u8, alpha: i32, beta: i32) -> Option<TTEntry> {
        let key = (hash & 0xffff) as u16;
        let mut table = self.inner.lock().unwrap();

        table
            .get(&hash)
            .copied()
            .filter(|e| {
                e.key == key &&
                    e.depth >= depth &&
                    (match e.bound {
                        TTBound::Exact => true,
                        TTBound::LowerBound => (e.value as i32) >= beta,
                        TTBound::UpperBound => (e.value as i32) <= alpha,
                    })
            })
    }

    pub fn store(
        &self,
        hash: u64,
        value: i32,
        depth: u8,
        alpha: i32,
        beta: i32,
        best_move: Option<ChessMove>
    ) {
        let bound = if value <= alpha {
            TTBound::UpperBound
        } else if value >= beta {
            TTBound::LowerBound
        } else {
            TTBound::Exact
        };

        let entry = TTEntry {
            key: (hash & 0xffff) as u16,
            value: value as i16,
            depth,
            bound,
            best_move: best_move.map(|m| m.get_dest().to_index() as u16).unwrap_or(0),
            age: 0,
        };

        self.inner.lock().unwrap().put(hash, entry);
    }
}

pub fn store_killer(context: &mut SearchContext, ply: usize, mv: ChessMove) {
    let killers = &mut context.killer_moves[ply];

    if killers[0] != Some(mv) {
        killers[1] = killers[0];
        killers[0] = Some(mv);
    }

    let src = mv.get_source().to_index();
    let dst = mv.get_dest().to_index();
    context.history_scores[src][dst] += 1;
}
