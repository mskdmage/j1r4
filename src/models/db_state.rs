use crate::models::{epic::Epic, story::Story};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

impl DBState {
    pub fn new() -> Self {
        Self {
            epics: HashMap::new(),
            stories: HashMap::new(),
            last_item_id: 0,
        }
    }
}
