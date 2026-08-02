use git2::{Oid, Repository};

use crate::git::Git;

/// The core struct whose each value contains a `Vec<GlobRefs>` and substitute one of the dir under `.git/refs/`
#[allow(dead_code)]
pub struct Refs {
  pub heads: Vec<GlobRefs>,
}

/// Contains `name` and `oid` of the underlying items of underlying ref as per the globs (e.g, `".git/refs/heads/**` etc").
#[allow(dead_code)]
pub struct GlobRefs {
  pub name: String,
  pub oid: Oid,
}

#[allow(dead_code)]
#[allow(unused)]
impl Git {
  /// This function returns `Vec<GlobRefs>` where each entry contains `name` & `oid` of the given `glob`.
  /// E.g of glob --> `"refs/heads/**"`
  ///
  pub fn get_refs_from_glob(
    repo: &Repository,
    glob: &str,
  ) -> anyhow::Result<Vec<GlobRefs>, anyhow::Error> {
    let glob_head = repo.references_glob(glob)?;
    let mut vector = Vec::new();
    for head in glob_head {
      let head = head?;
      let name = head.name()?.to_string();
      let oid = head.target().unwrap();
      vector.push(GlobRefs { name, oid });
    }
    Ok(vector)
  }
}
