fn main() {
    println!(
        "flag arrived: {}, deterministic: {}",
        cfg!(sketch_flag_arrived),
        cfg!(sketch_deterministic)
    );
}
