use chess::Board;
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

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
    pub statistics: HashMap<u32, Statistics>,
}

#[derive(Clone)]
pub struct ServerState {
    pub engines: Arc<Mutex<HashMap<String, EngineState>>>,
}
