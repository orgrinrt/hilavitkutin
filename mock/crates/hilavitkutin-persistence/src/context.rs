//! PersistenceContext: bundle of references the cold store needs.
//!
//! Holds a memory provider (for mmap / allocate / protect) and a
//! string interner (for Str eviction / injection). Consumers assemble
//! the context once per session and pass it into the cold store.

use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_str::{ArenaInterner, StringInterner};

/// References to the external surfaces the persistence layer needs.
pub struct PersistenceContext<'a, M: MemoryProviderApi, A: ArenaInterner> {
    memory: &'a M,
    interner: &'a StringInterner<A>,
}

impl<'a, M: MemoryProviderApi, A: ArenaInterner> PersistenceContext<'a, M, A> {
    /// Bundle the provided references into a new context.
    pub fn new(memory: &'a M, interner: &'a StringInterner<A>) -> Self {
        Self { memory, interner }
    }

    /// Borrow the memory provider.
    pub fn memory(&self) -> &M {
        self.memory
    }

    /// Borrow the string interner.
    pub fn interner(&self) -> &StringInterner<A> {
        self.interner
    }
}
