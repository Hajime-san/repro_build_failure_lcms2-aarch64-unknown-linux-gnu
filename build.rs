use std::env;

fn main() {
  let ci = env::var("CI");
  #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
  if ci.is_ok() == true {
    println!("cargo:warning=this is a workaround for aarch64 linux");
    // println!("cargo:rustc-env=PKG_CONFIG_SYSROOT_DIR=/sysroot/usr/lib/aarch64-linux-gnu/pkgconfig");
    // println!("cargo:rustc-env=LCMS2_INCLUDE_DIR=/usr/lib/aarch64-linux-gnu");
  }
}
