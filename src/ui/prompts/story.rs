use crate::models::Story;
use crate::ui::prompts::{get_user_input, prompt};

pub fn create_story_prompt() -> Story {
    prompt("Story Name => ", colored::Color::BrightGreen);
    let name = get_user_input();
    prompt("Story Description => ", colored::Color::BrightGreen);
    let description = get_user_input();
    Story::new(name, description)
}

pub fn delete_story_prompt() -> bool {
    prompt(
        "Are you sure you want to delete this story? [Y/n] => ",
        colored::Color::BrightRed,
    );
    loop {
        let option = get_user_input();
        match option.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                prompt(
                    "Are you sure you want to delete this story? [Y/n] => ",
                    colored::Color::BrightRed,
                );
                continue;
            }
        }
    }
}
