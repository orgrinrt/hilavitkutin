//! An arena of 2^28 entries is refused at build: every issued id stays
//! below 2^28 - 1, one short of the 28-bit `Str` id field, which is
//! ticket `the_memory_arena_s_entry_bound_is_one_short_of_the_28_bit_id_field`. The arena is
//! made in a const item so the refusal is reached by `cargo check` as well
//! as by codegen.

use hilavitkutin_providers::MemoryArena;

const _TOO_MANY: MemoryArena<0, { 1 << 28 }> = MemoryArena::new();

fn main() {}
