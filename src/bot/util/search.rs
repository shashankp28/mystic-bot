use std::sync::{ Arc, Mutex, atomic::{ AtomicBool, Ordering } };
use std::thread;

use chess::{ Board, ChessMove };

use crate::bot::{
    include::types::{
        RepetitionHistory,
        SearchContext,
        SearchHandle,
        SearchResult,
        TranspositionTable,
    },
    util::lookup::lookup_opening_db,
};

impl SearchHandle {
    pub fn start(board: Board, tt: TranspositionTable, history: RepetitionHistory) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let best = Arc::new(Mutex::new(None));

        let stop_clone = Arc::clone(&stop);
        let best_clone = Arc::clone(&best);
        let tt_clone = tt.clone(); // This now clones the internal Arc, which is what we want

        let handle = thread::spawn(move || {
            let mut context = SearchContext {
                board,
                history,
                tt: tt_clone, // This matches the type in SearchContext now
                stop_signal: stop_clone,
                killer_moves: [[None; 2]; 64],
                history_scores: [[0; 64]; 64],
                nodes_visited: 0,
            };

            Self::search_root(&mut context, best_clone);
        });

        SearchHandle {
            stop,
            best,
            handle: Some(handle),
        }
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }

    fn search_root(context: &mut SearchContext, best_out: Arc<Mutex<Option<SearchResult>>>) {
        if let Some(book_move) = lookup_opening_db(&context.board) {
            let result = SearchResult {
                best_move: book_move,
                pv: vec![book_move],
                eval: 0,
                depth: 0,
                nodes: 0,
            };

            *best_out.lock().unwrap() = Some(result);

            // Signal search completion
            context.stop_signal.store(true, Ordering::Release);
            return;
        }

        for depth in 1..=64 {
            if context.stop_signal.load(Ordering::Relaxed) {
                break;
            }

            // TODO: call negamax / alpha-beta here
            let dummy_move = ChessMove::new(chess::Square::A2, chess::Square::A4, None);

            let result = SearchResult {
                best_move: dummy_move,
                pv: vec![dummy_move],
                eval: 0,
                depth: depth as u8,
                nodes: context.nodes_visited,
            };

            *best_out.lock().unwrap() = Some(result);
        }
        context.stop_signal.store(true, Ordering::Release);
    }
}

pub fn estimate_search_time(time_left_ms: u128, time_limit_ms: Option<u128>) -> u128 {
    time_limit_ms.unwrap_or(time_left_ms / 40)
}
