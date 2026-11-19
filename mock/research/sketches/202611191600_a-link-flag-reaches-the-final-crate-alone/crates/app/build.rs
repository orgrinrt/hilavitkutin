// The final crate's build script. SKETCH_APP_FLAGS goes to the wrapper,
// which appends it to this crate's rustc. SKETCH_APP_LINK_ARG goes to
// cargo as a link argument for the binary, with no wrapper in the way.
fn main() {
    println!("cargo::rerun-if-env-changed=SKETCH_APP_FLAGS");
    println!("cargo::rerun-if-env-changed=SKETCH_APP_LINK_ARG");
    println!("cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER");
    let flags = std::env::var("SKETCH_APP_FLAGS").unwrap_or_default();
    println!("cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={flags}");
    if let Ok(arg) = std::env::var("SKETCH_APP_LINK_ARG") {
        println!("cargo::rustc-link-arg-bins={arg}");
    }
}
