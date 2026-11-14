//! The arena the scheduler builder tests build on, and the default-capacity
//! column store over it.
//!
//! The provider is the shared `stable_provider`: the scheduler records where
//! every store it reserves lives and then moves, so the buffer cannot sit
//! inline in the provider (`tests/stable_provider/mod.rs`).

#[path = "../stable_provider/mod.rs"]
mod stable_provider;

use hilavitkutin_providers::ArenaColumnStorage;
pub use stable_provider::{StaticBumpMemory, stable_provider};

/// The default-capacity arena store over a 4 KiB provider. The return type
/// omits `D`, applying the `Dim<256>` default so inference is anchored at the
/// `build` call sites.
pub fn store() -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(stable_provider(4096, 0))
}
