use std::sync::{ Arc, Mutex, atomic::{ AtomicBool, Ordering } };
use std::thread;
use chess::Board;
use crate::bot::include::types::{
    SearchHandle,
    SearchResult,
    TranspositionTable,
    SearchContext,
    RepetitionHistory,
};

impl SearchHandle {
    pub fn start(board: Board, tt: TranspositionTable, mut history: RepetitionHistory) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let best = Arc::new(Mutex::new(None));

        let stop_clone = Arc::clone(&stop);
        let best_clone = Arc::clone(&best);

        let handle = thread::spawn(move || {
            let mut context = SearchContext {
                board,
                history: &mut history,
                tt: &tt,
                stop_signal: stop_clone,
                killer_moves: [[None; 2]; 64],
                history_scores: [[0; 64]; 64],
                nodes_visited: 0,
            };

            Self::iterative_deepening(&mut context, best_clone);
        });

        Self {
            stop,
            best,
            handle: Some(handle),
        }
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }

    fn iterative_deepening(
        context: &mut SearchContext,
        best_mutex: Arc<Mutex<Option<SearchResult>>>
    ) {
        for depth in 1..=255 {
            if context.stop_signal.load(Ordering::Relaxed) {
                break;
            }

            // Negamax/Alpha-Beta call would go here:
            // let score = negamax(context, depth, ...);

            let result = SearchResult {
                best_move: chess::ChessMove::new(chess::Square::A2, chess::Square::A4, None),
                pv: vec![],
                eval: 0,
                depth: depth as u8,
                nodes: context.nodes_visited,
            };

            let mut best_lock = best_mutex.lock().unwrap();
            *best_lock = Some(result);
            drop(best_lock);

            if context.stop_signal.load(Ordering::Relaxed) {
                break;
            }
        }
        context.stop_signal.store(true, Ordering::SeqCst)
    }
}

pub fn estimate_search_time(time_left_ms: u128, time_limit_ms: Option<u128>) -> u128 {
    time_limit_ms.unwrap_or(time_left_ms / 40)
}
