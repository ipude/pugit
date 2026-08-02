use crate::git::Git;
use git2::Repository;

#[allow(dead_code)]
impl Git {
  // Gives vector containing tuple of stash's index, message, and oid.
  pub fn get_stash_list(
    repo: &mut Repository,
  ) -> anyhow::Result<Vec<(usize, String, git2::Oid)>, git2::Error> {
    let mut stashes = Vec::new();

    repo.stash_foreach(|index, message, oid| {
      stashes.push((index, message.to_string(), *oid));
      true // return true to keep iterating, false to stop early
    })?;

    Ok(stashes)
  }
}
