use std::process::Command;

fn main() {
  let env = "TDATE_GIT_HASH";

  let git_hash = Command::new("git")
    .args(["rev-parse", "HEAD"])
    .output()
    .ok()
    .filter(|output| output.status.success())
    .and_then(|x| String::from_utf8(x.stdout).ok())
    .map(|hash| hash[..8].to_owned());

  if let Some(hash) = git_hash {
    let dirty = Command::new("git")
      .args(["diff", "--stat"])
      .output()
      .ok()
      .filter(|output| output.status.success())
      .map(|output| !matches!(output.stdout.len(), 0));

    match dirty {
      Some(true) => println!("cargo:rustc-env={}={}-dirty", env, hash),
      Some(false) => println!("cargo:rustc-env={}={}", env, hash),
      _ => unreachable!("How can we have a git hash, yet not know if the tree is dirty?"),
    }
  } else {
    println!("cargo:rustc-env={}=unknown", env);
  }
}
