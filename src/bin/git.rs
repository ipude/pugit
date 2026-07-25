use pugit::git::{Git, current::upstream::Upstream};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;

  match &git.upstream {
    Upstream::Found(name) => {
      let oid = Upstream::get_oid(&git.repo, &Upstream::to_branch(name, &git.repo)?)?;
      println!("upstream branch: {name}\nUpstream Oid: {oid}")
    }
    Upstream::NotFound => println!("no upstream configured"),
    Upstream::Error(e) => println!("upstream error: {e}"),
  }

  for (i, v) in &git.config.config {
    println!("{i} --> {v}");
  }

  Ok(())
}
