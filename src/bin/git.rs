use git2::{Repository, StatusOptions, Statuses};
use pugit::git::Git;

fn index_char(s: git2::Status) -> char {
  use git2::Status;
  if s.contains(Status::INDEX_NEW) {
    'A'
  } else if s.contains(Status::INDEX_MODIFIED) {
    'M'
  } else if s.contains(Status::INDEX_DELETED) {
    'D'
  } else if s.contains(Status::INDEX_RENAMED) {
    'R'
  } else if s.contains(Status::INDEX_TYPECHANGE) {
    'T'
  } else {
    ' '
  }
}

fn wt_char(s: git2::Status) -> char {
  use git2::Status;
  if s.contains(Status::WT_MODIFIED) {
    'M'
  } else if s.contains(Status::WT_DELETED) {
    'D'
  } else if s.contains(Status::WT_TYPECHANGE) {
    'T'
  } else if s.contains(Status::WT_RENAMED) {
    'R'
  } else {
    ' '
  }
}

fn conflict_code(index: &git2::Index, path: &str) -> Option<&'static str> {
  for c in index.conflicts().ok()?.flatten() {
    if c
      .our
      .as_ref()
      .or(c.their.as_ref())
      .or(c.ancestor.as_ref())
      .map(|e| e.path == path.as_bytes())
      != Some(true)
    {
      continue;
    }
    return Some(
      match (c.ancestor.is_some(), c.our.is_some(), c.their.is_some()) {
        (true, false, false) => "DD",
        (false, true, false) => "AU",
        (true, true, false) => "UD",
        (false, false, true) => "UA",
        (true, false, true) => "DU",
        (false, true, true) => "AA",
        (true, true, true) => "UU",
        _ => "??",
      },
    );
  }
  None
}

fn get_statuses<'repo>(repo: &'repo Repository) -> anyhow::Result<Statuses<'repo>, anyhow::Error> {
  let mut opts = StatusOptions::new();
  opts
    .include_untracked(true)
    .include_ignored(false)
    .recurse_untracked_dirs(true)
    .exclude_submodules(false)
    .renames_head_to_index(true)
    .renames_index_to_workdir(true)
    .no_refresh(false);

  Ok(repo.statuses(Some(&mut opts))?)
}

fn main() -> anyhow::Result<(), anyhow::Error> {
  let git = Git::new("~/impl/rust/pugit/")?;
  let repo = &git.repo;
  let index = repo.index()?;
  let statuses = get_statuses(repo)?;

  for entry in statuses.iter() {
    let path = entry.path().unwrap_or("<invalid utf-8>");
    let status = entry.status();

    let code = if status.contains(git2::Status::WT_NEW)
      && !status.intersects(
        git2::Status::INDEX_NEW
          | git2::Status::INDEX_MODIFIED
          | git2::Status::INDEX_DELETED
          | git2::Status::INDEX_RENAMED
          | git2::Status::INDEX_TYPECHANGE,
      ) {
      "??".to_string()
    } else if status.is_conflicted() {
      conflict_code(&index, path).unwrap_or("UU").to_string()
    } else {
      format!("{}{}", index_char(status), wt_char(status))
    };

    println!("{code} {path}");
  }
  Ok(())
}
