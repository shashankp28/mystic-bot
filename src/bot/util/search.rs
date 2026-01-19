use std::{ sync::{ Arc, Mutex, atomic::{ AtomicBool, Ordering } }, time::Instant };
use std::thread;
use tracing::{ info, debug };
use chess::Board;

use crate::bot::{
    algorithm::negamax::negamax,
    include::types::{
        INF,
        MAX_PLY,
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
        let tt_clone = tt.clone();

        let handle = thread::spawn(move || {
            let mut context = SearchContext {
                board,
                history,
                tt: tt_clone,
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

    pub fn search_root(context: &mut SearchContext, best_out: Arc<Mutex<Option<SearchResult>>>) {
        let start_time = Instant::now();

        if let Some(book_move) = lookup_opening_db(&context.board) {
            info!(move = %book_move, "Opening book hit");
            *best_out.lock().unwrap() = Some(SearchResult {
                best_move: book_move,
                pv: vec![book_move],
                eval: 0,
                depth: 0,
                nodes: 0,
            });
            context.stop_signal.store(true, Ordering::Release);
            return;
        }

        let mut alpha = -INF;
        let beta = INF;

        for depth in 1..=MAX_PLY {
            let (score, pv) = negamax(context, depth as i32, 0, alpha, beta);
            if context.stop_signal.load(Ordering::Relaxed) {
                debug!(depth, "Search stop signal received");
                break;
            }

            if let Some(&mv) = pv.first() {
                info!(
                depth,
                score,
                best_move = %mv,
                nodes = context.nodes_visited,
                time_ms = start_time.elapsed().as_millis(),
                "Depth completed"
            );

                *best_out.lock().unwrap() = Some(SearchResult {
                    best_move: mv,
                    pv: pv.clone(),
                    eval: score,
                    depth: depth as u8,
                    nodes: context.nodes_visited,
                });
            }

            alpha = alpha.max(score);
        }

        context.stop_signal.store(true, Ordering::Release);
    }
}

pub fn estimate_search_time(time_left_ms: u128, time_limit_ms: Option<u128>) -> u128 {
    time_limit_ms.unwrap_or(time_left_ms / 40)
}
