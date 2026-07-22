use anyhow::Ok;
use git2::{Branch, Oid, Repository};

use crate::git::{Git, current::local::Local};

#[allow(dead_code)]
pub enum Upstream {
  Branch(String),
  Oid(Oid),
  Error(String),
  None,
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
      return Ok(Upstream::None);
    }
  }

  pub fn get_upstream(local: &Local, branch: &Branch) -> anyhow::Result<Upstream, anyhow::Error> {
    if local.is_branch() {
      return Ok(Upstream::Branch(
        branch.upstream()?.name()?.unwrap().to_string(),
      ));
    } else {
      return Ok(Upstream::None);
    }
  }
}
