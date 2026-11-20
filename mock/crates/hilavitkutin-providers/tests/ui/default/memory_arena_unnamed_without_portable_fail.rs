//! A build without `portable` carries no default interner arena.

use hilavitkutin_providers::MemoryArena;

fn main() {
    let _ = core::mem::size_of::<MemoryArena<64, 4>>();
}
