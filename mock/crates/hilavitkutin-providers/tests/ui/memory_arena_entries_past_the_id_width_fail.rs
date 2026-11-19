//! An arena of more entries than the 28-bit id can address is refused at
//! build, since its ids past 2^28 would alias. The arena is made in a const
//! item so the refusal is reached by `cargo check` as well as by codegen.

use hilavitkutin_providers::MemoryArena;

const _TOO_MANY: MemoryArena<0, { (1 << 28) + 1 }> = MemoryArena::new();

fn main() {}
