use pragma_requirements_as_bounds::Profiling;
use pragma_requirements_as_bounds::mask::configure;

type Set = (Profiling, ());

fn main() {
    let mut text = String::new();
    configure::<Set, _>(&mut text).unwrap();
    println!("{text}");
}
