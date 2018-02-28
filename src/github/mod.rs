extern crate git2;

use git2::Repository;

pub fn clone_remote_repo() -> {
  let url = "https://github.com/alexcrichton/git2-rs";

  // download to cache in ~/.hydra
  let repo = match Repository::clone(url, ".hydra") {
      Ok(repo) => repo,
      Err(e) => panic!("failed to clone: {}", e),
    };
}