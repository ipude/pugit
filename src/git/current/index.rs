use git2::Repository;

pub fn index(repo: &Repository) -> anyhow::Result<()> {
  let idx = repo.index()?;
  for i in idx.iter() {
    let path = String::from_utf8_lossy(&i.path);
    println!("{}, {}, {}", i.id, i.mode, path);
  }
  Ok(())
}
