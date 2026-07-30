// ==========================
use crate::git::current::{
  config::{self, Config},
  head::Head,
  index::FileStatus,
};
use git2::{Branch, Oid, Repository};
// ==========================

// ==========================
pub mod current;
pub mod string_to_path;
// ==========================

/// Pugit's core data sturcture that holds almost all important Git things.
#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: Head,
  pub config: Config,
  pub index: Vec<FileStatus>,
}

#[allow(dead_code)]
impl Git {
  /// Compiles everything into a single structure.
  /// Can be used for Repowide refresh if called again.
  pub fn new(path: &str) -> anyhow::Result<Self> {
    // Core initialization
    let repo = Repository::open(Git::string_to_path(path)?)?;
    let head = Head::new(&repo)?;

    // Config and Index
    let config = config::Config::new(&repo)?;
    let index = FileStatus::new(&repo)?;

    // Return
    Ok(Self {
      repo,
      head,
      config,
      index,
    })
  }

  /// Global helper function to tackle `String to Branch<'repo>` cases through Git
  /// Convert any valid `local` branch name into `Branch<'repo>`
  pub fn to_branch_local<'repo>(
    repo: &'repo Repository,
    attached: &str,
  ) -> anyhow::Result<Branch<'repo>, anyhow::Error> {
    Ok(repo.find_branch(attached, git2::BranchType::Local)?)
  }

  /// Get oid of any **branch: `Branch<'repo>`**
  pub fn get_oid(repo: &Repository, branch: &Branch) -> anyhow::Result<Oid, anyhow::Error> {
    return Ok(repo.find_commit(branch.get().target().unwrap())?.id());
  }

  /// Returns upstream of any `Branch<'repo>`
  /// May return `None` if there is no `Upstream`.
  /// May return `error: String` if there is an Error.
  /// Else will return a Upstream
  pub fn get_upstream(branch: &Branch) -> anyhow::Result<Option<String>, anyhow::Error> {
    match branch.upstream() {
      Ok(b) => Ok(Some(b.name()?.unwrap().to_string())),
      Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
      Err(e) => Ok(Some(e.to_string())),
    }
  }
}
