use crate::ui::{
    formatting::{column::get_ellipse_string, padding::get_padded_string},
    prompts::prompt,
};
use colored::*;

pub fn page_header(text: &str, width: usize) {
    println!("{}", get_padded_string("▭", width, "▭").blue());
    println!(" {}", text.bright_yellow());
    println!("{}", get_padded_string("▭", width, "▭").blue());
}

pub fn page_footer(payload: &[&str], width: usize) {
    let block = payload.join("\n");
    println!("{}", get_padded_string("▭", width, "▭").blue());
    println!();
    println!("{}", block);
    println!();
    prompt("User => ", Color::Green);
}

pub fn table_header(payload: &[(&str, usize)], width: usize) {
    let columns: Vec<String> = payload
        .iter()
        .map(|(s, w)| {
            get_padded_string(&get_ellipse_string(s, *w), *w, " ")
                .yellow()
                .to_string()
        })
        .collect();

    let separator = " ║ ".blue().to_string();
    let full = columns.join(&separator);

    println!("{}", full);
    println!("{}", get_padded_string("═", width, "═").blue());
}

pub fn table_row(payload: &[(&str, usize)]) {
    let columns: Vec<String> = payload
        .iter()
        .map(|(s, w)| {
            get_padded_string(&get_ellipse_string(s, *w), *w, " ")
                .white()
                .to_string()
        })
        .collect();

    let separator = " ║ ".blue().to_string();
    let full = columns.join(&separator);

    println!("{}", full);
}
