use chess::Board;
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
    pub history: HashMap<u64, u32>,
    pub statistics: HashMap<u64, Statistics>,
    pub global_map: Arc<GlobalMap>,
    pub transposition_table: Arc<Mutex<LruCache<(u64, u8), i32>>>,
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

pub const TT_TABLE_SIZE: usize = 100_1000;

#[derive(Debug, Clone, Deserialize)]
pub struct OpeningEntry(pub String, pub u32);
pub type OpeningBook = HashMap<u64, Vec<OpeningEntry>>;
