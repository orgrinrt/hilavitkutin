// Prints the C compiler variables as the build script sees them, which
// is where a crate that compiles C reads them.
fn main() {
    for key in ["CC", "CXX", "AR"] {
        println!("cargo::rerun-if-env-changed={key}");
        match std::env::var(key) {
            Ok(value) => println!("cargo::warning={key}={value}"),
            Err(_) => println!("cargo::warning={key} is unset"),
        }
    }
}
