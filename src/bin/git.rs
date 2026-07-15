use pugit::git::{Git, Remote};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;
  let repo = git.repo;

  let head_state = Git::get_head_state(&repo);
  let local_branch = Git::get_local_branch(&repo, &head_state);
  let local_branch_oid = Git::get_oid_local_remote(&local_branch);
  match local_branch_oid {
    Remote::Commit(oid) => println!("{oid}"),
    Remote::Error(e) => println!("{e}"),
  }
  Ok(())
}
