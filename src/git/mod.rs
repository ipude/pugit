use git2::{Branch, Commit, Reference, Repository};

pub mod current;
pub mod refresh;
pub mod string_to_path;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
}

pub enum RepoState {
  Clean,
  Merging,
  Rebasing,
  RebaseMerge,
  SingleCherryPick,
  MultiCherryPick,
  SingleRevert,
  MultiRevert,
  Bisect,
  ApplyingMailBox,
  MailBoxOrRebase,
}

#[allow(dead_code)]
pub enum Head<'repo> {
  Refrence(Reference<'repo>),
  Detached(Commit<'repo>),
  Error(String),
  Unborn,
}

#[allow(dead_code)]
pub enum Local<'repo> {
  Branch(Branch<'repo>),
  Error(String),
}

#[allow(dead_code)]
pub enum Upstream<'repo> {
  Branch(Branch<'repo>),
  Commit(Commit<'repo>),
  Error(String),
  None,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    Ok(Self {
      repo: Repository::open(Git::string_to_path(path)?)?,
    })
  }
}
