mod action;
mod cmd;
mod git;
mod keys;
mod tui;
mod watcher;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  cmd::parser();
  let mut app = crate::tui::App::new()?;
  app.run()?;
  Ok(())
}
