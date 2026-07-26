use git2::{Branch, Repository};

use crate::git::current::head::Head;

/// Returns either of :
/// `Branch(String)` -> Branch's name
/// `Error(String)` -> Error related to branch.
/// None -> None
///
/// No need to use match arm as this is a self contained enum -- mostly
/// You can substitute following methods for `Local.method` or `&git.local.method` :
/// `.is_branch()`
/// `.is_error()`
/// `.is_none()`
///
/// ```
/// if &git.local.is_branch() {
///   let local_branch = local.to_branch()?.unwrap();
/// }
///```
#[allow(dead_code)]
pub enum Local {
  Branch(String),
  Error(String),
  None,
}

// Match handeling -- only one of following can be true.
#[allow(dead_code)]
impl Local {
  /// True if `Branch(String)` exists
  pub fn is_branch(&self) -> bool {
    matches!(self, Local::Branch(_))
  }
  /// True if `Error(String)` exists
  pub fn is_error(&self) -> bool {
    matches!(self, Local::Error(_))
  }
  /// True if `Local` is `None`
  pub fn is_none(&self) -> bool {
    matches!(self, Local::None)
  }
}

impl Local {
  /// This method takes &self
  /// so you call it directly :
  /// ```
  /// &git.local.to_branch()?.unwrap()
  /// ```
  pub fn to_branch<'repo>(
    &self,
    repo: &'repo Repository,
  ) -> anyhow::Result<Option<Branch<'repo>>, anyhow::Error> {
    match self {
      Local::Branch(name) => Ok(Some(repo.find_branch(name, git2::BranchType::Local)?)),
      _ => Ok(None),
    }
  }
}

impl Local {
  /// Returns the Local enum -- very unlikely to fail
  /// Needs a rewrite including Head's rewrite.
  /// Everything else is fine.
  pub fn new(head_ref: &Head, repo: &Repository) -> anyhow::Result<Local> {
    if head_ref.is_refrence() {
      match repo.find_branch(&head_ref.get_value().unwrap(), git2::BranchType::Local) {
        Ok(branch) => Ok(Local::Branch(branch.name()?.unwrap().to_string())),
        Err(e) => Ok(Local::Error(e.to_string())),
      }
    } else {
      Ok(Local::None)
    }
  }
}
