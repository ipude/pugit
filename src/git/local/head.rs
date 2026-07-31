use git2::{Branch, Oid};
use git2::{ErrorCode, Repository};

use crate::git::Git;

/// This enum contains status of Head.
/// `Attached(String)` if head is a branch.
/// `Detached(Oid)` if head is'nt a branch but points to a commit.
/// `Error(String)` Serious errors worth displaying in tui.
/// `Unborn` if head is unborn.
///
/// If `head.is_attached()` returns true then look for `head.get_attached(repo: &Repository)` to get **Current Branch**.
#[allow(dead_code)]
pub enum Head {
  Attached(String),
  Detached(Oid),
  Error(String),
  Unborn,
}

// Helper functions for Head
#[allow(dead_code)]
impl Head {
  /// Returns true if `Head::Attached(_)` matches.
  pub fn is_attached(&self) -> bool {
    matches!(self, Head::Attached(_))
  }
  /// Returns true if `Head::Detached(_)` matches.
  pub fn is_detached(&self) -> bool {
    matches!(self, Head::Detached(_))
  }
  /// Returns true if `Head::Error(_)` matches.
  pub fn is_error(&self) -> bool {
    matches!(self, Head::Error(_))
  }
  /// Returns true if `Head::Unborn` matches.
  pub fn is_unborn(&self) -> bool {
    matches!(self, Head::Unborn)
  }

  /// Returns the current `Branch<'repo>` you are on.
  pub fn get_attached<'repo>(
    &self,
    repo: &'repo Repository,
  ) -> anyhow::Result<Option<Branch<'repo>>, anyhow::Error> {
    match self {
      Head::Attached(name) => {
        let branch = Git::to_branch_local(repo, &name.to_string())?;
        Ok(Some(branch))
      }
      _ => Ok(None),
    }
  }

  /// Returns oid if Head is deatched but points to a commit.
  pub fn get_detached(&self) -> Option<Oid> {
    match self {
      Head::Detached(oid) => Some(*oid),
      _ => None,
    }
  }

  /// Returns important error for displaying.
  pub fn get_error(&self) -> Option<String> {
    match self {
      Head::Error(err) => Some(err.to_string()),
      _ => None,
    }
  }
}

#[allow(dead_code)]
impl Head {
  /// Resolves status of Head. See enum `Head` for more details.
  pub fn new(repo: &Repository) -> anyhow::Result<Head, anyhow::Error> {
    match repo.head() {
      Ok(head) => {
        if head.is_branch() {
          // Make sure it must be repo.shorthand()? not repo.name()?
          Ok(Head::Attached(head.shorthand()?.to_string()))
        } else {
          match head.target() {
            Some(oid) => Ok(Head::Detached(oid)),
            None => Ok(Head::Error(
              "Detached HEAD but points to no Commit.".to_string(),
            )),
          }
        }
      }
      Err(e) if e.code() == ErrorCode::UnbornBranch => Ok(Head::Unborn),

      Err(e) => Ok(Head::Error(e.to_string())),
    }
  }
}
