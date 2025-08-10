use crate::db::{Database, json_file_database::JsonFileDatabase};
use crate::models::{DBState, Epic, Status, Story};
use anyhow::{Result, anyhow};

pub struct JiraDatabase {
    pub database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JsonFileDatabase::new(file_path)),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut state = self
            .read_db()
            .map_err(|_| anyhow!("[!] Database could not be read."))?;

        state.last_item_id += 1;
        state.epics.insert(state.last_item_id, epic);

        self.database
            .write_db(&state)
            .map_err(|_| anyhow!("[!] Could not write epic to disk!"))?;

        Ok(state.last_item_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut state = self
            .read_db()
            .map_err(|_| anyhow!("[!] Database could not be read."))?;

        let epic_exists = state.epics.get_mut(&epic_id);

        if let Some(epic) = epic_exists {
            state.last_item_id += 1;

            state.stories.insert(state.last_item_id, story);

            epic.stories.push(state.last_item_id);

            self.database
                .write_db(&state)
                .map_err(|_| anyhow!("[!] Could not write story to disk!"))?;

            Ok(state.last_item_id)
        } else {
            Err(anyhow!("[!] Epic {} does not exist.", epic_id))
        }
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut state = self
            .read_db()
            .map_err(|_| anyhow!("[!] Database could not be read."))?;

        let epic_exists = state.epics.remove(&epic_id);

        if let Some(epic) = epic_exists {
            for story_id in &epic.stories {
                state.stories.remove(story_id);
            }

            self.database
                .write_db(&state)
                .map_err(|_| anyhow!("[!] Database could not be written."))?;

            Ok(())
        } else {
            Err(anyhow!("[!] Epic {} does not exist.", epic_id))
        }
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut state = self
            .read_db()
            .map_err(|_| anyhow!("[!] Database could not be read."))?;

        let deleted_story = state.stories.remove(&story_id);

        if let Some(_story) = deleted_story {
            if let Some(epic) = state.epics.get_mut(&epic_id) {
                epic.stories.retain(|&id| id != story_id);

                self.database
                    .write_db(&state)
                    .map_err(|_| anyhow!("[!] Database could not be written."))?;

                Ok(())
            } else {
                Err(anyhow!("[!] Epic {} does not exist.", epic_id))
            }
        } else {
            Err(anyhow!("[!] Story {} does not exist.", story_id))
        }
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut state = self
            .read_db()
            .map_err(|_| anyhow!("[!] Database could not be read."))?;

        if let Some(epic) = state.epics.get_mut(&epic_id) {
            epic.status = status;

            self.database
                .write_db(&state)
                .map_err(|_| anyhow!("[!] Database could not be written."))?;

            Ok(())
        } else {
            Err(anyhow!("[!] Epic {} does not exist.", epic_id))
        }
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut state = self
            .read_db()
            .map_err(|_| anyhow!("[!] Database could not be read."))?;

        if let Some(story) = state.stories.get_mut(&story_id) {
            story.status = status;

            self.database
                .write_db(&state)
                .map_err(|_| anyhow!("[!] Database could not be written."))?;

            Ok(())
        } else {
            Err(anyhow!("[!] Story {} does not exist.", story_id))
        }
    }
}
