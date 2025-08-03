use std::{ fs, io::{ self, Write }, path::Path, sync::Arc };

use flate2::read::GzDecoder;
use once_cell::sync::Lazy;
use tar::Archive;

use crate::bot::include::types::{ GlobalMap, OpeningBook };

const COMPRESSED_OPENING_DB: &[u8] = include_bytes!("../../data/openings.tar.gz");

pub fn read_opening_db() -> Result<OpeningBook, io::Error> {
    let output_dir = Path::new("./db");
    let compressed_path = output_dir.join("openings.tar.gz");
    let file_path = output_dir.join("openingDB.json");

    if !file_path.exists() {
        println!("OpeningDB not found, extracting...");

        fs::create_dir_all(output_dir)?;

        {
            let mut file = fs::File::create(&compressed_path)?;
            file.write_all(COMPRESSED_OPENING_DB)?;
        }

        let tar_file = fs::File::open(&compressed_path)?;
        let tar = GzDecoder::new(tar_file);
        let mut archive = Archive::new(tar);
        archive.unpack(output_dir)?;

        fs::remove_file(&compressed_path)?;
        println!("OpeningDB extracted to {:?}", file_path);
    }

    let file_content = fs::read_to_string(&file_path)?;
    let db: OpeningBook = serde_json
        ::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(db)
}
pub static OPENING_DB: Lazy<Arc<OpeningBook>> = Lazy::new(|| {
    Arc::new(read_opening_db().expect("Failed to load opening DB"))
});

impl GlobalMap {
    pub fn opening_db() -> Arc<OpeningBook> {
        Arc::clone(&OPENING_DB)
    }

    // NOTE: All these assume that Index 0 === a1 ( Top-Left of the board )
    pub const PAWN_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [5, 10, 10, -20, -20, 10, 10, 5],
        [5, -5, -10, 0, 0, -10, -5, 5],
        [0, 0, 0, 20, 20, 0, 0, 0],
        [5, 5, 10, 25, 25, 10, 5, 5],
        [10, 10, 20, 30, 30, 20, 10, 10],
        [50, 50, 50, 50, 50, 50, 50, 50],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    pub const KNIGHT_TABLE: [[i32; 8]; 8] = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 0, 5, 5, 0, -20, -40],
        [-30, 5, 10, 15, 15, 10, 5, -30],
        [-30, 0, 15, 20, 20, 15, 0, -30],
        [-30, 5, 15, 20, 20, 15, 5, -30],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];

    pub const BISHOP_TABLE: [[i32; 8]; 8] = [
        [-20, -10, -10, -10, -10, -10, -10, -20],
        [-10, 5, 0, 0, 0, 0, 5, -10],
        [-10, 10, 10, 10, 10, 10, 10, -10],
        [-10, 0, 10, 10, 10, 10, 0, -10],
        [-10, 5, 5, 10, 10, 5, 5, -10],
        [-10, 0, 5, 10, 10, 5, 0, -10],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-20, -10, -10, -10, -10, -10, -10, -20],
    ];

    pub const ROOK_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 5, 5, 0, 0, 0],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [-5, 0, 0, 0, 0, 0, 0, -5],
        [5, 10, 10, 10, 10, 10, 10, 5],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    pub const QUEEN_TABLE: [[i32; 8]; 8] = [
        [-20, -10, -10, -5, -5, -10, -10, -20],
        [-10, 0, 5, 0, 0, 0, 0, -10],
        [-10, 5, 5, 5, 5, 5, 0, -10],
        [0, 0, 5, 5, 5, 5, 0, -5],
        [-5, 0, 5, 5, 5, 5, 0, -5],
        [-10, 0, 5, 5, 5, 5, 0, -10],
        [-10, 0, 0, 0, 0, 0, 0, -10],
        [-20, -10, -10, -5, -5, -10, -10, -20],
    ];

    pub const KING_TABLE_START: [[i32; 8]; 8] = [
        [20, 30, 10, 0, 0, 10, 30, 20],
        [20, 20, 0, 0, 0, 0, 20, 20],
        [-10, -20, -20, -20, -20, -20, -20, -10],
        [-20, -30, -30, -40, -40, -30, -30, -20],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
        [-30, -40, -40, -50, -50, -40, -40, -30],
    ];

    pub const KING_TABLE_END: [[i32; 8]; 8] = [
        [-25, -23, -20, -20, -20, -20, -23, -25],
        [-23, -15, -5, -5, -5, -5, -15, -23],
        [-20, -5, 15, 20, 20, 15, -5, -20],
        [-20, -5, 20, 25, 25, 20, -5, -20],
        [-20, -5, 20, 25, 25, 20, -5, -20],
        [-20, -5, 15, 20, 20, 15, -5, -20],
        [-23, -15, -5, -5, -5, -5, -15, -23],
        [-25, -23, -20, -20, -20, -20, -23, -25],
    ];
}
