use std::fmt::Debug;
use crate::base::{ engine_state::EngineState, moves::Move };

pub trait Metadata: Debug {
    fn on_move(&self, engine: &EngineState, mv: &Move);

    fn clone_box(&self) -> Box<dyn Metadata>;
}

impl Clone for Box<dyn Metadata> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
