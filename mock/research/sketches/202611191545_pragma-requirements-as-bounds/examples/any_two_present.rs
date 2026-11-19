use pragma_requirements_as_bounds::bounds::configure;
use pragma_requirements_as_bounds::{Bolt, Pgo, Profiling};

// A coherent set: Profiling stands on Pgo or Bolt, and both are here.
type Set = (Profiling, (Pgo, (Bolt, ())));

fn main() {
    let mut text = String::new();
    configure::<Set, _, _>(&mut text).unwrap();
    println!("{text}");
}
