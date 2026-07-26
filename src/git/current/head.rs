use git2::Oid;
use git2::{ErrorCode, Repository};

/// Returns either of :
/// `Refrence(String)` -- Attached head's branch name
/// `Detached(Oid)` -- Detached head's oid
/// `Error(String)` -- Real error
/// `Unborn` -- When is unborn.
#[allow(dead_code)]
pub enum Head {
  Refrence(String),
  Detached(Oid),
  Error(String),
  Unborn,
}

// Match handeling.
#[allow(dead_code)]
impl Head {
  /// Returns true if `Head::Refrence(_)` matches.
  pub fn is_refrence(&self) -> bool {
    matches!(self, Head::Refrence(_))
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
}

impl Head {
  /// This function is intentionally made to give `Option<String>` i.e the name of `Head::Refrence(name)` if it exists.
  /// This may panic on `.unwrap()` if `Head::Refrence()` is None.
  pub fn get_value(&self) -> Option<String> {
    match self {
      Head::Refrence(name) => Some(name.to_string()),
      _ => None,
    }
  }
}

#[allow(dead_code)]
impl Head {
  /// Retuns enum `Head`.
  /// May return :
  /// `Refrence(String)` if head is a branch.
  /// `Detached(Oid)` if head is detached.
  /// `Unborn` if head is unborn.
  /// `Error` if somthing is wrong.
  pub fn new(repo: &Repository) -> anyhow::Result<Head, anyhow::Error> {
    match repo.head() {
      Ok(head) => {
        // Note: This is repo.head()?.is_branch()
        if head.is_branch() {
          Ok(Head::Refrence(head.shorthand()?.to_string()))
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

  // pub fn ahead_behind(repo: &Repository, head: &Head, local: &Local) -> anyhow::Result<()> {
  //   match &head {
  //     Head::Refrence(head) => {
  //       // let local_oid = head.target().unwrap();
  //       // let upstream_oid = Git::get_current_upstream(&local);
  //     }
  //     _ => {}
  //   }
  //   Ok(())
  // }
}
