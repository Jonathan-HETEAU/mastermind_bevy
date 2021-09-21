use bevy::prelude::{FromWorld, World};
pub struct State {
    pub row: usize,
    pub code: [Option<mastermind::Color>; 4],
}

impl State {
    pub fn new() -> Self {
        State {
            row: 0,
            code: [Option::None; 4],
        }
    }
}

impl FromWorld for State {
    fn from_world(_world: &mut World) -> Self {
        State::new()
    }
}
