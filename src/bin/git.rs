use fgit::git::Git;

#[allow(dead_code)]
#[allow(unused)]
fn main() -> anyhow::Result<(), anyhow::Error> {
  let git = Git::new("../")?;
  if git.head.is_attached() {
    let b = git.head.get_attached(&git.repo)?.unwrap();
    println!("{}", b.name()?.unwrap());
  }
  Ok(())
}
