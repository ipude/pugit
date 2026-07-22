use pugit::git::{Git};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;

  if git.head.is_refrence() {
    println!("It is head")
  }
  Ok(())
}
