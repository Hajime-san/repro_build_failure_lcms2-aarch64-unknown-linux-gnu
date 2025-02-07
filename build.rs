use std::env;
use std::process::Command;

fn main() {
  let ci = env::var("CI");
  if ci.is_ok() == true {
    // #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    // println!("cargo:rustc-env=PKG_CONFIG_SYSROOT_DIR=/sysroot/usr/lib/aarch64-linux-gnu/pkgconfig");
  }
}
