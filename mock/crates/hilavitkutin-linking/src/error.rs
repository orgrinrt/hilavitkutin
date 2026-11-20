//! Structured no-heap error types.

use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::USize;

/// The type of an OS error code.
///
/// An unsigned integer of the platform's size, since both the unix `errno`
/// and the Windows `GetLastError` ranges are non-negative. The adaptation is
/// declared here because no strategy preset names one yet. The crate never
/// rounds, a host integer being on the grid, and a code outside the range
/// must not wrap into a different valid code. A code enters through
/// `from_host` and is read through `host`.
pub type OsCode = USize<Adapt<TowardZero, Saturate>>;

/// Errors produced by `Library::load` / `Library::resolve`.
///
/// All variants are fixed-size; no heap payloads. Platform error codes
/// are carried as `OsCode` for variants where the underlying OS
/// produced a numeric error that the caller might want to surface.
#[non_exhaustive]
pub enum LinkError {
    /// The supplied path did not resolve to a readable library.
    PathNotFound,

    /// The OS loader rejected the library. `platform_code` carries
    /// the platform's numeric error code at the failure site.
    ///
    /// On Windows this is `GetLastError()` and is always populated.
    /// On Unix this is the thread-local `errno` read via the per-libc
    /// helper (`__errno_location` on Linux/Android, `__error` on
    /// Darwin and BSD). POSIX does not require `dlopen` to set
    /// `errno`, so the value is best-effort: in practice every libc
    /// this crate targets propagates the underlying syscall failure
    /// (ENOENT, EACCES, ENOMEM, etc.) into `errno`, but a future libc
    /// is free to leave it untouched. On unsupported unixes the value
    /// is `0`.
    ///
    /// Unsigned because both forms are non-negative on the platforms
    /// we target.
    LoadFailed {
        platform_code: OsCode,
    },

    /// Symbol lookup returned null; the library did not export the
    /// requested name.
    SymbolMissing,

    /// The artefact was loadable but targets an incompatible platform
    /// (reserved for future use when the loader inspects headers).
    PlatformMismatch,

    /// A compatibility check ahead of load rejected the artefact.
    IncompatibleVersion,

    /// The supplied path could not be converted to the platform's
    /// native encoding.
    ///
    /// On Windows, this fires when the path contains non-ASCII bytes
    /// (v1 lacks a UTF-8 to UTF-16 transcoder) or exceeds the classic
    /// `MAX_PATH` (260 wchars). Unix has no analogue: the OS loader
    /// treats the path as opaque bytes and never returns this
    /// variant. Distinct from `PathNotFound` so consumers can
    /// distinguish "the loader cannot speak this path" from "no file
    /// at this path".
    PathEncodingUnsupported,
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
