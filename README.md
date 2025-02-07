# repro_build_failure_lcms2-aarch64-unknown-linux-gnu

This repository contains a minimal reproduciton for building failure the [lcms2](https://github.com/kornelski/rust-lcms2).

Specifically, when performing a sysroot build with the `ubuntu-22.04-arm` runner in GitHub Actions and the `aarch64-unknown-linux-gnu` Rust build target, the following error is output.

```bash
...
= note: ld.lld-19: error: undefined symbol: __libc_csu_init
          >>> referenced by /sysroot/usr/lib/aarch64-linux-gnu/Scrt1.o:(.text+0x20)
          >>> referenced by /sysroot/usr/lib/aarch64-linux-gnu/Scrt1.o:(.text+0x24)
          
          ld.lld-19: error: undefined symbol: __libc_csu_fini
          >>> referenced by /sysroot/usr/lib/aarch64-linux-gnu/Scrt1.o:(.text+0x28)
          >>> referenced by /sysroot/usr/lib/aarch64-linux-gnu/Scrt1.o:(.text+0x2C)
          clang-19: error: linker command failed with exit code 1 (use -v to see invocation)
...
```