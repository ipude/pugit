use pugit::git::{Git, global::remote};

fn main() -> anyhow::Result<(), anyhow::Error>{
  let git = Git::new("~/tmp")?;
  let remote = Git::get_remotes(&git.repo)?;
  for i in remote.iter() {
    println!("REMOTE   : {}", i.name);
    println!("URL      : {}", i.url);
    println!("PUSH URL : {}", i.pushurl);
  }
  Ok(())
}
