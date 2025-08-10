use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{formatting::*, pages::page::Page},
};
use anyhow::{Result, anyhow};
use std::any::Any;
use std::rc::Rc;

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let story = db_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("could not find story!"))?;

        page_header("STORY", 80);

        table_header(&vec![
            ("ID", 5),
            ("NAME", 25),
            ("DESCRIPTION", 25),
            ("STATUS", 15),
        ], 80);

        let id_info = &*self.story_id.to_string();
        let name_info = &story.name;
        let description_info = &story.description.to_string();
        let status_info = &story.status.to_string();

        table_row(&vec![
            (id_info, 5),
            (name_info, 25),
            (description_info, 25),
            (status_info, 25),
        ]);

        page_footer(&["[p] Previous", "[u] Update Story", "[d] Delete Story"], 80);

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus {
                story_id: self.story_id,
            })),
            "d" => Ok(Some(Action::DeleteStory {
                epic_id: self.epic_id,
                story_id: self.story_id,
            })),
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
