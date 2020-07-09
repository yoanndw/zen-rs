use super::*;

pub struct TitleMenu;

impl Menu for TitleMenu {
    fn update(&mut self) -> MenuTrans {
        println!("-------[ZEN]-------");
        println!("[1] Nouvelle partie");
        println!("[2] Reprendre partie");
        println!("[3] Quitter");

        MenuTrans::None
    }
}
