fn main() {
    sketch_script_lib::script!(Default);
    // Whether cargo hands a build script the variable at all, with none set by the caller.
    println!(
        "cargo::warning=CARGO_HOME reaches the script: {}",
        std::env::var_os("CARGO_HOME").is_some()
    );
}
