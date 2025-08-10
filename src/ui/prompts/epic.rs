use crate::models::Epic;
use crate::ui::prompts::{get_user_input, prompt};

pub fn create_epic_prompt() -> Epic {
    prompt("Epic Name => ", colored::Color::BrightGreen);
    let name = get_user_input();
    prompt("Epic Description => ", colored::Color::BrightGreen);
    let description = get_user_input();
    Epic::new(name, description)
}

pub fn delete_epic_prompt() -> bool {
    prompt(
        "Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n] => ",
        colored::Color::BrightRed,
    );
    loop {
        let option = get_user_input();
        match option.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                prompt(
                    "Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n] => ",
                    colored::Color::BrightRed,
                );
                continue;
            }
        }
    }
}
