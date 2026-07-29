use pugit::git::Git;

#[allow(dead_code)]
#[allow(unused)]
fn main() -> anyhow::Result<(), anyhow::Error> {
  let git = Git::new("~/tmp/")?;
  let refs = git.repo.references_glob("refs/heads/**")?;
  for r in refs {
    let r = r?;
    let name = r.name().unwrap_or("").to_string();
    let target = r.target();
    println!("{name} --> {:?}", target)
  }

  let remotes = git.repo.remotes()?;
  for i in remotes.iter() {
    let i = i?.unwrap_or("");
    let remote = git.repo.find_remote(i)?.name()?.unwrap_or("").to_string();
    println!("{i}");
    println!("{remote}");
  }

  let main = git.repo.find_branch("main", git2::BranchType::Local)?;
  let new = git.repo.find_branch("new", git2::BranchType::Local)?;

  let (ahead, behind) = git
    .repo
    .graph_ahead_behind(main.get().target().unwrap(), new.get().target().unwrap())?;

  if ahead > behind {
    println!(
      "Branch: {} is ahead by {ahead} commits from  Branch: {}",
      main.name()?.unwrap_or("<no name>").to_uppercase(),
      new.name()?.unwrap_or("<no name>").to_uppercase()
    )
  }

  Ok(())
}
