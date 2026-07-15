use pugit::git::{Git, Remote};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/.config/nvim/")?;
  let repo = git.repo;

  let head_state = Git::get_head_state(&repo);
  let local_branch = Git::get_local_branch(&repo, &head_state);
  match &local_branch {
    pugit::git::Current::LocalBranch(branch_name) => {
      let oid = match branch_name.upstream() {
        Ok(b) => Remote::Commit(b.get().target().unwrap()),
        Err(e) => Remote::Error(e.to_string())
      };
      match oid {
        Remote::Commit(oid) => println!("{oid}"),
        Remote::Error(e) => println!("{e}")
      }
    }
    pugit::git::Current::Error(e) => println!("Error (while searching for Current Local Branch ): {}", e),
  }
  Ok(())
}
