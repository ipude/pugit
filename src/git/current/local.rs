use git2::{Branch, Repository};

use crate::git::{Git, current::head::Head};

#[allow(dead_code)]
pub enum Local {
  Branch(String),
  Error(String),
  None,
}

#[allow(dead_code)]
impl Local {
  pub fn is_branch(&self) -> bool {
    matches!(self, Local::Branch(_))
  }
  pub fn is_error(&self) -> bool {
    matches!(self, Local::Error(_))
  }
  pub fn is_none(&self) -> bool {
    matches!(self, Local::None)
  }
}

impl Local {
  pub fn to_branch<'repo>(&self, repo: &'repo Repository) -> anyhow::Result<Option<Branch<'repo>>, anyhow::Error> {
    match self {
      Local::Branch(name) => Ok(Some(repo.find_branch(name, git2::BranchType::Local)?)),
      _ => Ok(None),
    }
  }
}

impl Git {
  pub fn get_current_local_branch(head_ref: &Head, repo: &Repository) -> anyhow::Result<Local> {
    match &head_ref {
      Head::Refrence(name) => match repo.find_branch(name, git2::BranchType::Local) {
        Ok(branch) => Ok(Local::Branch(branch.name()?.unwrap().to_string())),
        Err(e) => Ok(Local::Error(e.to_string())),
      },
      _ => Ok(Local::None),
    }
  }
}
