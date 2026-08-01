use git2::{Branch, Repository};

use crate::git::{Git, global::branches::BranchStatus};

/// ABData means Ahead Behind Data. This struct contains two touples.
/// (String, String) touple stands for (Current branch, Non-current branch).
/// (usize, usize) touple stands for (ahead, behind)
///
/// This struct is meant to be packed inside Vec: ABData where each branch.0 stands for current branch or the branch entered as parameter of `Git::ahead_behind_from_current(&repo, &branch1, &branches)?`
#[allow(dead_code)]
pub struct ABData {
  pub branch: (String, String),
  pub ahead_behind: (usize, usize),
}

#[allow(dead_code)]
impl Git {
  /// This function can compare the parameter 2 or simoly the entered branch against branches: &[BranchStatus] i.e the vector of branches returned via `Git::branches()?`.
  /// Keep in mind that the returned data i.e ABData will always be a struct of two touples and dont forget to check the struct docs.
  pub fn ahead_behind_from_current(
    repo: &Repository,
    branch1: &Branch,
    branches: &[BranchStatus],
  ) -> anyhow::Result<Vec<ABData>, anyhow::Error> {
    let mut vector = Vec::new();
    for branch in branches {
      if branch.found() {
        // The current entry of branches that will be matched against branch 1.
        let branch2 = Git::to_branch_local(repo, &branch.get_value())?;

        // To avoid recalculation everytime
        let name1 = branch1.name()?.unwrap().to_string();
        let name2 = branch2.name()?.unwrap().to_string();

        // The filteration tactic to skip grace fully for the iteration where branch 1 equates branch 2
        if name1 != name2 {
          let (ahead, behind) = repo.graph_ahead_behind(
            branch1.get().target().unwrap(),
            branch2.get().target().unwrap(),
          )?;

          vector.push(ABData {
            branch: (name1, name2),
            ahead_behind: (ahead, behind),
          });
        }
      }
    }
    Ok(vector)
  }
}
