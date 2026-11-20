use flat_tuple_set::{FastMath, ThinLto, checked, crate_reach};

const _: () = checked::<(FastMath, ThinLto)>();
const _: () = crate_reach::<(FastMath, ThinLto)>();

fn main() {}
