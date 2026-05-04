// no_std cdylibs on macOS need libSystem re-linked because `-nodefaultlibs`
// strips it and `dyld_stub_binder` becomes undefined. Re-add the System
// dylib so dyld can resolve the stub binder at load time.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-lib=dylib=System");
    }
}
