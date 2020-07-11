use super::*;
use crate::cli::input;
use crate::data::*;

pub struct GamemodeMenu;

impl Menu for GamemodeMenu {
    fn update(&mut self) -> MenuTrans {
        println!("-------[NOUVELLE PARTIE]-------");
        println!("[1] Mode solo");
        println!("[2] Mode multi");
        println!("[3] Retour");

        let command = input::read_int_ranged("Que voulez-vous faire ?", 1, 3);
        match command {
            1 => MenuTrans::Change(MenuKind::Game, TransData::Mode(Gamemode::Solo)),
            2 => MenuTrans::Change(MenuKind::Game, TransData::Mode(Gamemode::Multi)),
            _ => MenuTrans::Change(MenuKind::Title, TransData::None),
        }
    }
}
