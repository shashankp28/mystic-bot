use chess::{ Board, ChessMove };
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
    pub history: RepetitionHistory,
    pub statistics: HashMap<u64, Statistics>,
    pub global_map: Arc<GlobalMap>,
    pub transposition_table: TranspositionTable,
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
    CastleKingside,
    CastleQueenside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone)]
pub struct TTEntry {
    pub value: i32,
    pub depth: u8,
    pub flag: BoundType,
    pub best_move: Option<ChessMove>,
}
#[derive(Debug, Clone)]
pub struct TranspositionTable {
    pub inner: Arc<Mutex<LruCache<(u64, u8), TTEntry>>>,
}

pub const TT_TABLE_SIZE: usize = 100_1000;

#[derive(Debug, Clone, Deserialize)]
pub struct OpeningEntry(pub String, pub u32);
pub type OpeningBook = HashMap<u64, Vec<OpeningEntry>>;

#[derive(Debug, Clone)]
pub struct RepetitionHistory {
    pub inner: HashMap<u64, u32>,
}

// CONSTANTS
// Piece base scores
pub const PAWN_BASE: i32 = 100;
pub const KNIGHT_BASE: i32 = 300;
pub const BISHOP_BASE: i32 = 350;
pub const ROOK_BASE: i32 = 500;
pub const QUEEN_BASE: i32 = 900;
pub const KING_BASE: i32 = 0;
pub const MATE_SCORE_BASE: i32 = 1_000_000;

// Move priority
pub const CHECK_BONUS: i32 = 80;
pub const CAPTURE_BONUS: i32 = 70;
pub const PROMOTION_BONUS: i32 = 60;
pub const CASTLING_BONUS: i32 = 50;

// King endgame
pub const KING_PROXIMITY_BASE: i32 = 11;
pub const KING_PROXIMITY_MAX_DISTANCE: i32 = 14;
pub const KING_PROXIMITY_SCORE_THRESHOLD: i32 = 300;

// Extra bonus
pub const CONNECTED_PAWN_BONUS: i32 = 5;
pub const PASSED_PAWN_BASE_BONUS: i32 = 5;
pub const PASSED_PAWN_RANK_MULTIPLIER: i32 = 5;
pub const DOUBLED_PAWN_PENALTY: i32 = 20;
pub const BISHOP_PAIR_BONUS: i32 = 40;
pub const NUM_PIECE_BONUS: i32 = 5;

// Misc.
pub const ENDGAME_MATERIALS: i32 = 1600;
pub const HALF_MOVE_DRAW_LIMIT: u32 = 100;
pub const BOARD_FILES: usize = 8;
pub const BOARD_RANKS: usize = 8;
pub const MAX_NOISE: f32 = 2500.0;
pub const MAX_THINK_TIME_MS: u128 = 40_000;
pub const QUIET_FALL_SHARPNESS: f32 = 1.0;
