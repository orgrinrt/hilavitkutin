fn main() {
    println!(
        "CC as cargo set it for this build: {}",
        option_env!("CC").unwrap_or("(unset)")
    );
}
