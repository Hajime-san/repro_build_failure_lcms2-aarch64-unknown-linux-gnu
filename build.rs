use std::env;

fn main() {
  if let Ok(ci) = env::var("CI") {
    if ci == "true" {
      println!(
        "cargo::rustc-env=PKG_CONFIG_LIBDIR=/sysroot/usr/lib/aarch64-linux-gnu"
      );
    }
  }
}
