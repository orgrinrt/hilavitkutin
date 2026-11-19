// The dylib crate alone hands the flag to the workspace wrapper this
// sketch inherits from `mock/.cargo/`.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Cprefer-dynamic");
}
