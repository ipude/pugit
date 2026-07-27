use std::path::PathBuf;
use git2::Repository;

/// `repo.statuses()` returns `Statuses`
/// Statuses returns files with status.
/// This enum is to mark status of a file that contains status.
#[allow(dead_code)]
pub enum FileState {
  Modified,
  Renamed,
  Deleted,
  Added,
  Untracked,
  Conflicted,
  Ignored,
  Empty,
  TypeChanged,
}

/// Status of a File including:
/// Path and FileState
/// This field must be used as -> Vec<FileStatus>
#[allow(dead_code)]
pub struct FileStatus {
  pub path: PathBuf,
  pub state: FileState,
}

#[allow(dead_code)]
impl FileStatus {
  fn insert(path: String, state: FileState) -> Self {
    let path = PathBuf::from(path);
    Self { path, state }
  }
}

#[allow(dead_code)]
impl FileState {
  fn collect(repo: &Repository) -> anyhow::Result<Vec<FileStatus>> {
    let mut status_opts = git2::StatusOptions::new();
    status_opts
      .include_untracked(true)
      .include_ignored(true)
      .renames_head_to_index(true)
      .renames_index_to_workdir(true)
      .recurse_untracked_dirs(true);
    let statuses = repo.statuses(Some(&mut status_opts))?;
    let mut vector = Vec::new();
    for entry in statuses.iter() {
      let status = entry.status();
      let path = entry.path().unwrap_or("").to_string();

      if status.is_wt_new() {
        vector.push(FileStatus::insert(path, Self::Untracked))
      } else if status.is_wt_modified() || status.is_index_modified() {
        vector.push(FileStatus::insert(path, Self::Modified))
      } else if status.is_index_new() {
        vector.push(FileStatus::insert(path, Self::Added))
      } else if status.is_wt_deleted() || status.is_index_deleted() {
        vector.push(FileStatus::insert(path, Self::Deleted))
      } else if status.is_wt_renamed() || status.is_index_renamed() {
        vector.push(FileStatus::insert(path, Self::Renamed))
      } else if status.is_conflicted() {
        vector.push(FileStatus::insert(path, Self::Conflicted));
      } else if status.is_ignored() {
        vector.push(FileStatus::insert(path, Self::Ignored));
      } else if status.is_empty() {
        vector.push(FileStatus::insert(path, Self::Empty));
      } else if status.is_index_typechange() || status.is_wt_typechange() {
        vector.push(FileStatus::insert(path, Self::TypeChanged));
      }
    }
    Ok(vector)
  }
}
