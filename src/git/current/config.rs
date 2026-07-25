use git2::Repository;

/// Repository's config
pub struct Config {
  pub config: Vec<(String, String)>,
}

impl Config {
  pub fn new(repo: &Repository) -> anyhow::Result<Config> {
    let mut config = repo.config()?;
    let snapshot = config.snapshot()?;
    let mut entries = snapshot.entries(None)?;
    let mut config_data = Vec::new();
    while let Some(entry) = entries.next() {
      let entry = entry?;
      let name = entry.name().unwrap_or("").to_string();
      let value = entry.value().unwrap_or("").to_string();

      config_data.push((name.clone(), value.clone()));
    }
    Ok(Self {
      config: config_data,
    })
  }
}
