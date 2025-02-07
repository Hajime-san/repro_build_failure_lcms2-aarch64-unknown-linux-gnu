use std::env;

fn main() {
  let ci = env::var("CI");
  if ci.is_ok() == true {
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    println!("cargo:warning=Hello, ARM64 Linux!");
  }
}
