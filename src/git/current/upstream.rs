use git2::{Branch, Oid, Repository};

#[allow(dead_code)]
pub enum Upstream {
  Found(String),
  Error(String),
  NotFound,
}

// Public method
#[allow(dead_code)]
impl Upstream {
  pub fn new(local_branch: &Branch) -> anyhow::Result<Upstream, anyhow::Error> {
    Self::get_upstream(local_branch)
  }

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

  pub fn to_branch<'repo>(
    upstream_branch: &str,
    repo: &'repo Repository,
  ) -> anyhow::Result<Branch<'repo>, anyhow::Error> {
    Ok(repo.find_branch(upstream_branch, git2::BranchType::Remote)?)
  }
}

// Private method called under new()
impl Upstream {
  fn get_upstream(local_branch: &Branch) -> anyhow::Result<Upstream, anyhow::Error> {
    match local_branch.upstream() {
      Ok(b) => Ok(Upstream::Found(b.name()?.unwrap().to_string())),
      Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(Upstream::NotFound),
      Err(e) => Ok(Upstream::Error(e.to_string())),
    }
  }
}
