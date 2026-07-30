use git2::{Branch, Oid};
use git2::{ErrorCode, Repository};

use crate::git::Git;

/// This Enum contains -->
/// Attached head's branch name -- `Attached(String)`
/// Detached head's oid         -- `Detached(Oid)`   
/// Important error             -- `Error(String)`   
/// When head is unborn.        -- `Unborn`          
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
  pub fn is_branch(&self) -> bool {
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

  /// Returns a tuple of `Head::Attached(String)` along with a `Branch<'repo>` if Head is attached else `None`
  pub fn get_attached<'repo>(
    &self,
    repo: &'repo Repository,
  ) -> anyhow::Result<Option<(String, Branch<'repo>)>, anyhow::Error> {
    match self {
      Head::Attached(name) => {
        let branch = Git::to_branch_local(repo, &name.to_string())?;
        Ok(Some((name.to_string(), branch)))
      }
      _ => Ok(None),
    }
  }

  /// Returns value of `Head::Detached(Oid)`
  pub fn get_detached(&self) -> Option<Oid> {
    match self {
      Head::Detached(oid) => Some(*oid),
      _ => None,
    }
  }

  /// Returns value of `Head::Error(String)`
  pub fn get_error(&self) -> Option<String> {
    match self {
      Head::Error(err) => Some(err.to_string()),
      _ => None,
    }
  }
}

#[allow(dead_code)]
impl Head {
  /// Generates the enum Head
  pub fn new(repo: &Repository) -> anyhow::Result<Head, anyhow::Error> {
    match repo.head() {
      Ok(head) => {
        if head.is_branch() {
          // Make sure it must be repo.shorthand()? not repo.name()?
          Ok(Head::Attached(head.shorthand()?.to_string()))
        } else {
          match head.target() {
            Some(oid) => Ok(Head::Detached(repo.find_commit(oid)?.id())),
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
