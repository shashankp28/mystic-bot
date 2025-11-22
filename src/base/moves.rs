#[derive(Debug, Clone)]
pub struct Move {
    pub src_row: u8,
    pub src_col: u8,
    pub dest_row: u8,
    pub dest_col: u8,
    pub promotion: Option<char>, // e.g. 'Q' for queen
}

impl Move {
    pub fn to_uci(&self) -> String {
        let src_file = (b'a' + self.src_col) as char;
        let src_rank = (b'1' + self.src_row) as char;

        let dest_file = (b'a' + self.dest_col) as char;
        let dest_rank = (b'1' + self.dest_row) as char;

        let mut uci = format!("{}{}{}{}", src_file, src_rank, dest_file, dest_rank);

        if let Some(promo) = self.promotion {
            uci.push(promo.to_ascii_lowercase());
        }

        uci
    }

    pub fn from_uci(uci: &str) -> Option<Self> {
        let bytes = uci.as_bytes();

        if bytes.len() < 4 {
            return None;
        }

        let src_file = bytes[0];
        let src_rank = bytes[1];
        let dest_file = bytes[2];
        let dest_rank = bytes[3];

        if
            !(b'a'..=b'h').contains(&src_file) ||
            !(b'1'..=b'8').contains(&src_rank) ||
            !(b'a'..=b'h').contains(&dest_file) ||
            !(b'1'..=b'8').contains(&dest_rank)
        {
            return None;
        }

        let promotion = if bytes.len() == 5 { Some(bytes[4] as char) } else { None };

        Some(Move {
            src_col: src_file - b'a',
            src_row: src_rank - b'1',
            dest_col: dest_file - b'a',
            dest_row: dest_rank - b'1',
            promotion,
        })
    }
}

impl From<&Move> for chess::ChessMove {
    fn from(m: &Move) -> chess::ChessMove {
        use chess::{ ChessMove, Square, Rank, File, Piece };

        let from = Square::make_square(
            Rank::from_index(m.src_row as usize),
            File::from_index(m.src_col as usize)
        );

        let to = Square::make_square(
            Rank::from_index(m.dest_row as usize),
            File::from_index(m.dest_col as usize)
        );

        let promotion = m.promotion.map(|p| {
            match p.to_ascii_lowercase() {
                'q' => Piece::Queen,
                'r' => Piece::Rook,
                'b' => Piece::Bishop,
                'n' => Piece::Knight,
                _ => panic!("Invalid promotion '{}'", p),
            }
        });

        ChessMove::new(from, to, promotion)
    }
}
