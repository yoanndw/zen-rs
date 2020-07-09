use super::*;

pub struct TitleMenu;

impl Menu for TitleMenu {
    fn update(&mut self) -> MenuTrans {
        println!("Title menu");
        MenuTrans::None
    }
}
