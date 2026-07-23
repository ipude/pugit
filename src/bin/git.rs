use pugit::git::{
  Git,
  current::{local::Local, upstream::Upstream},
};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;

  if git.head.is_refrence() {
    match Git::get_current_local_branch(&git.head, &git.repo)? {
      Local::Branch(name) => println!("{}", name),
      Local::Error(error) => println!("{}", error),
      Local::None => println!("None"),
    }
  }

  let branch = git.local.to_branch(&git.repo)?;
  println!("{}", branch.unwrap().name()?.unwrap());
  let _upstream_oid = Git::get_upstream_oid(
    &git.repo,
    &git.local,
    &git.upstream.to_branch(&git.repo)?.unwrap(),
  )?;
  let id = match &_upstream_oid {
    Upstream::Oid(id) => Some(id),
    _ => None,
  };
  println!("{:?}", id);
  Ok(())
}
