use git2::{Branch, Commit, Oid, Repository};

pub mod current;
pub mod refresh;
pub mod string_to_path;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub check: Check,
  pub head: Head,
}

// Helps doing if else based checks when ever we dont want to match Enum's result.
pub struct Check {
  pub head: HeadCheck,
}

pub struct HeadCheck {
  pub is_head: bool,
  pub is_detached: bool,
  pub is_detached_no_commit: bool,
  pub is_unborn: bool,
  pub is_error: bool,
}

impl HeadCheck {
  pub fn new() -> Self {
    Self {
      is_head: false,
      is_detached: false,
      is_detached_no_commit: false,
      is_unborn: false,
      is_error: false,
    }
  }
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
    Ok(Self {
      repo: Repository::open(Git::string_to_path(path)?)?,
      check: Check {
        head: HeadCheck::new(),
      },
    })
  }
}

pub struct SharedEnum {
  pub head: Head,
}
