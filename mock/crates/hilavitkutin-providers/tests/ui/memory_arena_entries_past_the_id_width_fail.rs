//! An arena of 2^28 entries is refused at build: its last id would be
//! 2^28 - 1, the id the overflow sentinel takes once masked to 28 bits,
//! so an overflow handle would resolve to that slot's string. The arena is
//! made in a const item so the refusal is reached by `cargo check` as well
//! as by codegen.

use hilavitkutin_providers::MemoryArena;

const _TOO_MANY: MemoryArena<0, { 1 << 28 }> = MemoryArena::new();

fn main() {}
