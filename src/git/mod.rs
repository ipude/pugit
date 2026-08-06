use std::result;

use crate::git::{
  ahead_behind::ABData, branches::BranchStatus, config::ConfigData, head::HeadCondition,
  index::StatusCode, refs::RefrenceContainer, remote::RemoteStatus, repo_state::RepoState,
  tags_list::TagInfo,
};
use git2::{Oid, Repository};

// ==========================
pub mod ahead_behind;
pub mod branches;
pub mod commit_log;
pub mod config;
pub mod head;
pub mod index;
pub mod refs;
pub mod remote;
pub mod repo_state;
pub mod stash_list;
pub mod string_to_path;
pub mod tags_list;
pub mod utils;
// ==========================

/// Fgit's data struct for Git.
#[allow(dead_code)]
pub struct Git {
  pub repo: Repository,
  pub head: HeadCondition,
  pub refs: RefrenceContainer,
  pub config: Vec<result::Result<ConfigData, String>>,
  pub git_status: Vec<result::Result<StatusCode, String>>,

  // all unique remotes connected to repo
  pub remotes: Vec<RemoteStatus>,

  // all branches inside a repo
  pub branches: Vec<BranchStatus>,

  // ahead behind data for current_branch compared with branches of a repo.
  pub ahead_behind_from_current: Vec<ABData>,

  // commit list of entire repo.
  // equivalent to: git log --oneline
  // purpose: for listing all commits done
  pub commits: Vec<Oid>,

  // Stash list of entire repo.
  pub stash_list: Vec<(usize, String, Oid)>,

  // Tag list of entire repo
  pub tag_list: Vec<TagInfo>,

  // State of repo
  pub state: RepoState,
}

#[allow(dead_code)]
impl Git {
  /// Compiles everything into a single structure.
  /// Can be used for Repowide refresh if called again.
  pub fn new(path: &str) -> anyhow::Result<Self> {
    // Current
    let mut repo = Repository::open(Git::string_to_path(path)?)?;
    let head = HeadCondition::new(&repo)?;
    let refs = RefrenceContainer::new(&repo);

    // Repo prefixed:
    let config = Git::get_config(&repo);
    let git_status = StatusCode::new(&repo);
    let remotes = Git::get_remotes(&repo)?;
    let branches = Git::get_branches(&repo)?;
    let commits = Git::get_commits_log(&repo)?;
    let stash_list = Git::get_stash_list(&mut repo)?;
    let tag_list = Git::get_tags_detailed(&repo)?;

    let state = Git::get_repo_state(&repo);

    // Comparison of all branches with current one for ahead_behind
    let ahead_behind_from_current =
      Git::ahead_behind_from_current(&repo, &head.get_attached(&repo)?.unwrap(), &branches)?;

    Ok(Self {
      repo,
      head,
      refs,
      config,
      git_status,
      remotes,
      branches,
      commits,
      stash_list,
      tag_list,
      state,
      ahead_behind_from_current,
    })
  }
}
// ==========================
// ==========================
