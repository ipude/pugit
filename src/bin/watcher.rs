use std::{
  sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
  },
  time::Duration,
};

use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use pugit::git::Git;

fn main() -> anyhow::Result<()> {
  let dr1_changed = Arc::new(AtomicBool::new(false));
  let file_changed = Arc::new(AtomicBool::new(false));

  let dir1_cloned = Arc::clone(&dr1_changed);
  let file_cloned = Arc::clone(&file_changed);

  let mut debouncer = new_debouncer(
    Duration::from_millis(500),
    None,
    move |result: DebounceEventResult| match result {
      Ok(events) => {
        for debounced in events {
          for path in &debounced.event.paths {
            if path.starts_with(".git") {
              dir1_cloned.store(true, Ordering::Release);
            } else if path.starts_with("src") {
              file_cloned.store(true, Ordering::Release);
            }
          }
        }
      }
      Err(errors) => {
        for error in errors {
          println!("{error}")
        }
      }
    },
  )?;

  debouncer.watch(
    Git::string_to_path("~/tmps/dir1/")?,
    notify::RecursiveMode::NonRecursive,
  )?;
  debouncer.watch(
    Git::string_to_path("~/tmps/readme.md")?,
    notify::RecursiveMode::NonRecursive,
  )?;

  Ok(())
}
