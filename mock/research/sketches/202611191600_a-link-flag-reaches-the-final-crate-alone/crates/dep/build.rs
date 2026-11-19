// The library's build script. It hands the wrapper the flags named in
// SKETCH_DEP_FLAGS, which is unset in every run but the one that shows
// what a link flag on a library costs.
fn main() {
    println!("cargo::rerun-if-env-changed=SKETCH_DEP_FLAGS");
    println!("cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER");
    let flags = std::env::var("SKETCH_DEP_FLAGS").unwrap_or_default();
    println!("cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={flags}");
}
