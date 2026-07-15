use ratatui::layout::{self};
use std::error::Error;

use ratatui::DefaultTerminal;

use crate::state::tabs::App;
#[allow(dead_code)]
impl App {
  pub fn draw_home_page(
    &self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    terminal.draw(|frame| {
      let [help_page] =
        ratatui::layout::Layout::vertical([layout::Constraint::Fill(1)]).areas(frame.area());
      frame.render_widget(
        ratatui::widgets::Paragraph::new("This is the Home Page").block(
          ratatui::widgets::Block::new()
            .borders(ratatui::widgets::Borders::ALL)
            .title("Home Page"),
        ),
        help_page,
      );
    })?;
    Ok(())
  }

  pub fn draw_help_page(
    &self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    terminal.draw(|frame| {
      let [help_page] =
        ratatui::layout::Layout::vertical([layout::Constraint::Fill(1)]).areas(frame.area());
      frame.render_widget(
        ratatui::widgets::Paragraph::new("This is the Help Page").block(
          ratatui::widgets::Block::new()
            .borders(ratatui::widgets::Borders::ALL)
            .title("Help Page"),
        ),
        help_page,
      );
    })?;
    Ok(())
  }
}
