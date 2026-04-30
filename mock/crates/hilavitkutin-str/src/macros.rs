//! `str_const!()` — compile-time string registration + handle derivation.

/// Register a string literal in the `.hilavitkutin_strings` linker section
/// and return its const-origin `Str` handle.
///
/// One macro invocation produces both sides of the registration:
/// the linker-section entry (read at startup by the section walker)
/// and the handle value the call site uses immediately.
#[macro_export]
macro_rules! str_const {
    ($s:literal) => {{
        #[used]
        #[cfg_attr(
            any(target_os = "linux", target_os = "android"),
            unsafe(link_section = "hilavitkutin_strings")
        )]
        #[cfg_attr(
            any(target_os = "macos", target_os = "ios"),
            unsafe(link_section = "__DATA,__hvkstr")
        )]
        static __ENTRY: $crate::StaticStrEntry = $crate::StaticStrEntry {
            hash: $crate::Str::__make(::arvo_bits::Bits::<28>::from_raw(
                ($crate::const_fnv1a($s) & 0x0FFF_FFFF) as u32, // lint:allow(no-bare-numeric) reason: hash 28-bit-masked then narrowed to Bits<28, Hot> u32 container; arvo lacks a Widen counterpart to Narrow; tracked: #290
            )),
            value: $s,
        };
        $crate::Str::__make(::arvo_bits::Bits::<28>::from_raw(
            ($crate::const_fnv1a($s) & 0x0FFF_FFFF) as u32, // lint:allow(no-bare-numeric) reason: hash truncated to 28 bits then narrowed to Bits<28, Hot> u32 container; arvo lacks a Widen counterpart to Narrow; tracked: #290
        ))
    }};
}
