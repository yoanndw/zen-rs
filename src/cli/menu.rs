pub mod title;

pub enum MenuKind {
    Title,
}

pub enum MenuTrans {
    Change(MenuKind),
    None,
}

pub trait Menu {
    fn update(&mut self) -> MenuTrans;
}
