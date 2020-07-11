use std::io::{self, Stdin, Write};

pub fn read_str(prompt: &str) -> String {
    print!("\n> {}", prompt);
    io::stdout().flush();

    let mut s = String::new();
    io::stdin().read_line(&mut s);

    s
}

pub fn read_int(prompt: &str) -> i32 {
    let mut s = read_str(prompt);
    while s.parse::<i32>().is_err() {
        s = read_str(prompt);
    }

    s.parse().unwrap()
}

pub fn read_int_ranged(prompt: &str, min: i32, max: i32) -> i32 {
    let mut i = read_int(prompt);
    while i < min || i > max {
        i = read_int(prompt);
    }

    i
}
