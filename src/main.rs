mod cli;

use cli::menu::{title::TitleMenu, Menu};

struct Zen {
    curr_menu: Box<dyn Menu>,
}

impl Zen {
    fn run(&mut self) {
        self.curr_menu.update();
    }
}

fn main() {
    let mut zen = Zen {
        curr_menu: Box::new(TitleMenu),
    };
    //println!("Hello, world!");

    zen.run();
}
