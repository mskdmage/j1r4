use crate::models::Status;
use crate::ui::prompts::{get_user_input, prompt};

pub fn update_status_prompt() -> Option<Status> {
    println!("{}", "NEW STATUS");
    println!("{}", "1 - OPEN");
    println!("{}", "2 - IN PROGRESS");
    println!("{}", "3 - RESOLVED");
    println!("{}", "4 - CLOSED");
    prompt(
        "User => ",
        colored::Color::BrightGreen,
    );
    let option = get_user_input();
    match option.trim().to_lowercase().as_str() {
        "1" => Some(Status::Open),
        "2" => Some(Status::InProgress),
        "3" => Some(Status::Resolved),
        "4" => Some(Status::Closed),
        _ => None,
    }
}
