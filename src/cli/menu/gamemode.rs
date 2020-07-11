use super::*;
use crate::cli::input;

pub struct GamemodeMenu;

impl Menu for GamemodeMenu {
    fn update(&mut self) -> MenuTrans {
        println!("-------[NOUVELLE PARTIE]-------");

        MenuTrans::None
    }
}
