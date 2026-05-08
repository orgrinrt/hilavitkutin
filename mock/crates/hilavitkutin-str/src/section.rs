//! Platform-specific walkers for the `.hilavitkutin_strings` linker
//! section.
//!
//! At startup (or on demand), the interner reads this section to build
//! the const-string resolution table.

#![allow(improper_ctypes)]

use crate::entry::StaticStrEntry;

/// Returns the slice of `StaticStrEntry` collected in the
/// `.hilavitkutin_strings` linker section at link time. Returns an
/// empty slice on platforms without a walker.
#[inline]
pub fn static_entries() -> &'static [StaticStrEntry] {
    imp::static_entries()
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod imp {
    use crate::entry::StaticStrEntry;

    unsafe extern "C" {
        static __start_hilavitkutin_strings: StaticStrEntry;
        static __stop_hilavitkutin_strings: StaticStrEntry;
    }

    pub fn static_entries() -> &'static [StaticStrEntry] {
        unsafe {
            let start: *const StaticStrEntry = &__start_hilavitkutin_strings;
            let stop: *const StaticStrEntry = &__stop_hilavitkutin_strings;
            if start.is_null() || stop.is_null() || stop < start {
                return &[];
            }
            let len = stop.offset_from(start) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice length from raw pointer `offset_from`; host-width usize required by `core::slice::from_raw_parts`; tracked: #72
            core::slice::from_raw_parts(start, len)
        }
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
        unsafe {
            let start: *const StaticStrEntry = &__start_hilavitkutin_strings;
            let stop: *const StaticStrEntry = &__stop_hilavitkutin_strings;
            if start.is_null() || stop.is_null() || stop < start {
                return &[];
            }
            let len = stop.offset_from(start) as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice length from raw pointer `offset_from`; host-width usize required by `core::slice::from_raw_parts`; tracked: #72
            core::slice::from_raw_parts(start, len)
        }
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
