mod cli;

use cli::menu::{gamemode::GamemodeMenu, title::TitleMenu, Menu, MenuKind, MenuTrans};

struct Zen {
    curr_menu: Box<dyn Menu>,
}

impl Zen {
    fn run(&mut self) {
        loop {
            let trans = self.curr_menu.update();
            match trans {
                MenuTrans::Change(new) => match new {
                    MenuKind::Title => self.curr_menu = Box::new(TitleMenu),
                    MenuKind::Gamemode => self.curr_menu = Box::new(GamemodeMenu),
                },
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
