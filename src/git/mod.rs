use git2::{Branch, Commit, Oid, Repository};

pub mod current;
pub mod refresh;
pub mod string_to_path;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: Head,
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
pub enum Head {
  Refrence(String),
  Detached(Oid),
  Error(String),
  Unborn,
}

#[allow(dead_code)]
impl Head {
  pub fn is_refrence(&self) -> bool {
    matches!(self, Head::Refrence(_))
  }
  pub fn is_detached(&self) -> bool {
    matches!(self, Head::Detached(_))
  }
  pub fn is_error(&self) -> bool {
    matches!(self, Head::Error(_))
  }
  pub fn is_unborn(&self) -> bool {
    matches!(self, Head::Unborn)
  }
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
    let repo = Repository::open(Git::string_to_path(path)?)?; 
    let head = Git::get_current_head(&repo)?;
    Ok(Self {
      repo,
      head,
    })
  }
}
