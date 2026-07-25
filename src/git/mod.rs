use git2::Repository;

use crate::git::current::{
  config::{self, Config},
  head::Head,
  local::Local,
  upstream::Upstream,
};
pub mod current;
pub mod string_to_path;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: Head,
  pub local: Local,
  pub upstream: Upstream,
  pub config: Config,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    let repo = Repository::open(Git::string_to_path(path)?)?;
    let head = Git::get_current_head(&repo)?;
    let local = Git::get_current_local_branch(&head, &repo)?;
    let upstream = {
      let local_branch = &local.to_branch(&repo)?.expect("No such Local Branch");
      Upstream::new(local_branch)?
    };
    let config = config::Config::new(&repo)?;
    Ok(Self {
      repo,
      head,
      local,
      upstream,
      config,
    })
  }
}
