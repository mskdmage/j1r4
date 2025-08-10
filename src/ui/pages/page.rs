use crate::models::Action;
use anyhow::Result;
use std::any::Any;

#[allow(dead_code)]
pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}
