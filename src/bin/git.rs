use pugit::git::{Git, Upstream};

fn main() -> anyhow::Result<()> {
  let mut git = Git::new("~/impl/rust/pugit/")?;

  let head = git.get_current_head()?;

  if git.check.head.is_head {
    println!("It is head")
  }
  Ok(())
}
