mod db;
mod models;
mod navigator;
mod ui;

use crate::ui::prompts::{get_user_input, prompt, wait_for_key_press};
use std::rc::Rc;

fn main() {
    let db = Rc::new(db::JiraDatabase::new("./db.json".to_string()));
    let mut navigator = navigator::Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();
        if let Some(page) = navigator.get_current_page() {
            if let Err(e) = page.draw_page() {
                println!("[!] Error showing page! {}", e);
                prompt("Press any key to continue...", colored::Color::BrightBlue);
                wait_for_key_press();
            } else {
                let input = get_user_input();
                match page.handle_input(&input) {
                    Ok(res) => {
                        if let Some(action) = res {
                            match navigator.handle_action(action) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("[!] Error handling user input! {}", e);
                                    prompt(
                                        "Press any key to continue...",
                                        colored::Color::BrightBlue,
                                    );
                                    wait_for_key_press();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("[!] Error getting user input! {}", e);
                        prompt("Press any key to continue...", colored::Color::BrightBlue);
                        wait_for_key_press();
                    }
                }
            };
        } else {
            break;
        }
    }
}
