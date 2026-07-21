use std::sync::atomic::AtomicBool;
pub mod core;
#[allow(dead_code)]
#[derive(Default)]
pub struct WatchSignals {
  head_changed: AtomicBool,
  orig_head_changed: AtomicBool,
  config_changed: AtomicBool,
  description_changed: AtomicBool,
  index_changed: AtomicBool,
  packed_refs_changed: AtomicBool,
  refs_head_changed: AtomicBool,
  refs_tags_changed: AtomicBool,
  refs_remotes_changed: AtomicBool,
}
