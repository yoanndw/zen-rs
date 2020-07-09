mod cli;

use cli::menu::Menu;

struct Zen {
    curr_menu: dyn Menu,
}

impl Zen {
    fn run(&mut self) {
        self.curr_menu.update();
    }
}

fn main() {
    println!("Hello, world!");
}
