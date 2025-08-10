use anyhow::{Context, Result, anyhow};
use std::rc::Rc;

use crate::{
    db::JiraDatabase,
    models::Action,
    ui::prompts::Prompts,
    ui::{EpicDetail, HomePage, Page, StoryDetail},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    db: Rc<JiraDatabase>,
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        Self {
            pages: vec![Box::new(HomePage { db: Rc::clone(&db) })],
            prompts: Prompts::new(),
            db,
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => {
                let instance = EpicDetail {
                    epic_id,
                    db: self.db.clone(),
                };
                self.pages.push(Box::new(instance));
            }
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                let instance = StoryDetail {
                    epic_id,
                    story_id,
                    db: self.db.clone(),
                };
                self.pages.push(Box::new(instance));
            }
            Action::NavigateToPreviousPage => {
                self.pages.pop();
            }
            Action::CreateEpic => {
                let epic = (self.prompts.create_epic)();
                self.db
                    .create_epic(epic)
                    .with_context(|| anyhow!("[!] Epic was not created!"))?;
            }
            Action::UpdateEpicStatus { epic_id } => {
                if let Some(s) = (self.prompts.update_status)() {
                    self.db
                        .update_epic_status(epic_id, s)
                        .with_context(|| anyhow!("[!] Epic was not updated!"))?
                };
            }
            Action::DeleteEpic { epic_id } => {
                if (self.prompts.delete_epic)() {
                    self.db
                        .delete_epic(epic_id)
                        .with_context(|| anyhow!("[!] Epic was not deleted!"))?;
                    self.pages.pop();
                };
            }
            Action::CreateStory { epic_id } => {
                let story = (self.prompts.create_story)();
                self.db
                    .create_story(story, epic_id)
                    .with_context(|| anyhow!("[!] Story could not be created!"))?;
            }
            Action::UpdateStoryStatus { story_id } => {
                if let Some(s) = (self.prompts.update_status)() {
                    self.db
                        .update_story_status(story_id, s)
                        .with_context(|| anyhow!("[!] Story could not be updated!"))?
                };
            }
            Action::DeleteStory { epic_id, story_id } => {
                if (self.prompts.delete_story)() {
                    self.db
                        .delete_story(epic_id, story_id)
                        .with_context(|| anyhow!("[!] Story was not deleted!"))?;
                    self.pages.pop();
                }
            }
            Action::Exit => {
                self.pages.clear();
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    #[allow(dead_code)]
    pub fn set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}
