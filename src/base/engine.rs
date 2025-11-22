use std::sync::Arc;
use rayon::ThreadPool;
use crate::base::engine_state::EngineState;

#[derive(Debug)]
pub struct Engine {
    pub max_threads: usize,
    pub thread_pool: ThreadPool,
    pub current_state: Arc<std::sync::Mutex<EngineState>>,
}
