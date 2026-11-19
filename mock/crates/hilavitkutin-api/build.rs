//! Builds the crate through `hilavitkutin-build`, which chooses its rustc flags.

fn main() {
    hilavitkutin_build::script!(Default);
}
