fn main() {
    println!("cargo::rerun-if-env-changed=SKETCH_LINK_ARGS");
    if let Ok(args) = std::env::var("SKETCH_LINK_ARGS") {
        for arg in args.split(' ').filter(|arg| !arg.is_empty()) {
            println!("cargo::rustc-link-arg={arg}");
        }
    }
}
