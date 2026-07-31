use pugit::git::Git;

fn main() -> anyhow::Result<(), anyhow::Error>{
  let git = Git::new("~/tmp")?;
  let remotes_array = git.repo.remotes()?;
  for remote_name in remotes_array.iter().flatten() {
    let remote = git.repo.find_remote(remote_name.unwrap())?;
    println!("Remote:  {}\nUrl:     {:?},\nPushurl: {}", remote.name()?.unwrap(), remote.url()?, remote.pushurl()?.unwrap_or("None"));
  }
  Ok(())
}
