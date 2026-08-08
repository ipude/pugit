use git2::{Branch, Repository};

use crate::git::Git;

/// Contains ahead-behind data.
#[allow(dead_code)]
pub struct ABData {
  pub given_branch: String,
  pub other_branch: String,
  pub ahead: usize,
  pub behind: usize,
}

#[allow(dead_code)]
impl Git {
  /// **Given branch** is matched against **all local branches** of the **repository**, **excluding the given branch**.
  pub fn get_ahead_behind(
    repo: &Repository,
    current_branch: &Branch,
    branches: &[(usize, String)],
  ) -> anyhow::Result<Vec<ABData>, anyhow::Error> {
    let mut vector = Vec::new();
    for (_idx, branch) in branches {
      // The current entry of branches that will be matched against branch 1.
      let other_branch = Git::to_branch_local(repo, &branch.as_str())?;

      // To avoid recalculation everytime
      let current_branch_name = match current_branch.name() {
        Ok(Some(v)) => v.to_string(),
        Ok(None) => "<Invalid Utf-8>".to_string(),
        Err(_) => "<Invalid Branch/HEAD is detached>".to_string(),
      };
      let other_branch_name = match current_branch.name() {
        Ok(Some(v)) => v.to_string(),
        Ok(None) => "<Invalid Utf-8>".to_string(),
        Err(_) => "<Invalid Branch/HEAD is detached>".to_string(),
      };

      // The filteration tactic to skip grace fully for the iteration where branch 1 equates branch 2
      if current_branch_name != other_branch_name {
        let (ahead, behind) = repo.graph_ahead_behind(
          current_branch.get().target().unwrap(),
          other_branch.get().target().unwrap(),
        )?;

        vector.push(ABData {
          given_branch: current_branch_name,
          other_branch: other_branch_name,
          ahead,
          behind,
        });
      }
    }
    Ok(vector)
  }
}
