use chess::Board;
use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;

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
