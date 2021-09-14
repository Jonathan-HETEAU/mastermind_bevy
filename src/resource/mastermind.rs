use bevy::prelude::{FromWorld, World};
use mastermind::Game;

pub struct Mastermind {
    pub state: Game,
}

impl FromWorld for Mastermind {
    fn from_world(_world: &mut World) -> Self {
        let game = Game::new();
        Mastermind { state: game }
    }
}


pub fn is_all_some(tab: &[Option<mastermind::Color>; 4]) -> bool {
    let mut bool = true;
    for code in tab.iter() {
        bool &= code.is_some();
    }
    bool
}

pub fn some_code_to_code(tab: &[Option<mastermind::Color>; 4]) -> mastermind::Code {
    let mut code: [mastermind::Color; 4] = [
        mastermind::Color::Black,
        mastermind::Color::Black,
        mastermind::Color::Black,
        mastermind::Color::Black,
    ];
    for i in 0..4 {
        if let Some(color) = &tab[i] {
            code[i] = color.clone();
        }
    }
    code as mastermind::Code
}