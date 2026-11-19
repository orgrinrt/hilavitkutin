pub fn main() {
    println!("cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER");
    println!("cargo::warning=the shared build script ran");
}
