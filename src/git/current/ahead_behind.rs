use git2::{Branch, Repository};

use crate::git::Git;

#[allow(dead_code)]
impl Git {
  fn ahead_behind_from_current(
    repo: &Repository,
    local_branch: &Branch,
  ) -> anyhow::Result<(), anyhow::Error> {
    let main = repo.find_branch("main", git2::BranchType::Local)?;
    let new = repo.find_branch("new", git2::BranchType::Local)?;

    let (ahead, behind) =
      repo.graph_ahead_behind(main.get().target().unwrap(), new.get().target().unwrap())?;

    if ahead > behind {
      println!(
        "Branch: {} is ahead by {ahead} commits from  Branch: {}",
        main.name()?.unwrap_or("<no name>").to_uppercase(),
        new.name()?.unwrap_or("<no name>").to_uppercase()
      )
    }
    Ok(())
  }
}
