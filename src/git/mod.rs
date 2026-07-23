use git2::Repository;

use crate::git::current::{head::Head, local::Local, upstream::Upstream};
pub mod current;
pub mod string_to_path;

#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: Head,
  pub local: Local,
  pub upstream: Upstream,
}

#[allow(dead_code)]
impl Git {
  pub fn new(path: &str) -> anyhow::Result<Self> {
    let repo = Repository::open(Git::string_to_path(path)?)?;
    let head = Git::get_current_head(&repo)?;
    let local = Git::get_current_local_branch(&head, &repo)?;
    let _upstream_constructor_1 = {
      let local_branch = &local.to_branch(&repo)?.unwrap();
      Git::get_upstream(&local, local_branch)?
    };
    let upstream = Git::get_upstream_oid(&repo, &local, &_upstream_constructor_1.to_branch(&repo)?.unwrap())?;

    match &upstream {
      Upstream::Oid(o)=> println!("{o}"),
      _ => println!("no oid")
    }
    Ok(Self {
      repo,
      head,
      local,
      upstream,
    })
  }
}
