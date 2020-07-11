mod board;
mod cli;
mod data;

use cli::menu::{
    game::GameMenu, gamemode::GamemodeMenu, title::TitleMenu, Menu, MenuKind, MenuTrans,
};
use data::TransData;

struct Zen {
    curr_menu: Box<dyn Menu>,
}

impl Zen {
    fn clear(&self) {
        println!("{}", "\n".repeat(20));
    }

    fn change_menu(&mut self, new: MenuKind, data: TransData) {
        self.curr_menu = match new {
            MenuKind::Title => Box::new(TitleMenu),
            MenuKind::Gamemode => Box::new(GamemodeMenu),
            MenuKind::Game => Box::new(GameMenu::new(data)),
        };

        self.clear();
    }

    fn run(&mut self) {
        loop {
            let trans = self.curr_menu.update();
            match trans {
                MenuTrans::Change(new, data) => self.change_menu(new, data),
                MenuTrans::None => {}
                MenuTrans::Quit => break,
            }
        }
    }
}

fn main() {
    let mut zen = Zen {
        curr_menu: Box::new(TitleMenu),
    };
    //println!("Hello, world!");

    zen.run();
}
