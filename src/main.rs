mod cli;

use cli::menu::{gamemode::GamemodeMenu, title::TitleMenu, Menu, MenuKind, MenuTrans};

struct Zen {
    curr_menu: Box<dyn Menu>,
}

impl Zen {
    fn clear(&self) {
        println!("{}", "\n".repeat(20));
    }

    fn change_menu(&mut self, new: MenuKind) {
        self.curr_menu = match new {
            MenuKind::Title => Box::new(TitleMenu),
            MenuKind::Gamemode => Box::new(GamemodeMenu),
        };

        self.clear();
    }

    fn run(&mut self) {
        loop {
            let trans = self.curr_menu.update();
            match trans {
                MenuTrans::Change(new) => self.change_menu(new),
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
