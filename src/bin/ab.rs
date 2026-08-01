use pugit::git::Git;

fn main() -> anyhow::Result<(), anyhow::Error> {
  let git = Git::new("~/tmp")?;
  Git::ahead_behind_from_current(&git.repo, &git.head.get_attached(&git.repo)?.unwrap(), &git.branches)?;
  Ok(())
}
