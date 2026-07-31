use pugit::git::{Git};

fn main() -> anyhow::Result<(), anyhow::Error>{
  let git = Git::new("~/tmp")?;
  let branches = git.repo.branches(Some(git2::BranchType::Local))?;

  for branch in branches {
    let (branch, _btype) = branch?;
    println!("Name: {}", branch.name()?.unwrap_or("<None>"));
  }
  Ok(())
}
