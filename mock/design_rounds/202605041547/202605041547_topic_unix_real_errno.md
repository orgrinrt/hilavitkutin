**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-linking (1 backend file, internal helper only)
**Source topics:** task #347 (PLUGIN-HOST-D2, plugin-host audit F5)

# Topic: surface real errno on Unix dlopen failure

## Background

The Unix backend in `hilavitkutin-linking` returns `LinkError::LoadFailed { platform_code: USize }` when `dlopen` or `dlclose` fails, but the helper that produces `platform_code` is stubbed:

```rust
// src/backend/unix.rs:65-73
fn read_errno() -> USize {
    // libc exposes errno via thread-local; extracting it portably
    // without std means calling through __errno_location / __error
    // per-platform. For v1 we return a sentinel 0. The variant
    // conveys the error category even when the numeric code is not
    // captured. Follow-up round refines this if callers need the
    // exact errno value.
    USize(0)
}
```

The Windows backend already does the right thing via `GetLastError()`. The Unix sentinel was a v1 placeholder explicitly flagged for follow-up. The plugin-host audit (2026-05-04) called this out as F5: "Real errno on Unix dlopen failure" — consumers cannot distinguish ENOENT vs EACCES vs ELIBBAD when `dlopen` fails, which is a coarseness an extension host wants to surface at diagnostic time.

This round refines the helper. The `LinkError::LoadFailed { platform_code: USize }` variant signature is unchanged.

## errno is "best effort", not POSIX-mandated

POSIX does not require `dlopen` to set `errno`. In practice, every libc this crate targets does set it, because the underlying syscalls (`open(2)`, `mmap(2)`, etc.) propagate their own errno into the thread-local during the failed dlopen. So:

- Linux glibc: errno set to the underlying syscall failure (ENOENT, EACCES, ENOMEM, EAGAIN, etc.).
- musl: same behaviour.
- macOS: errno set similarly via the Mach-O loader's syscall path.
- *BSD: errno set similarly.

This is not POSIX-guaranteed. A future libc could legitimately leave errno untouched on dlopen failure. The doc on the variant must say "best-effort errno; reflects the most recent syscall failure during the load and may be unset on platforms where the loader does not propagate it."

The POSIX-canonical channel is `dlerror()`, which returns a string. Using `dlerror()` here would mean either copying the string into a fixed-size buffer (changes the variant ABI, growth out of scope for this round) or carrying a `*const c_char` whose lifetime is undefined after subsequent dl* calls (unsafe). The numeric errno path is the right scope for D2.

## Per-platform helper symbols

Different libcs expose the thread-local errno through different functions. The cfg matrix:

| Platform | Symbol | Signature |
|---|---|---|
| Linux (glibc, musl, Bionic) | `__errno_location` | `extern "C" fn() -> *mut c_int` |
| macOS, iOS, watchOS, tvOS | `__error` | `extern "C" fn() -> *mut c_int` |
| FreeBSD, OpenBSD, NetBSD, DragonFly | `__error` | `extern "C" fn() -> *mut c_int` |
| Other unixes (Solaris, Haiku, etc.) | varies (`___errno`, `_errnop`, ...) | varies |

The first three rows cover the build targets the workspace cares about. For "other unixes", keeping the `USize(0)` sentinel is the right behaviour: returning a wrong value would be worse than returning a known-sentinel. Document this fallback in the helper.

## Cast: c_int to USize

`errno` is a `c_int` (typically `i32`). When set, it is always positive (POSIX requires errno values to be positive integers; `0` means "not set"). Cast through `as usize`:

```rust
let val: c_int = unsafe { *__errno_location() };
USize(val as usize)
```

For non-negative `c_int` values the cast is lossless on every supported platform. Documented in source as a safety/cast comment.

## Proposed shape

`mock/crates/hilavitkutin-linking/src/backend/unix.rs`:

```rust
fn read_errno() -> USize {
    // Per-libc symbol that returns the address of the thread-local
    // `errno` integer.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    unsafe extern "C" {
        fn __errno_location() -> *mut c_int;
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
    ))]
    unsafe extern "C" {
        fn __error() -> *mut c_int;
    }

    // SAFETY: each per-platform symbol returns a thread-local pointer
    // that is always valid for the lifetime of the calling thread, per
    // libc contract. The dereference reads a single `c_int` value.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let val: c_int = unsafe { *__errno_location() };
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
    ))]
    let val: c_int = unsafe { *__error() };
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
    )))]
    let val: c_int = 0;

    // errno values are non-negative when set; the cast is lossless on
    // every supported platform.
    USize(val as usize)
}
```

Plus an updated doc comment on `LinkError::LoadFailed::platform_code` capturing the "best-effort" semantics.

## What stays untouched

- `LinkError::LoadFailed { platform_code: USize }` variant signature unchanged.
- Windows backend untouched (already correct via `GetLastError`).
- No new public API.
- `is_null_terminated`, `platform_load`, `platform_resolve`, `platform_close` bodies unchanged.

## Open questions

None. The audit settled the approach; per-platform symbols are well-known and consistent.

## Decision

Adopt the per-platform helper as proposed. Single round, single file, no ABI change.
