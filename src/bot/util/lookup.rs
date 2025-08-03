use chess::{ Board, ChessMove };
use lru::LruCache;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::bot::include::map::OPENING_DB;
use crate::bot::include::types::{ OpeningEntry, RepetitionHistory, TTEntry, TranspositionTable };
use crate::bot::util::moves::parse_uci_move;
use std::collections::HashMap;
use std::num::NonZeroUsize;
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
    pub fn new(capacity: usize) -> Self {
        let size = NonZeroUsize::new(capacity).expect("TT capacity must be > 0");
        Self {
            inner: Arc::new(Mutex::new(LruCache::new(size))),
        }
    }

    pub fn get(&self, key: &(u64, u8)) -> Option<TTEntry> {
        self.inner.lock().unwrap().get(key).cloned()
    }

    pub fn put(&self, key: (u64, u8), entry: TTEntry) {
        self.inner.lock().unwrap().put(key, entry);
    }

    pub fn clone_arc(&self) -> Arc<Mutex<LruCache<(u64, u8), TTEntry>>> {
        Arc::clone(&self.inner)
    }
}
