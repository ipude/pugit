mod cmd;
mod keys;
mod state;
mod tui;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  cmd::parser();
  ratatui::run(tui::app::app)?;
  Ok(())
}
