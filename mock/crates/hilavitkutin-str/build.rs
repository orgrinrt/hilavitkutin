//! Builds the crate through `hilavitkutin-build`, which chooses its rustc flags.

fn main() {
    let mut out = String::new();
    hilavitkutin_build::configure(|key| std::env::var(key).ok().into(), &mut out)
        .expect("a String takes every write");
    print!("{out}");
}
