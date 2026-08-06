pub mod draw;
use std::{error::Error, sync::Arc};

use ratatui::DefaultTerminal;

use crate::{action::navigate::Action, tui::Pages::HomePage, watcher::WatchSignals};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Pages {
  HomePage,
  HelpPage,
}

#[allow(dead_code)]
pub struct App {
  pub signal: Arc<WatchSignals>,
  pub active_page: Pages,
  pub text: String,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> anyhow::Result<App, anyhow::Error> {
    Ok(App {
      signal: WatchSignals::spawn()?,
      active_page: HomePage,
      text: "".to_string(),
    })
  }

  fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      match self.active_page {
        Pages::HomePage => {
          self.draw_home_page(terminal)?;
        }
        Pages::HelpPage => {
          self.draw_help_page(terminal)?;
        }
      }
      match crate::keys::input::handle_input(self)? {
        Action::Quit => break,
        Action::GoToHomePage => {
          self.active_page = Pages::HomePage;
        }
        Action::GoToHelpPage => {
          self.active_page = Pages::HelpPage;
        }
        Action::EnterContinue => {}
        Action::EnterSetting => {}
        Action::None => continue,
      }
    }
    Ok(())
  }

  pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| self.draw(terminal))?;
    Ok(())
  }
}
