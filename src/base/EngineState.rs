use chess::Board;
use std::sync::Arc;
use std::time::{ Duration, Instant };
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
pub struct EngineState {
    pub board: Board,
    pub metadata: Metadata,
    pub history: Vec<EngineState>, // stack of previous states (copies)
    pub last_move: Option<Move>,
    pub time_created: Instant,
    pub time_left: Duration,
}
