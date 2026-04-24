//! Structured no-heap error types.

use arvo::USize;

/// Errors produced by `Extension::load` / `Extension::resolve`.
///
/// All variants are fixed-size; no heap payloads. Platform error codes
/// are carried as `arvo::ISize` for variants where the underlying OS
/// produced a numeric error that the caller might want to surface.
#[non_exhaustive]
pub enum ExtensionError {
    /// The supplied path did not resolve to a readable library.
    PathNotFound,

    /// The OS loader rejected the library. `platform_code` carries
    /// `errno` on unix or `GetLastError()` on Windows. Unsigned
    /// because both forms are non-negative on the platforms we
    /// target.
    LoadFailed { platform_code: USize },

    /// Symbol lookup returned null; the library did not export the
    /// requested name.
    SymbolMissing,

    /// The artefact was loadable but targets an incompatible platform
    /// (reserved for future use when the loader inspects headers).
    PlatformMismatch,

    /// A compatibility check ahead of load rejected the artefact.
    IncompatibleVersion,
}

/// Errors produced by the optional `compatibility_check` helper.
///
/// Narrow by design: the loader performs no manifest parsing. If more
/// detailed incompatibility classification is needed, layer it above
/// this crate.
#[non_exhaustive]
pub enum IncompatibilityError {
    VersionSkew,
    ArchMismatch,
    AbiMismatch,
}
