use git2::Repository;

use crate::git::{Git, head::HeadCondition};

impl Git {
  pub fn filter_result(
    repo: &Repository,
    head: &HeadCondition,
  ) -> Option<(Vec<(usize, String)>, Vec<(usize, String)>)> {
    let result = Git::get_branches(&repo);
    if let Ok((branches, error_list)) = result {
      // let ahead_behind_from_current =
      //   Git::ahead_behind_from_current(&repo, &head.get_attached(&repo)?.unwrap(), &oks);
      return Some((branches, error_list));
    } else {
      return None;
    }
  }
}
