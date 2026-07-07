pub mod render;
use std::error::Error;

use ratatui::{
  DefaultTerminal, Frame,
};

use crate::state::tabs::TabPage::HomePage;

#[allow(dead_code)]
pub enum TabPage {
  HomePage,
  HelpPage,
}

#[allow(dead_code)]
pub struct App {
  pub current_tab: TabPage,
  pub render_new: bool,
  pub text: String,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> App {
    App {
      current_tab: HomePage,
      render_new: true,
      text: "".to_string(),
    }
  }

  fn draw(&self, terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| self.render(frame))?;
      if crate::keys::init::main()? {
        break;
      }
    }
    Ok(())
  }

  fn render(&self, frame: &mut Frame) {
    if self.render_new {
      App::render_homepage(frame);
    }
  }

  pub fn run(&self)-> Result<(), Box<dyn std::error::Error>>{
    ratatui::run(|terminal| self.draw(terminal))?;
    Ok(())
  }
}
