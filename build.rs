use std::env;

fn main() {
  #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
  println!("Hello, ARM64 Linux!");

  let target = env::var("CARGO_BUILD_TARGET");
  match target {
    Ok(target) => {
      println!("build.target: {}", target);
    }
    Err(_) => {}
  }
}
