use std::collections::HashMap;
use std::sync::{ Arc, Mutex, atomic::AtomicBool };
use std::thread::JoinHandle;
use dashmap::DashMap;
use lru::LruCache;
use chess::{ Board, ChessMove };

/* ==========================================================================
   1. SEARCH COMMUNICATION & CONTROL
   How the background search thread talks to the main engine/server.
   ========================================================================== */

/// Represents the "best guess" found by the search at any given moment.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub best_move: ChessMove, // The top-tier move found.
    pub pv: Vec<ChessMove>, // Principal Variation: The predicted line of play.
    pub eval: i32, // Position score in centipawns.
    pub depth: u8, // How deep the engine successfully searched.
    pub nodes: u64, // Total positions visited (for calculating Nodes Per Second).
}

/// A handle to a live search. Allows the main thread to stop or query the search.
pub struct SearchHandle {
    pub stop: Arc<AtomicBool>, // Thread-safe flag to trigger an immediate halt.
    pub best: Arc<Mutex<Option<SearchResult>>>, // Shared storage for the most recent complete depth.
    pub handle: JoinHandle<()>, // The actual system thread running the search.
}

/* ==========================================================================
   2. MEMORY & TRANSPOSITION TABLE
   The "Brain" of the engine that prevents it from recalculating the same work.
   ========================================================================== */

/// Number of entries in the Transposition Table.
/// 1,048,576 entries (2^20) is a good balance for 16GB-32GB RAM systems.
pub const TT_TABLE_SIZE: usize = 1_048_576;

/// A thread-safe, size-limited cache for storing evaluated board positions.
#[derive(Clone)]
pub struct TranspositionTable {
    pub inner: Arc<Mutex<LruCache<u64, TTEntry>>>, // Uses Zobrist hash as key.
}

/// A specific entry in the Transposition Table.
#[derive(Debug, Clone)]
pub struct TTEntry {
    pub value: i32, // The score for this position.
    pub depth: u8, // The depth used to reach this score.
    pub best_move: Option<ChessMove>, // The "Hash Move" - crucial for move ordering.
}

/* ==========================================================================
   3. GAME STATE & HISTORY
   Tracks a single game, including rules like 3-fold repetition.
   ========================================================================== */

/// Tracks board hashes to detect draws by repetition.
#[derive(Debug, Clone)]
pub struct RepetitionHistory {
    pub inner: HashMap<u64, u32>,
}

/// The complete container for an active chess game instance.
pub struct EngineState {
    pub game_id: String,
    pub current_board: Board,
    pub history: RepetitionHistory,
    pub transposition_table: TranspositionTable,
    pub search: Option<SearchHandle>, // Present only when the engine is "thinking".
}

/// Manages multiple concurrent games (used for a web server or multi-instance bot).
#[derive(Clone)]
pub struct ServerState {
    pub engines: Arc<DashMap<String, EngineState>>,
}

/* ==========================================================================
   4. EVALUATION CONSTANTS
   The fundamental weights used to score board positions.
   ========================================================================== */

// Piece values in centipawns (Pawn = 100 units).
pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 350;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 0;

// Positional Adjustments
pub const BISHOP_PAIR_BONUS: i32 = 40;
pub const DOUBLED_PAWN_PENALTY: i32 = 20;

// Game Rules & Search Bounds
pub const HALF_MOVE_DRAW_LIMIT: u32 = 100; // 50-move rule logic.
pub const MATE_SCORE_BASE: i32 = 1_000_000; // Starting point for mate scores.
