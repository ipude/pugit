use ratatui::{Frame, layout::Constraint, widgets::{Block, Borders, Paragraph}};
use crate::state::tabs::App;

#[allow(dead_code)]
impl App {
  pub fn render_homepage(frame: &mut Frame){
    let [homepage] = ratatui::layout::Layout::vertical([Constraint::Fill(1)]).areas(frame.area());

    frame.render_widget(Paragraph::new("This is the Homepage").block(Block::new().borders(Borders::ALL)), homepage);
  }
}
