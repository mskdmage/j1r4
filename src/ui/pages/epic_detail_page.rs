use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{formatting::*, pages::page::Page},
};
use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::any::Any;
use std::rc::Rc;

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        page_header("EPIC", 80);
        
        table_header(&vec![
            ("ID", 5),
            ("NAME", 25),
            ("DESCRIPTION", 25),
            ("STATUS", 15),
        ], 80);

        let id_info = &*self.epic_id.to_string();
        let name_info = &epic.name;
        let description_info = &epic.description.to_string();
        let status_info = &epic.status.to_string();

        table_row(&vec![
            (id_info, 5),
            (name_info, 25),
            (description_info, 25),
            (status_info, 25),
        ]);

        page_header("STORIES", 80);

        table_header(&vec![
            ("ID", 5),
            ("NAME", 50),
            ("STATUS", 15),
        ], 80);

        let stories = &epic.stories;

        stories.iter().sorted().for_each(|id| {
            if let Some(story) = db_state.stories.get(id) {
                let id_info = &*id.to_string();
                let name_info = &story.name.to_string();
                let status_info = &story.status.to_string();
                table_row(&vec![
                    (id_info, 5),
                    (name_info, 50),
                    (status_info, 25),
                ]);
            };
        });

        page_footer(&["[p] Previous", "[u] Update Epic", "[d] Delete Epic", "[c] Create Story", "[:id:] Navigate to Story"], 80);

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus {
                epic_id: self.epic_id,
            })),
            "d" => Ok(Some(Action::DeleteEpic {
                epic_id: self.epic_id,
            })),
            "c" => Ok(Some(Action::CreateStory {
                epic_id: self.epic_id,
            })),
            _ => match input.parse::<u32>() {
                Ok(id) => {
                    let stories = self.db.read_db()?.stories;
                    match stories.contains_key(&id) {
                        true => Ok(Some(Action::NavigateToStoryDetail {
                            epic_id: self.epic_id,
                            story_id: id,
                        })),
                        false => Ok(None),
                    }
                }
                Err(_) => Ok(None),
            },
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
