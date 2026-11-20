//! An arena of 2^28 + 1 entries is refused at build: its last id would be
//! 2^28, one past what the 28-bit `Str` id field holds. The arena is made
//! in a const item so the refusal is reached by `cargo check` as well as
//! by codegen.

use hilavitkutin_providers::MemoryArena;

const _TOO_MANY: MemoryArena<0, { (1 << 28) + 1 }> = MemoryArena::new();

fn main() {}
