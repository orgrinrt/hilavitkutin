use flat_tuple_set::{
    Bolt,
    FastMath,
    MathPeephole,
    Mold,
    ParallelCodegen,
    Pgo,
    Profiling,
    Set,
    checked,
    crate_reach,
};

type Mine = (MathPeephole, FastMath, Profiling, Pgo, Bolt);
type Small = (Mold, ParallelCodegen<4>);

const _: () = checked::<Mine>();
const _: () = crate_reach::<Mine>();
const _: () = checked::<Small>();

fn main() {
    let mut text = String::new();
    Mine::write_flags(&mut text).unwrap();
    Small::write_flags(&mut text).unwrap();
    println!("{text}");
}
