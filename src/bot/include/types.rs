use std::collections::HashMap;
use std::sync::{ Arc, Mutex, atomic::AtomicBool };
use std::thread::JoinHandle;
use dashmap::DashMap;
use lru::LruCache;
use chess::{ Board, ChessMove };
use serde::Deserialize;

#[derive(Clone)]
pub struct ServerState {
    pub engines: Arc<DashMap<String, EngineState>>,
    pub global_map: Arc<GlobalMap>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub best_move: ChessMove,
    pub pv: Vec<ChessMove>,
    pub eval: i32,
    pub depth: u8,
    pub nodes: u64,
}

pub struct SearchHandle {
    pub stop: Arc<AtomicBool>,
    pub best: Arc<Mutex<Option<SearchResult>>>,
    pub handle: Option<JoinHandle<()>>,
}

pub struct SearchContext {
    pub board: Board,
    pub history: RepetitionHistory,
    pub tt: TranspositionTable, // Removed Arc wrapper here
    pub stop_signal: Arc<AtomicBool>,
    pub killer_moves: [[Option<ChessMove>; 2]; 64],
    pub history_scores: [[i32; 64]; 64],
    pub nodes_visited: u64,
}

pub const TT_TABLE_SIZE: usize = 1_048_576;

#[derive(Clone)]
pub struct TranspositionTable {
    pub inner: Arc<Mutex<LruCache<u64, TTEntry>>>,
}

#[derive(Debug, Clone, Copy)]
pub enum TTBound {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone, Copy)]
pub struct TTEntry {
    pub key: u16,
    pub value: i16,
    pub depth: u8,
    pub bound: TTBound,
    pub best_move: u16,
    pub age: u8,
}

#[derive(Debug, Clone)]
pub struct RepetitionHistory {
    pub inner: HashMap<u64, u32>,
}

pub struct EngineState {
    pub game_id: String,
    pub current_board: Board,
    pub history: RepetitionHistory,
    pub transposition_table: TranspositionTable,
    pub search: Option<SearchHandle>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpeningEntry(pub String, pub u32);
pub type OpeningBook = HashMap<u64, Vec<OpeningEntry>>;

#[derive(Debug)]
pub struct GlobalMap {}

// Piece Base value
pub const PAWN_BASE: i32 = 100;
pub const KNIGHT_BASE: i32 = 300;
pub const BISHOP_BASE: i32 = 350;
pub const ROOK_BASE: i32 = 500;
pub const QUEEN_BASE: i32 = 900;
pub const KING_BASE: i32 = 0;

// Phase Base value
pub const PAWN_PHASE: i32 = 100;
pub const KNIGHT_PHASE: i32 = 300;
pub const BISHOP_PHASE: i32 = 350;
pub const ROOK_PHASE: i32 = 500;
pub const QUEEN_PHASE: i32 = 900;
pub const KING_PHASE: i32 = 0;

pub const BISHOP_PAIR_BONUS: i32 = 40;
pub const DOUBLED_PAWN_PENALTY: i32 = 20;

pub const HALF_MOVE_DRAW_LIMIT: u32 = 100;
pub const MATE_SCORE_BASE: i32 = 1_000_000;

pub const ENDGAME_MATERIALS: i32 = 1400;

pub const INF: i32 = 1_000_000_000;
pub const MATE_SCORE: i32 = MATE_SCORE_BASE;
pub const MAX_PLY: usize = 64;


pub const LOGO: &str =
        r#"
 .--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--. 
/ .. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \
\ \/\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ \/ /
 \/ /`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'\/ / 
 / /\                                                                                                                                                                                                                                    / /\ 
/ /\ \            _____                _____                    _____                _____                    _____                    _____                            _____                   _______               _____             / /\ \
\ \/ /           /\    \              |\    \                  /\    \              /\    \                  /\    \                  /\    \                          /\    \                 /::\    \             /\    \            \ \/ /
 \/ /           /::\____\             |:\____\                /::\    \            /::\    \                /::\    \                /::\    \                        /::\    \               /::::\    \           /::\    \            \/ / 
 / /\          /::::|   |             |::|   |               /::::\    \           \:::\    \               \:::\    \              /::::\    \                      /::::\    \             /::::::\    \          \:::\    \           / /\ 
/ /\ \        /:::::|   |             |::|   |              /::::::\    \           \:::\    \               \:::\    \            /::::::\    \                    /::::::\    \           /::::::::\    \          \:::\    \         / /\ \
\ \/ /       /::::::|   |             |::|   |             /:::/\:::\    \           \:::\    \               \:::\    \          /:::/\:::\    \                  /:::/\:::\    \         /:::/~~\:::\    \          \:::\    \        \ \/ /
 \/ /       /:::/|::|   |             |::|   |            /:::/__\:::\    \           \:::\    \               \:::\    \        /:::/  \:::\    \                /:::/__\:::\    \       /:::/    \:::\    \          \:::\    \        \/ / 
 / /\      /:::/ |::|   |             |::|   |            \:::\   \:::\    \          /::::\    \              /::::\    \      /:::/    \:::\    \              /::::\   \:::\    \     /:::/    / \:::\    \         /::::\    \       / /\ 
/ /\ \    /:::/  |::|___|______       |::|___|______    ___\:::\   \:::\    \        /::::::\    \    ____    /::::::\    \    /:::/    / \:::\    \            /::::::\   \:::\    \   /:::/____/   \:::\____\       /::::::\    \     / /\ \
\ \/ /   /:::/   |::::::::\    \      /::::::::\    \  /\   \:::\   \:::\    \      /:::/\:::\    \  /\   \  /:::/\:::\    \  /:::/    /   \:::\    \          /:::/\:::\   \:::\ ___\ |:::|    |     |:::|    |     /:::/\:::\    \    \ \/ /
 \/ /   /:::/    |:::::::::\____\    /::::::::::\____\/::\   \:::\   \:::\____\    /:::/  \:::\____\/::\   \/:::/  \:::\____\/:::/____/     \:::\____\        /:::/__\:::\   \:::|    ||:::|____|     |:::|    |    /:::/  \:::\____\    \/ / 
 / /\   \::/    / ~~~~~/:::/    /   /:::/~~~~/~~      \:::\   \:::\   \::/    /   /:::/    \::/    /\:::\  /:::/    \::/    /\:::\    \      \::/    /        \:::\   \:::\  /:::|____| \:::\    \   /:::/    /    /:::/    \::/    /    / /\ 
/ /\ \   \/____/      /:::/    /   /:::/    /          \:::\   \:::\   \/____/   /:::/    / \/____/  \:::\/:::/    / \/____/  \:::\    \      \/____/          \:::\   \:::\/:::/    /   \:::\    \ /:::/    /    /:::/    / \/____/    / /\ \
\ \/ /               /:::/    /   /:::/    /            \:::\   \:::\    \      /:::/    /            \::::::/    /            \:::\    \                       \:::\   \::::::/    /     \:::\    /:::/    /    /:::/    /             \ \/ /
 \/ /               /:::/    /   /:::/    /              \:::\   \:::\____\    /:::/    /              \::::/____/              \:::\    \                       \:::\   \::::/    /       \:::\__/:::/    /    /:::/    /               \/ / 
 / /\              /:::/    /    \::/    /                \:::\  /:::/    /    \::/    /                \:::\    \               \:::\    \                       \:::\  /:::/    /         \::::::::/    /     \::/    /                / /\ 
/ /\ \            /:::/    /      \/____/                  \:::\/:::/    /      \/____/                  \:::\    \               \:::\    \                       \:::\/:::/    /           \::::::/    /       \/____/                / /\ \
\ \/ /           /:::/    /                                 \::::::/    /                                 \:::\    \               \:::\    \                       \::::::/    /             \::::/    /                               \ \/ /
 \/ /           /:::/    /                                   \::::/    /                                   \:::\____\               \:::\____\                       \::::/    /               \::/____/                                 \/ / 
 / /\           \::/    /                                     \::/    /                                     \::/    /                \::/    /                        \::/____/                 ~~                                       / /\ 
/ /\ \           \/____/                                       \/____/                                       \/____/                  \/____/                          ~~                                                               / /\ \
\ \/ /                                                                                                                                                                                                                                  \ \/ /
 \/ /   ASCII Art: www.asciiart.eu                                                                                                                                                                                                       \/ / 
 / /\.--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--..--./ /\ 
/ /\ \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \/\ \
\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `' /
 `--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'                                                                       
                                                                                                            MYSTIC BOT                                
                                                                                                Ready to make your move? Let's play!
"#;