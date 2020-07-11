use super::*;
use crate::cli::input;

pub struct TitleMenu;

impl Menu for TitleMenu {
    fn update(&mut self) -> MenuTrans {
        println!("-------[ZEN]-------");
        println!("[1] Nouvelle partie");
        println!("[2] Reprendre partie");
        println!("[3] Quitter");

        let command = input::read_int_ranged("Que voulez-vous faire ?", 1, 3);
        match command {
            1 => MenuTrans::Change(MenuKind::Gamemode),
            2 => MenuTrans::None,
            _ => MenuTrans::Quit,
        }
    }
}
