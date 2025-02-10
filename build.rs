use std::{env, process::Command};

fn main() {
  if let Ok(ci) = env::var("CI") {
    if ci == "true" {
      let machine_path = Command::new("clang")
        .arg("-dumpmachine")
        .output()
        .expect("failed to execute process");
      let machine_path = std::str::from_utf8(&machine_path.stdout).unwrap();
      println!(
        "cargo::rustc-env=PKG_CONFIG_LIBDIR=/sysroot/usr/lib/{}",
        machine_path
      );
    }
  }
}
