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
  ///
  /// This method turns current branch's name into `Brnach<'repo>` for fine control.
  ///
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
  /// ---
  ///
  /// Take a refrence at `Head::new(repo: &Repository)` before using this method as this is the second step after `Head::new(repo)`.
  ///
  /// ---
  ///
  /// 1. This method may return `Local::Branch(name)` if `Head::Branch(name)` is valid utf-8 string else you will get an error string to display.
  ///
  /// 2. If Head is not at all a branch i.e if `Head.is_branch()` returns `false` then you will get `Local::None` i.e None.
  pub fn new(head_ref: &Head, repo: &Repository) -> anyhow::Result<Local> {
    // Only check for a branch if head returns a branch name.
    // If the name returns a branch then give its name i.e `Local::Branch(String)` or if Err then return `Local::Error(String)`.
    // Else give `Local::None`
    if head_ref.is_branch() {
      match repo.find_branch(&head_ref.get_value().unwrap(), git2::BranchType::Local) {
        Ok(branch) => Ok(Local::Branch(branch.name()?.unwrap().to_string())),
        Err(e) => Ok(Local::Error(e.to_string())),
      }
    } else {
      Ok(Local::None)
    }
  }
}
