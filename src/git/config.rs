use git2::Repository;
use crate::git::Git;

/// `GitConfig` conatins all existing configs with their assigned value.
/// Tip toe copy of `.git/config`
#[allow(dead_code)]
pub struct GitConfig {
  // name of config
  pub config_name: String,
  // value of config
  pub config_value: String,
}

impl Git {
  /// Get the config of entered `repo: &Repository`.
  /// Each config has a `name` and a `value`.
  pub fn get_config(repo: &Repository) -> anyhow::Result<Vec<GitConfig>> {
    // get repo's config and snapshot it.
    let mut config = repo.config()?;
    let snapshot = config.snapshot()?;
    let mut vector = Vec::new();

    // iter over all the config variable (everything insdie .git/config irrespective of any specific glob)
    let mut entries = snapshot.entries(None)?;

    // Until entries.next() don't returns None
    while let Some(entry) = entries.next() {
      // Skip if there is an error
      let entry = match entry{
        Ok(entry) => entry,
        Err(_) => continue,
      };

      // use .unwrap_or() instead of try_operator (?) as runtime faliures along with long error messages are not top pick.
      // Keep in mind that these fields are for display.
      let entry_name = entry.name().unwrap_or("<Invalid Entry Name>").to_string();
      let entry_value = entry.value().unwrap_or("Invalid Entry Value").to_string();

      vector.push(GitConfig {
        config_name: entry_name,
        config_value: entry_value,
      });
    }
    Ok(vector)
  }
}
