fn main() {
    let args: [&str; 1] = if cfg!(target_os = "macos") {
        [&"-DPREFIX"]
    } else {
        [&""]
    };
    nasm_rs::compile_library_args("asm", &["src/x86/repro.asm"], &args);
    println!("cargo:rerun-if-changed={}", "src/x86/repro.asm");
    println!("cargo:rustc-link-lib=static=asm");
}
