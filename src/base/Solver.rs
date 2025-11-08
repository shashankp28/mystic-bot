#[derive(Debug)]
pub struct Solver {
    pub max_threads: usize,
    pub thread_pool: ThreadPool,
    pub current_state: Arc<std::sync::Mutex<EngineState>>,
}
