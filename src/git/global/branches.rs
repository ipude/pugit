use crate::git::Git;
use git2::Repository;

/// Contains either `Found(bname)` or the `NotFound` for the underlying branch.
/// Return `NotFound` on `None` to avoid `.unwrap()` or `.unwrap_or()` while calling `.name()?` for the underlying branch in `Git::get_branches(repo: &Repository)`.
#[allow(dead_code)]
pub enum BranchStatus {
  Found(String),
  NotFound,
}

#[allow(dead_code)]
impl BranchStatus {
  /// Gives a `unique_name: String` if `BranchStatus:: Found(String)` matches else will return `"<Invalid Branch Name>"`
  pub fn get_value(&self) -> String {
    match self {
      BranchStatus::Found(b) => b.to_string(),
      BranchStatus::NotFound => "<Invalid Branch Name>".to_string(),
    }
  }

  /// Returns true if there is a valid branch name else will return false.
  pub fn found(&self) -> bool {
    matches!(self, BranchStatus::Found(_))
  }
}

#[allow(dead_code)]
impl Git {
  /// Returns `Vec<BranchStatus>`. Each entry is a either `Found(branch_name)` or `NotFound`.
  /// If Branch's name is **invalid utf-8**, or **can't be parsed**, or is **blank** then this function would return `BranchStatus::NotFound`
  pub fn get_branches(repo: &Repository) -> anyhow::Result<Vec<BranchStatus>, anyhow::Error> {
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    let mut vector = Vec::new();

    for branch in branches {
      let (branch, _btype) = branch?;

      vector.push(match branch.name() {
        Ok(Some(b)) if !b.is_empty() => BranchStatus::Found(b.to_string()),
        _ => BranchStatus::NotFound,
      });
    }
    Ok(vector)
  }
}
