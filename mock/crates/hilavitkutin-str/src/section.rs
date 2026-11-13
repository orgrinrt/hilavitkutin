//! Platform-specific walkers for the `.hilavitkutin_strings` linker
//! section.
//!
//! At startup (or on demand), the interner reads this section to build
//! the const-string resolution table.
//!
//! The Linux linker defines `__start_<section>` and `__stop_<section>` only
//! when at least one input object holds a section of that name, so a link
//! with no `str_const!` anywhere would leave both undefined and fail. The
//! crate therefore places one entry itself, the empty string, in the section
//! on the platforms that have a walker. The walker references it, which
//! pulls it into every link that reads the section, and it returns the empty
//! slice for a section that does not contain it.

#![allow(improper_ctypes)]

use crate::entry::StaticStrEntry;

/// Returns the slice of `StaticStrEntry` collected in the
/// `.hilavitkutin_strings` linker section at link time. Returns an
/// empty slice on platforms without a walker.
#[inline]
pub fn static_entries() -> &'static [StaticStrEntry] {
    imp::static_entries()
}

/// The crate's own entry in the section: the empty string. It keeps the
/// section present in every link and marks which section the walker's
/// bounds belong to.
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
))]
#[used]
#[cfg_attr(
    any(target_os = "linux", target_os = "android"),
    unsafe(link_section = "hilavitkutin_strings")
)]
#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    unsafe(link_section = "__DATA,__hvkstr")
)]
static SECTION_ANCHOR: StaticStrEntry = StaticStrEntry::new("");

/// The entries between two section bounds, or none when the bounds are
/// reversed or do not contain the anchor.
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
))]
unsafe fn between(
    start: *const StaticStrEntry,
    stop: *const StaticStrEntry,
) -> &'static [StaticStrEntry] {
    let anchor: *const StaticStrEntry = &SECTION_ANCHOR;
    if start.is_null() || stop.is_null() || stop < start || anchor < start || anchor >= stop {
        return &[];
    }
    unsafe {
        let len = stop.offset_from(start) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice length from raw pointer `offset_from`; host-width usize required by `core::slice::from_raw_parts`; tracked: #72
        core::slice::from_raw_parts(start, len)
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod imp {
    use crate::entry::StaticStrEntry;

    unsafe extern "C" {
        static __start_hilavitkutin_strings: StaticStrEntry;
        static __stop_hilavitkutin_strings: StaticStrEntry;
    }

    pub fn static_entries() -> &'static [StaticStrEntry] {
        unsafe { super::between(&__start_hilavitkutin_strings, &__stop_hilavitkutin_strings) }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod imp {
    use crate::entry::StaticStrEntry;

    // Mach-O linker provides these automatic sentinels for any
    // `__DATA,<sectname>` section via the
    // `section$start$__DATA$<sectname>` / `section$end$__DATA$<sectname>`
    // symbol mangling.
    unsafe extern "C" {
        #[link_name = "\x01section$start$__DATA$__hvkstr"]
        static __start_hilavitkutin_strings: StaticStrEntry;
        #[link_name = "\x01section$end$__DATA$__hvkstr"]
        static __stop_hilavitkutin_strings: StaticStrEntry;
    }

    pub fn static_entries() -> &'static [StaticStrEntry] {
        unsafe { super::between(&__start_hilavitkutin_strings, &__stop_hilavitkutin_strings) }
    }
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
)))]
mod imp {
    use crate::entry::StaticStrEntry;

    pub fn static_entries() -> &'static [StaticStrEntry] {
        &[]
    }
}
