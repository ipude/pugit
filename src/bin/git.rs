use pugit::git::{
  Git,
  current::{
    local::Local,
    upstream::{self, Upstream},
  },
};

fn main() -> anyhow::Result<()> {
  let git = Git::new("~/impl/rust/pugit/")?;

  match &git.upstream {
    Upstream::Error(e) => println!("upstream error: {e}"),
    Upstream::None => println!("no upstream configured"),
    Upstream::Branch(name) => println!("upstream branch: {name}"),
    Upstream::Oid(oid) => println!("upstream oid: {oid}"),
  }

  git.upstream.get_oid();
  Ok(())
}
