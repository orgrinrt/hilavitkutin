use pragma_requirements_as_bounds::mask::configure;
use pragma_requirements_as_bounds::{
    Bolt,
    Cranelift,
    FastMath,
    MathPeephole,
    Mold,
    Pgo,
    Profiling,
};

type Set = (
    MathPeephole,
    (
        FastMath,
        (Profiling, (Pgo, (Bolt, (Mold, (Cranelift, ()))))),
    ),
);

fn main() {
    let mut text = String::new();
    configure::<Set, _>(&mut text).unwrap();
    println!("{text}");
}
