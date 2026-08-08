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
  /// Given branch is matched against every local branch of repo, excluding itself.
  pub fn get_ahead_behind(
    repo: &Repository,
    current_branch: &Branch,
    branches: &[(usize, String)],
  ) -> anyhow::Result<Vec<ABData>, anyhow::Error> {
    let mut vector = Vec::new();

    for (_idx, branch) in branches {
      let other_branch = Git::to_local_branch(repo, &branch.as_str())?;

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

      // skip same branch
      if current_branch_name != other_branch_name {

        let (ahead, behind) = match repo.graph_ahead_behind(
          current_branch.get().target().unwrap(),
          other_branch.get().target().unwrap(),
        ) {
          Ok(v) => v,
          Err(_) => continue,
        };

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
