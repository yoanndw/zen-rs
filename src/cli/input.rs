use std::io::{self, Stdin, Write};

pub struct Input {
    stdin: Stdin,
}

impl Input {
    pub fn new(stdin: Stdin) -> Self {
        Self { stdin }
    }

    pub fn read_str(&mut self, prompt: &str) -> String {
        print!("\n> {}", prompt);
        io::stdout().flush();

        let mut s = String::new();
        self.stdin.read_line(&mut s);

        s
    }

    pub fn read_int(&mut self, prompt: &str) -> i32 {
        let mut s = self.read_str(prompt);
        while s.parse::<i32>().is_err() {
            s = self.read_str(prompt);
        }

        s.parse().unwrap()
    }

    pub fn read_int_ranged(&mut self, prompt: &str, min: i32, max: i32) -> i32 {
        let mut i = self.read_int(prompt);
        while i < min || i > max {
            i = self.read_int(prompt);
        }

        i
    }
}
