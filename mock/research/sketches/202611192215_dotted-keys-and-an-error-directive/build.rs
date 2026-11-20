//! Prints an error directive where `SKETCH_REFUSE` is set, and goes on
//! printing after it, to see what cargo does with the rest.

fn main() {
    println!("cargo::rerun-if-env-changed=SKETCH_REFUSE");
    println!("cargo::rerun-if-env-changed=CC");
    if std::env::var_os("SKETCH_REFUSE").is_some() {
        println!("cargo::error=sketch: Mold links ELF only, and this target is not ELF");
        println!("cargo::error=sketch: a second refusal of the same script");
    }
    println!("cargo::warning=sketch: the script ran to its end");
}
