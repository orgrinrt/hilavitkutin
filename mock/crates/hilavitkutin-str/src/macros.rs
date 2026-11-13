//! `str_const!()`: compile-time string registration + handle derivation.

/// Register a string literal in the `.hilavitkutin_strings` linker section
/// and return its const-origin `Str` handle.
///
/// One macro invocation produces both sides of the registration:
/// the linker-section entry (read at startup by the section walker)
/// and the handle value the call site uses immediately. Both come from
/// `StaticStrEntry::new`, so the handle is the entry's own `hash`.
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
        static __ENTRY: $crate::StaticStrEntry = $crate::StaticStrEntry::new($s);
        $crate::StaticStrEntry::new($s).hash
    }};
}
