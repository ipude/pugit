use git2::Repository;

#[allow(dead_code)]
#[allow(unused)]
fn branches(repo: &Repository) -> anyhow::Result<(), anyhow::Error> {
  let refs = repo.references_glob("refs/heads/**")?;
  for r in refs {
    let r = r?;
    let name = r.name().unwrap_or("").to_string();
    let target = r.target();
    println!("{name} --> {:?}", target)
  }
  Ok(())
}
