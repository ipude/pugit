use std::error::Error;
use crossterm::event::{Event, KeyCode, KeyModifiers};
pub fn main()-> std::result::Result<bool, Box<dyn Error>>{
  if let Event::Key(key) = crossterm::event::read()? {
    match key.code {
      KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
      _ => {}
    }
  }
  Ok(false)
}
