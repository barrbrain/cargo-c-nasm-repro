To reproduce:
```sh
cargo install cargo-c --version 0.2.2
cargo cbuild --release
```
Resulting error log:
```
   Compiling nasm-rs v0.1.4
     Running `rustc --crate-name nasm_rs /home/barrbrain/.cargo/registry/src/github.com-1ecc6299db9ec823/nasm-rs-0.1.4/src/lib.rs --color never --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C metadata=27c4284caf027c96 -C extra-filename=-27c4284caf027c96 --out-dir /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps -L dependency=/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps --cap-lints allow`
   Compiling cargo-c-nasm-repro v0.1.0 (/home/barrbrain/Workspace/cargo-c-nasm-repro)
     Running `rustc --edition=2018 --crate-name build_script_build build.rs --color never --crate-type bin --emit=dep-info,link -C opt-level=3 -C metadata=bab0fafb9e8206fd -C extra-filename=-bab0fafb9e8206fd --out-dir /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-bab0fafb9e8206fd -L dependency=/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps --extern nasm_rs=/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps/libnasm_rs-27c4284caf027c96.rlib`
     Running `/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-bab0fafb9e8206fd/build-script-build`
     Running `rustc --edition=2018 --crate-name cargo_c_nasm_repro src/lib.rs --color never --crate-type lib --crate-type staticlib --emit=dep-info,link -C opt-level=3 --crate-type staticlib --crate-type cdylib --cfg cargo_c -C link-arg=-Wl,-soname,libcargo-c-nasm-repro.so.0 -C metadata=8349941d615b1704 -C extra-filename=-8349941d615b1704 --out-dir /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps -L dependency=/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps -L /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-9c9bc7bb60092581/out -l static=asm`
error: linking with `x86_64-pc-linux-gnu-gcc` failed: exit code: 1
  |
  = note: "x86_64-pc-linux-gnu-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib" "/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps/cargo_c_nasm_repro-8349941d615b1704.cargo_c_nasm_repro.ekmg50ex-cgu.0.rcgu.o" "-o" "/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps/libcargo_c_nasm_repro-8349941d615b1704.so" "-Wl,--version-script=/tmp/rustcmoJS5n/list" "/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps/cargo_c_nasm_repro-8349941d615b1704.43hs5yc35dpwee44.rcgu.o" "-Wl,--gc-sections" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps" "-L" "/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-9c9bc7bb60092581/out" "-L" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lasm" "-Wl,--no-whole-archive" "-Wl,--start-group" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libstd-d00fe5b0df1921ae.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-84dd020faabf1ccb.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-40604f4835d35ca1.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-7bf590d43347c657.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-8244a9d462c42ae4.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-db2714e7fc28f2c1.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-708fc00aaebad4c1.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d4cf36941b79006c.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-eed9ecded7746a53.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-62dbc9ac1c0452b5.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-e636a26433720b04.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-c7e05ccb385e06c5.rlib" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcore-4cd09ee2b9a64664.rlib" "-Wl,--end-group" "/usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-40e1aed7e90a73ac.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-shared" "-Wl,-soname,libcargo-c-nasm-repro.so.0"
  = note: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../x86_64-pc-linux-gnu/bin/ld: /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-9c9bc7bb60092581/out/libasm.a(repro.o): warning: relocation against `cargo_c_nasm_repro_table' in readonly section `.text'
          /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../x86_64-pc-linux-gnu/bin/ld: /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-9c9bc7bb60092581/out/libasm.a(repro.o): relocation R_X86_64_PC32 against symbol `cargo_c_nasm_repro_table' can not be used when making a shared object; recompile with -fPIC
          /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../x86_64-pc-linux-gnu/bin/ld: final link failed: Bad value
          collect2: error: ld returned 1 exit status


error: aborting due to previous error

error: Could not compile `cargo-c-nasm-repro`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name cargo_c_nasm_repro src/lib.rs --color never --crate-type lib --crate-type staticlib --emit=dep-info,link -C opt-level=3 --crate-type staticlib --crate-type cdylib --cfg cargo_c -C link-arg=-Wl,-soname,libcargo-c-nasm-repro.so.0 -C metadata=8349941d615b1704 -C extra-filename=-8349941d615b1704 --out-dir /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps -L dependency=/home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/deps -L /home/barrbrain/Workspace/cargo-c-nasm-repro/target/release/build/cargo-c-nasm-repro-9c9bc7bb60092581/out -l static=asm` (exit code: 1)
Error: Kind(InvalidInput)
```
