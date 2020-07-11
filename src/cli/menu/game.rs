use super::*;
use crate::board::Board;
use crate::cli::input;
use crate::data::*;

pub struct GameMenu {
    mode: Gamemode,
    board: Board,
}

impl GameMenu {
    pub fn new(data: TransData) -> Self {
        let mode = match data {
            TransData::Mode(mode) => mode,
            _ => Gamemode::Solo, // TODO: prevent the None with better type management
        };

        Self {
            mode,
            board: Board::new(),
        }
    }
}

impl Menu for GameMenu {
    fn update(&mut self) -> MenuTrans {
        println!("-------[MODE {:?}]-------", self.mode);
        println!("{}", self.board);

        MenuTrans::Quit
    }
}
