use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type BoardHash = u64;

#[derive(Debug)]
pub enum PieceColour {
    Black,
    White,
    Any,
}

pub enum CastleSide {
    King,
    Queen,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GameState {
    Checkmate,
    Stalemate,
    Playable,
}

#[derive(Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    // Flattended Matrix representation of 8x8 Chess Board, with `a1` at the Top-Left
    // Bit is 1 if the corresponding piece is at corresponding index else 0
    // The black and white parts of the boards are concatenated in 64+64 = 128 bits
    // The MSB part corresponds to black and LSB part corresponds to white
    // The below representation based on
    // Video: https://www.youtube.com/watch?v=w4FFX_otR-4&pp=ygUSbWFraW5nIGEgY2hlc3MgYm90
    pub rooks: u128,
    pub knights: u128,
    pub bishops: u128,
    pub queens: u128,
    pub kings: u128,
    pub pawns: u128,

    // 1 bit, whether the board has an en-passant
    // It is not possible for a board to have multiple en-passants at the same time!
    // ( [ X bits full move number ], [ 7 bits Half move clock ], is_white_move, en_passant_warn,
    //   [ 3 bits en_passant_column  ], Black o-o, Black o-o-o, White o-o, White o-o-o )
    //   --> 16 + fullmove_number / 32 bits used
    pub metadata: u32,

    // Will not be Hashed
    // [ 1 bit is_pawn_promotion ] [ 2 bits for Q, R, B, N promotion ],
    // [ 1 bit king-side castle ], [1 bit queen side castle], [ 6 bits for source ],
    // [ 6 bits for destination ] = 17 bits :(
    pub latest_move: u32,
}

#[derive(Debug)]
pub struct Search {
    pub board: Board,
    pub memory: HashMap<BoardHash, f64>,
    pub num_nodes: usize,
    pub max_depth: u32,
    pub num_prunes: u32,
}

#[derive(Debug)]
pub struct LegalMoveVec {
    pub data: Vec<Board>,
}
