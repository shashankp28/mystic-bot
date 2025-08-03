use chess::{ Board, ChessMove };
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use dashmap::DashMap;
use lru::LruCache;

#[derive(Debug, Clone)]
pub struct Statistics {
    pub nodes_explored: u64,
    pub time_taken_ms: u128,
}

#[derive(Debug, Clone)]
pub struct EngineState {
    pub game_id: String,
    pub current_board: Board,
    pub history: RepetitionHistory,
    pub statistics: HashMap<u64, Statistics>,
    pub global_map: Arc<GlobalMap>,
    pub transposition_table: TranspositionTable,
}

#[derive(Debug)]
pub struct GlobalMap {}

#[derive(Clone)]
pub struct ServerState {
    pub engines: Arc<DashMap<String, EngineState>>,
    pub global_map: Arc<GlobalMap>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SpecialMove {
    Check,
    Capture,
    Attack,
    Promotion,
    EnPassant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone)]
pub struct TTEntry {
    pub value: i32,
    pub depth: u8,
    pub flag: BoundType,
    pub best_move: Option<ChessMove>,
}
#[derive(Debug, Clone)]
pub struct TranspositionTable {
    pub inner: Arc<Mutex<LruCache<(u64, u8), TTEntry>>>,
}

pub const TT_TABLE_SIZE: usize = 100_1000;

#[derive(Debug, Clone, Deserialize)]
pub struct OpeningEntry(pub String, pub u32);
pub type OpeningBook = HashMap<u64, Vec<OpeningEntry>>;

#[derive(Debug, Clone)]
pub struct RepetitionHistory {
    pub inner: HashMap<u64, u32>,
}
