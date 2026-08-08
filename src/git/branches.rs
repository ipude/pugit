use std::result::Result;

use crate::git::Git;
use git2::Repository;


#[allow(dead_code)]
impl Git {
  /// Returns Error early if `repo.branches()` return error.
  /// Individual index can contain `Result<branch_name: String, error: String>`
  pub fn get_branches(
    repo: &Repository,
  ) -> Result<(Vec<(usize, String)>, Vec<(usize, String)>), String> {
    // Map error and return early 
    let branches = repo
      .branches(Some(git2::BranchType::Local))
      .map_err(|e| e.to_string())?;
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

    let mut oks: Vec<(usize, String)> = Vec::new();
    let mut errs: Vec<(usize, String)> = Vec::new();

    for (idx, branches) in vector.iter().enumerate() {
      match branches {
        Ok(branch) => oks.push((idx, branch.to_string())),
        Err(error) => errs.push((idx, error.to_string())),
      }
    }
    Ok((oks, errs))
  }
}
