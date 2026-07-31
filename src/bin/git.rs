use pugit::git::Git;

#[allow(dead_code)]
#[allow(unused)]
fn main() -> anyhow::Result<(), anyhow::Error> {
  let git = Git::new("~/.config/nvim/")?;
  let refs = git.repo.references_glob("refs/heads/**")?;
  for r in refs {
    let r = r?;
    let name = r.name().unwrap_or("").to_string();
    let target = r.target();
    println!("{name} --> {:?}", target)
  }

  for reference in git.repo.references_glob("refs/remotes/*")? {
    let reference = reference?;
    println!("{}", reference.name().unwrap_or("?"));
  }

  Ok(())
}
