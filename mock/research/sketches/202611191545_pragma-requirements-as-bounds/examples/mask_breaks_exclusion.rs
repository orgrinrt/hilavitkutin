use pragma_requirements_as_bounds::mask::configure;
use pragma_requirements_as_bounds::{Mold, Wild};

type Set = (Mold, (Wild, ()));

fn main() {
    let mut text = String::new();
    configure::<Set, _>(&mut text).unwrap();
    println!("{text}");
}
