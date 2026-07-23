use git2::{Branch, Oid, Repository};

use crate::git::{Git, current::local::Local};

#[allow(dead_code)]
pub enum Upstream {
  Branch(String),
  Oid(Oid),
  Error(String),
  None,
}

#[allow(dead_code)]
impl Upstream {
  pub fn to_branch<'repo>(
    &self,
    repo: &'repo Repository,
  ) -> anyhow::Result<Option<Branch<'repo>>, anyhow::Error> {
    match self {
      Upstream::Branch(name) => Ok(Some(repo.find_branch(name, git2::BranchType::Remote)?)),
      _ => Ok(None),
    }
  }
}

#[allow(dead_code)]
impl Upstream {
  pub fn is_branch(&self) -> bool {
    matches!(self, Upstream::Branch(_))
  }
  pub fn contains_oid(&self) -> bool {
    matches!(self, Upstream::Oid(_))
  }
  pub fn is_error(&self) -> bool {
    matches!(self, Upstream::Error(_))
  }
  pub fn is_none(&self) -> bool {
    matches!(self, Upstream::None)
  }
}

#[allow(dead_code)]
impl Upstream {
  pub fn get_branch(&self) -> Option<String> {
    if self.is_branch() {
      match self {
        Upstream::Branch(name) => Some(name.to_string()),
        _ => None,
      }
    } else {
      None
    }
  }
  pub fn get_oid(&self) -> Option<Oid> {
    if self.is_branch() {
      match self {
        Upstream::Oid(oid) => Some(oid.to_owned()),
        _ => None,
      }
    } else {
      None
    }
  }
}

impl Git {
  pub fn get_upstream_oid(
    repo: &Repository,
    local: &Local,
    branch: &Branch,
  ) -> anyhow::Result<Upstream, anyhow::Error> {
    if local.is_branch() {
      return Ok(Upstream::Oid(
        repo.find_commit(branch.get().target().unwrap())?.id(),
      ));
    } else {
      Ok(Upstream::None)
    }
  }

  pub fn get_upstream(local: &Local, branch: &Branch) -> anyhow::Result<Upstream, anyhow::Error> {
    if local.is_branch() {
      match branch.upstream() {
        Ok(b) => Ok(Upstream::Branch(b.name()?.unwrap().to_string())),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(Upstream::None),
        Err(e) => Ok(Upstream::Error(e.to_string())),
      }
    } else {
      Ok(Upstream::None)
    }
  }
}
