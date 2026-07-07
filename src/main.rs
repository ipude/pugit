mod cmd;
mod keys;
mod state;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  cmd::parser();
  let app = crate::state::tabs::App::new();
  app.run()?;
  Ok(())
}
