pub struct Board {
    pub white_rooks : u64,
    pub white_knights : u64,
    pub white_bishops : u64,
    pub white_queens : u64,
    pub white_king : u64,
    pub white_pawns : u64,

    pub black_rooks : u64,
    pub black_knights : u64,
    pub black_bishops : u64,
    pub black_queens : u64,
    pub black_king : u64,
    pub black_pawns : u64,

    pub en_passant : u16,
    pub castling_rights : u8,
}

fn main() {
    println!("Hello, world!");
}
