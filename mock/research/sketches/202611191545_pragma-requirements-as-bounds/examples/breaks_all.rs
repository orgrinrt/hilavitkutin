use pragma_requirements_as_bounds::MathPeephole;
use pragma_requirements_as_bounds::bounds::configure;

type Set = (MathPeephole, ());

fn main() {
    let mut text = String::new();
    configure::<Set, _, _>(&mut text).unwrap();
    println!("{text}");
}
