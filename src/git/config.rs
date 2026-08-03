use crate::git::Git;
use git2::Repository;

// Contains cached individual config entry and its value.
#[allow(dead_code)]
pub struct ConfigEntry {
  pub config_entry: String,
  pub config_value: String,
}

/// Purpose: Handeling error and storing in String format.
/// There can be either `Found(ConfigEntry)` or Err(String)
#[allow(dead_code)]
pub enum Config {
  Found(ConfigEntry),
  Err(String),
}

impl Git {
  /// Get config fields and their values for any repo.
  pub fn get_config(repo: &Repository) -> Vec<Config> {
    // Get config or return early
    let mut config = match repo.config() {
      Ok(config) => config,
      Err(e) => return vec![Config::Err(e.to_string())],
    };

    // Cache the config.
    let snapshot = match config.snapshot() {
      Ok(s) => s,
      Err(e) => return vec![Config::Err(e.to_string())],
    };

    // iter over everything inside .git/config
    let mut entries = match snapshot.entries(None) {
      Ok(e) => e,
      Err(e) => return vec![Config::Err(e.to_string())],
    };
    let mut vector = Vec::new();

    // Until entries.next() don't returns None
    while let Some(entry) = entries.next() {
      // Push error and iter next or do the futher operation.
      let entry = match entry {
        Ok(entry) => entry,
        Err(e) => {
          vector.push(Config::Err(e.to_string()));
          continue;
        }
      };

      // Keep in mind that these fields are for display purpose.
      let entry_name = entry.name().unwrap_or("<Invalid Entry Name>").to_string();
      let entry_value = entry.value().unwrap_or("<Invalid Entry Value>").to_string();

      vector.push(Config::Found(ConfigEntry {
        config_entry: entry_name,
        config_value: entry_value,
      }));
    }
    vector
  }
}
