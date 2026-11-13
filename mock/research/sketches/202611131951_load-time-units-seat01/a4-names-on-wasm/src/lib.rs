#![feature(const_trait_impl)]
//! A const name declared and resolved inside one image, the same image, so
//! no seam is involved. On linux the section holds the entry and the name
//! resolves; on wasm32 `static_entries` is `&[]` and the name cannot.
//!
//! Exports, plain C ABI:
//! - `own_name_resolves() -> u32`: 1 when `str_const!("host.kind.pos")`
//!   resolves to its own text through `StringInterner`, else 0.
//! - `section_entries() -> usize`: how many const entries the image sees.

use hilavitkutin_str::{ArenaInterner, Str, StringInterner, static_entries, str_const};
use notko::Maybe;

struct NoArena;
impl ArenaInterner for NoArena {
    fn arena_intern(&self, _s: &str) -> u32 {
        u32::MAX
    }

    fn arena_resolve(&self, _id: u32) -> &str {
        "(arena)"
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn own_name_resolves() -> u32 {
    let own: Str = str_const!("host.kind.pos");
    let interner = StringInterner::new(NoArena);
    match interner.resolve(own) {
        Maybe::Is("host.kind.pos") => 1,
        _ => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn section_entries() -> usize {
    static_entries().len()
}
