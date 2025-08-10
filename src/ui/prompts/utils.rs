use colored::{Color, Colorize};
use std::io::{self, Write};

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input.trim().to_string()
}

pub fn prompt(text: &str, color: Color) {
    print!("{}", text.to_string().color(color));
    let _ = io::stdout().flush();
}

pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}
