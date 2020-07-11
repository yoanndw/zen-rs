pub mod game;
pub mod gamemode;
pub mod title;

use crate::data::TransData;

pub enum MenuKind {
    Title,
    Gamemode,
    Game,
}

pub enum MenuTrans {
    Change(MenuKind, TransData),
    None,
    Quit,
}

pub trait Menu {
    fn update(&mut self) -> MenuTrans;
}
