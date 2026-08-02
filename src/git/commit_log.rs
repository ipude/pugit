use git2::{Oid, Repository, Sort};
use crate::git::Git;

#[allow(dead_code)]
impl Git {
  /// Get the entire commit log of `repo`.
  pub fn get_commits_log(repo: &Repository) -> anyhow::Result<Vec<Oid>, git2::Error> {
    // Walk the commit graph.
    let mut revwalk = repo.revwalk()?;
    // use the head'oid as seed to the iterator.
    revwalk.push_head()?;
    // sort the oid into Topological and then sort with Time based ordering.
    // The parameter should always be --> 
    // (Sort::TOPOLOGICAL | Sort::TIME)
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;

    // return 
    revwalk.collect()
  }
}
