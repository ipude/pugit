// ==========================
// Imports
// ==========================
use crate::git::{
  ahead_behind::ABData, branches::BranchStatus, config::{Config}, head::Head, index::FileStatus,
  refs::Refs, remote::RemoteStatus, tags_list::TagInfo,
};
use git2::{Oid, Repository};
// ==========================
// ==========================

// ==========================
// Modules
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
// ==========================

// ==========================
// Git init
// ==========================
/// Pugit's core data sturcture that holds almost all important Git things.
#[allow(dead_code)]
pub struct Git {
  // The current repo
  pub current_repo: Repository,
  // The current head's state
  // No need to add current: String as head.get_attached() returns the same.
  pub current_head: Head,

  // Everything under .git/refs/
  // Manage Refs fields as per your needs.
  pub repo_refs: Refs,

  // Everytging under .git/index
  pub repo_config: Vec<Config>,

  // similar to: git status , already contains conflicted files.
  // for staging related work.
  pub repo_staging_index: Vec<FileStatus>,

  // all unique remotes connected to repo
  pub repo_remotes: Vec<RemoteStatus>,

  // all branches inside a repo
  pub repo_branches: Vec<BranchStatus>,

  // ahead behind data for current_branch compared with branches of a repo.
  pub ahead_behind_from_current: Vec<ABData>,

  // commit list of entire repo.
  // equivalent to: git log --oneline
  // purpose: for listing all commits done
  pub repo_commits_done: Vec<Oid>,

  // Stash list of entire repo.
  pub repo_stash_list: Vec<(usize, String, Oid)>,

  // Tag list of entire repo
  pub repo_tag_list: Vec<TagInfo>,
}

#[allow(dead_code)]
impl Git {
  /// Compiles everything into a single structure.
  /// Can be used for Repowide refresh if called again.
  pub fn new(path: &str) -> anyhow::Result<Self> {
    // Current
    let mut current_repo = Repository::open(Git::string_to_path(path)?)?;
    let current_head = Head::new(&current_repo)?;

    // .git/refs/
    let refs_heads = Git::get_refs_from_glob(&current_repo, "refs/heads/**")?;
    let repo_refs = Refs { heads: refs_heads };

    // Repo prefixed:
    let repo_config = Git::get_config(&current_repo);
    let repo_staging_index = FileStatus::new(&current_repo)?;
    let repo_remotes = Git::get_remotes(&current_repo)?;
    let repo_branches = Git::get_branches(&current_repo)?;
    let repo_commits_done = Git::get_commits_log(&current_repo)?;
    let repo_stash_list = Git::get_stash_list(&mut current_repo)?;
    let repo_tag_list = Git::get_tags_detailed(&current_repo)?;

    // Comparison of all branches with current one for ahead_behind
    let ahead_behind_from_current = Git::ahead_behind_from_current(
      &current_repo,
      &current_head.get_attached(&current_repo)?.unwrap(),
      &repo_branches,
    )?;

    Ok(Self {
      current_repo,
      current_head,
      repo_refs,
      repo_config,
      repo_staging_index,
      repo_remotes,
      repo_branches,
      repo_commits_done,
      repo_stash_list,
      repo_tag_list,
      ahead_behind_from_current,
    })
  }
}
// ==========================
// ==========================
