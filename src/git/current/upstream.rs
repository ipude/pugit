use git2::{Branch, Oid, Repository};

/// `Found` & `Error` are of type string.
/// NotFound is equivalent to None.
#[allow(dead_code)]
pub enum Upstream {
  Found(String),
  Error(String),
  NotFound,
}

// Public method
#[allow(dead_code)]
impl Upstream {
  /// Gives a new Upstream value 
  pub fn new(local_branch: &Branch) -> anyhow::Result<Upstream, anyhow::Error> {
    Self::get_upstream(local_branch)
  }

  /// Gives oid of repo and is called explicitly.
  pub fn get_oid(
    repo: &Repository,
    upstream_branch: &Branch,
  ) -> anyhow::Result<Oid, anyhow::Error> {
    return Ok(
      repo
        .find_commit(upstream_branch.get().target().unwrap())?
        .id(),
    );
  }

  /// Converts upstream branch's name to Branch<'_> 
  pub fn to_branch<'repo>(
    upstream_branch_name: &str,
    repo: &'repo Repository,
  ) -> anyhow::Result<Branch<'repo>, anyhow::Error> {
    Ok(repo.find_branch(upstream_branch_name, git2::BranchType::Remote)?)
  }
}

// Private method called under new()
impl Upstream {
  /// Gives either of :
  /// 1. `Upstream::Found` if branch has an upstream.
  /// 2. `Upstream::NotFound` if there is no Upstream but local branch exits.
  /// 3. `Upstream::Error` if there is one.
  fn get_upstream(local_branch: &Branch) -> anyhow::Result<Upstream, anyhow::Error> {
    match local_branch.upstream() {
      Ok(b) => Ok(Upstream::Found(b.name()?.unwrap().to_string())),
      Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(Upstream::NotFound),
      Err(e) => Ok(Upstream::Error(e.to_string())),
    }
  }
}
