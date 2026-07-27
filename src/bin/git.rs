use git2::{Repository, StatusOptions};
use pugit::git::{
  Git,
  current::{index, upstream::Upstream},
};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;

  match &git.upstream {
    Upstream::Found(name) => {
      let oid = Upstream::get_oid(&git.repo, &Upstream::to_branch(name, &git.repo)?)?;
      println!("upstream branch: {name}\nUpstream Oid: {oid}")
    }
    Upstream::NotFound => println!("no upstream configured"),
    Upstream::Error(e) => println!("upstream error: {e}"),
  }

  let mut opts = git2::StatusOptions::new();

  opts
    .include_untracked(true)
    .include_ignored(false)
    .renames_head_to_index(true)
    .renames_index_to_workdir(true)
    .recurse_untracked_dirs(true);

  let statuses = &git.repo.statuses(Some(&mut opts))?;
  let mut modified = Vec::new();
  let mut added = Vec::new();
  let mut deleted = Vec::new();
  let mut renamed = Vec::new();
  let mut untracked = Vec::new();

  for entry in statuses.iter() {
    let status = entry.status();
    let path = entry.path().unwrap_or("").to_string();

    if status.is_wt_new() {
      untracked.push(path);
    } else if status.is_wt_modified() || status.is_index_modified() {
      modified.push(path);
    } else if status.is_index_new() {
      added.push(path);
    } else if status.is_wt_deleted() || status.is_index_deleted() {
      deleted.push(path);
    } else if status.is_wt_renamed() || status.is_index_renamed() {
      renamed.push(path);
    }
  }

  println!("modified: {:#?}", modified);
  println!("added: {:#?}", added);
  println!("deleted:{:#?}", deleted);
  println!("Untracked : {:#?}", untracked);
  println!("Renamed : {:#?}", renamed);
  Ok(())
}
