use std::{
  sync::{Arc, atomic::Ordering},
  time::Duration,
};

use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
use pugit::git::Git;

use crate::watcher::WatchSignals;

#[allow(dead_code)]
#[allow(unused_allocation)]
#[allow(unused_variables)]
fn debounced_watcher() -> anyhow::Result<()> {
  let signal = Arc::new(WatchSignals::default());
  let repo = Git::string_to_path("~/impl/rust/pugit/.git/")?;

  let ref_dir = repo.join("refs/");
  let head = repo.join("HEAD");
  let origin_head = repo.join("ORIG_HEAD");
  let config = repo.join("config");
  let description = repo.join("description");
  let index = repo.join("index");
  let packed_refs = repo.join("packed-refs");

  let mut debouncer = new_debouncer(Duration::from_millis(500), {
    let signals = signal.clone();
    let path = repo.clone();
    move |res: DebounceEventResult| {
      let Ok(events) = res else { return };
      for e in events {
        if e.path.starts_with(path.join("refs/heads/")) {
          signals
            .refs_head_changed
            .store(true, std::sync::atomic::Ordering::Relaxed);
        } else if e.path.starts_with(path.join("refs/tags/")) {
          signals
            .refs_tags_changed
            .store(true, std::sync::atomic::Ordering::Relaxed);
        } else if e.path.starts_with(path.join("refs/remotes/")) {
          signals
            .refs_remotes_changed
            .store(true, std::sync::atomic::Ordering::Relaxed);
        }

        match e.path.file_name().and_then(|p| p.to_str()) {
          Some("HEAD") => signals
            .head_changed
            .store(true, std::sync::atomic::Ordering::Relaxed),
          Some("ORIG_HEAD") => signals.orig_head_changed.store(true, Ordering::Relaxed),
          Some("config") => signals.config_changed.store(true, Ordering::Relaxed),
          Some("index") => signals.index_changed.store(true, Ordering::Relaxed),
          Some("packed-refs") => signals.packed_refs_changed.store(true, Ordering::Relaxed),
          _ => {}
        }
      }
    }
  })?;

  for i in ["HEAD", "ORIG_HEAD", "config", "index", "packed-refs"] {
    if repo.join(i).try_exists()? {
      debouncer
        .watcher()
        .watch(&repo.join(i), notify::RecursiveMode::NonRecursive)?;
    }
  }

  for j in ["refs/heads/", "refs/tags/", "refs/remotes"] {
    if repo.join(j).try_exists()? {
      debouncer
        .watcher()
        .watch(&repo.join(j), notify::RecursiveMode::Recursive)?;
    }
  }
  Ok(())
}
