use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{formatting::*, pages::page::Page},
};
use anyhow::Result;
use itertools::Itertools;
use std::any::Any;
use std::rc::Rc;

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        
        page_header("EPICS", 80);
        
        table_header(&vec![
            ("ID", 5),
            ("NAME", 50),
            ("STATUS", 15),
        ], 80);

        let epics = self.db.read_db()?.epics;

        epics.keys().sorted().for_each(|id| {
            if let Some(epic) = epics.get(id) {
                let id_info = &*id.to_string();
                let name_info = &epic.name;
                let status_info = &epic.status.to_string();

                table_row(&vec![
                    (id_info, 5),
                    (name_info, 50),
                    (status_info, 25)
                ]);
            }
        });

        page_footer(&["[q] Quit", "[c] Create Epic", "[:id:] Navigate to Epic"], 80);

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            _ => match input.parse::<u32>() {
                Ok(id) => {
                    let epics = self.db.read_db()?.epics;
                    match epics.contains_key(&id) {
                        true => Ok(Some(Action::NavigateToEpicDetail { epic_id: id })),
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
