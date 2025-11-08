#[derive(Debug, Clone)]
pub struct Move {
    pub src_row: u8,
    pub src_col: u8,
    pub dest_row: u8,
    pub dest_col: u8,
    pub promotion: Option<char>, // e.g. 'Q' for queen
}
