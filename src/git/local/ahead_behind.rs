use git2::{Branch, Repository};

use crate::git::{Git, global::branches::BranchStatus};

#[allow(dead_code)]
impl Git {
  pub fn ahead_behind_from_current(
    repo: &Repository,
    current_branch: &Branch,
    branches: &[BranchStatus],
  ) -> anyhow::Result<(), anyhow::Error> {
    for branch in branches.iter() {
      if branch.found() {
        let b1 = Git::to_branch_local(repo, &branch.get_value())?;
        let b2 = Git::to_branch_local(repo, &branches.iter().next().unwrap().get_value())?;

        let (ahead, behind) =
          repo.graph_ahead_behind(b1.get().target().unwrap(), b2.get().target().unwrap())?;

        print!(
          "({}, {}):    ",
          b1.name()?.unwrap_or("<no name>").to_uppercase(),
          b2.name()?.unwrap_or("<no name>").to_uppercase()
        );

        println!("({}, {})", ahead, behind);
      }
    }
    Ok(())
  }
}
