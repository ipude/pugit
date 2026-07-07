use crate::action::navigate::Action;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;
pub fn handle_input() -> std::result::Result<Action, Box<dyn Error>> {
  if let Event::Key(key) = crossterm::event::read()? {
    match key.code {
      KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => return Ok(Action::Quit),

      KeyCode::Char('h') => return Ok(Action::GoToHelpPage),
      _ => {}
    }
  }
  Ok(Action::None)
}
