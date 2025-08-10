mod epic;
mod status;
mod story;
mod utils;

use crate::models::{Epic, Status, Story};
pub use utils::*;

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(epic::create_epic_prompt),
            create_story: Box::new(story::create_story_prompt),
            delete_epic: Box::new(epic::delete_epic_prompt),
            delete_story: Box::new(story::delete_story_prompt),
            update_status: Box::new(status::update_status_prompt),
        }
    }
}
