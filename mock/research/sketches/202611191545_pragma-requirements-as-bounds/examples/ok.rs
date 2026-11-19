use pragma_requirements_as_bounds::bounds::configure;
use pragma_requirements_as_bounds::{Cranelift, FastMath, MathPeephole, Mold, Pgo, Profiling};

type Set = (
    MathPeephole,
    (FastMath, (Profiling, (Pgo, (Mold, (Cranelift, ()))))),
);

fn main() {
    let mut text = String::new();
    configure::<Set, _, _>(&mut text).unwrap();
    println!("{text}");
}
