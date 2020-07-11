pub mod gamemode;
pub mod title;

pub enum MenuKind {
    Title,
    Gamemode,
}

pub enum MenuTrans {
    Change(MenuKind),
    None,
    Quit,
}

pub trait Menu {
    fn update(&mut self) -> MenuTrans;
}
