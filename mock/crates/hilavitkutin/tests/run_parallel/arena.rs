//! The arena, the input column and the hint every `run_parallel` fixture shares.
//!
//! The provider is the shared `stable_provider`: the scheduler records where
//! every store it reserves lives and then moves, so the buffer cannot sit
//! inline in the provider (`tests/stable_provider/mod.rs`).

#[path = "../stable_provider/mod.rs"]
mod stable_provider;

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_providers::ArenaColumnStorage;
pub use stable_provider::stable_provider;

pub fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// The host-populated input every producer reads. A producer reads it rather
/// than synthesising a value from the `each()` index: that index is
/// morsel-relative, so a unit sourcing from it breaks once a phase is split
/// across cores.
#[derive(Copy, Clone, Default)]
pub struct Inv(pub u32);

pub type OneIn = Cons<Column<Inv>, Empty>;

pub type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);
