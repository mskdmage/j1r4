mod db;
mod models;

use models::{DBState, Epic, Status, Story};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db_state = DBState::new();
    db_state
        .epics
        .insert(1, Epic::new("Hello".to_string(), "Hello".to_string()));

    dbg!();

    Ok(())
}
