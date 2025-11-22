use chess::Board;
use std::time::{ Duration, Instant };
use crate::base::{ metadata::Metadata, moves::Move };

#[derive(Debug, Clone)]
pub struct EngineState {
    pub board: Board,
    pub metadata: Vec<Box<dyn Metadata>>,
    pub history: Vec<EngineState>,
    pub last_move: Option<Move>,
    pub time_created: Instant,
    pub time_left: Duration,
}

impl EngineState {
    pub fn make_move(&mut self, mv: Move) {
        self.history.push(self.clone());
        for m in &self.metadata {
            m.on_move(self, &mv);
        }
        self.board = self.board.make_move_new((&mv).into());
        self.last_move = Some(mv);
    }

    pub fn undo(&mut self) {
        assert!(!self.history.is_empty(), "No previous states to undo to!");
        let prev = self.history.pop().unwrap();
        *self = prev;
    }
}
