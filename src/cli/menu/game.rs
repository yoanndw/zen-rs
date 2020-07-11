use super::*;
use crate::cli::input;
use crate::data::*;

pub struct GameMenu {
    mode: Gamemode,
}

impl GameMenu {
    pub fn new(data: TransData) -> Self {
        let mode = match data {
            TransData::Mode(mode) => mode,
            _ => Gamemode::Solo, // TODO: prevent the None with better type management
        };

        Self { mode }
    }
}

impl Menu for GameMenu {
    fn update(&mut self) -> MenuTrans {
        println!("-------[MODE {:?}]-------", self.mode);

        MenuTrans::Quit
    }
}
