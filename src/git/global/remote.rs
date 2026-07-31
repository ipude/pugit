use crate::git::Git;
use git2::Repository;

#[allow(dead_code)]
pub struct RemoteStatus {
  name: String,
  url: String,
  pushurl: String,
}

#[allow(dead_code)]
impl Git {
  /// Returns struct `Vec<RemoteStatus>`. This returns only the remote with distinct names explicitly set by user via :
  /// ```sh
  ///git remote add github https://github.com/usrname/repo.git
  /// git remote add gitlab https://gitlab.com/usrname/repo.git
  /// ```
  /// The number of remotes returned depend on whether they are named unique or not. For multiple rmeote url added under a single umbrella term dont expect each url to be returned:
  ///
  /// For cases like below this function will onky return the remote named `origin` its `url` and `pushurl` and move forward.
  ///
  /// Example:
  /// ```sh
  /// git remote add origin git@github.com:user/repo.git
  /// git remote set-url --add --push origin git@github.com:user/repo.git
  /// git remote set-url --add --push origin git@gitlab.com:user/repo.git
  /// ```
  ///
  /// This is intentionally done keeping these things in mind:
  ///
  /// 1. Majority uses single remotes or multiple remotes as mirror under the name `origin`.
  ///
  /// 2. Minority knows how to manage the remotes so they almost always have distinct names for precise control over push, pull and fetch.
  ///
  fn get_remotes(repo: &Repository) -> anyhow::Result<Vec<RemoteStatus>, anyhow::Error> {
    let string_array = repo.remotes()?;
    let mut vector = Vec::new();
    for i in string_array.iter().flatten() {
      let remote = repo.find_remote(i.unwrap())?;
      vector.push(RemoteStatus {
        name: remote.name()?.unwrap_or("<None>").to_string(),
        url: remote.url()?.to_string(),
        pushurl: remote.pushurl()?.unwrap_or("<None>").to_string(),
      });
    }
    Ok(vector)
  }
}
