use std::result;

use crate::git::Git;
use git2::Repository;

#[allow(dead_code)]
impl Git {
  /// Returns Error early if `repo.branches()` return error.
  /// Individual index can contain `Result<branch_name: String, error: String>`
  pub fn get_branches(
    repo: &Repository,
  ) -> result::Result<Vec<result::Result<String, String>>, String> {
    let branches = match repo.branches(Some(git2::BranchType::Local)) {
      Ok(branchiterator) => branchiterator,
      Err(e) => return Err(e.to_string()),
    };
    let mut vector = Vec::new();

    for branch in branches {
      let (branch, _btype) = match branch {
        Ok((b, t)) => (b, t),
        Err(e) => {
          vector.push(Err(e.to_string()));
          continue;
        }
      };

      vector.push(match branch.name() {
        Err(e) => Err(e.to_string()),
        Ok(Some(b)) if !b.is_empty() => Ok(b.to_string()),
        _ => Ok("NA".to_string()),
      });
    }
    Ok(vector)
  }
}
