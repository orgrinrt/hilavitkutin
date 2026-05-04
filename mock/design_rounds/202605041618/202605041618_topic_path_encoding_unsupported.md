**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-linking (LinkError enum + windows.rs ASCII conversion path + DESIGN paragraph)
**Source topics:** task #348 (PLUGIN-HOST-D3, plugin-host audit F6)

# Topic: PathEncodingUnsupported variant for Windows non-ASCII paths

## Background

The Windows backend in `hilavitkutin-linking` currently rejects non-ASCII bytes and oversized paths in `ascii_to_wide` and the caller maps the failure to `LinkError::PathNotFound`:

```rust
// src/backend/windows.rs:18-21
let Some(wide) = ascii_to_wide(path) else {
    return Outcome::Err(LinkError::PathNotFound);
};
```

```rust
// src/backend/windows.rs:61-72
fn ascii_to_wide(bytes: &[u8]) -> Option<[u16; MAX_PATH_WIDE]> {
    if !is_null_terminated(bytes) || bytes.len() > MAX_PATH_WIDE {
        return None;
    }
    let mut wide = [0u16; MAX_PATH_WIDE];
    for (i, &b) in bytes.iter().enumerate() {
        if b > 0x7F {
            return None; // non-ASCII; v1 rejects
        }
        wide[i] = b as u16;
    }
    Some(wide)
}
```

The `None` arm collapses three distinct failures into one error: "missing trailing NUL", "path too long for `MAX_PATH_WIDE` (260)", and "non-ASCII byte present". All three surface to the consumer as `PathNotFound`. The plugin-host audit (2026-05-04) called this out as F6: a consumer that hands the loader a UTF-8-encoded path containing any non-ASCII byte gets back `PathNotFound`, which sends debugging in the wrong direction (look for the file, not the encoding).

This round adds a new `LinkError::PathEncodingUnsupported` variant and routes the Windows-side ASCII rejection through it. Unix is unaffected (paths there are bytes; the OS loader interprets them).

## Out of scope

The audit's longer-term recommendation — actual UTF-8 → UTF-16 conversion plus `\\?\` LongPath prefix to support non-ASCII and oversized paths — is deliberately out of scope for D3. The task description marks it "future scope (separate task)". This round ships only the diagnostic precision, not the encoding support.

The reason: a no-std no-alloc UTF-8 → UTF-16 transcoder has design questions (output buffer sizing, surrogate-pair handling, LongPath prefix injection at the right cfg) that need their own round. Diagnostic precision is independent and ships immediately.

## Proposed shape

`mock/crates/hilavitkutin-linking/src/error.rs`:

```rust
#[non_exhaustive]
pub enum LinkError {
    PathNotFound,
    LoadFailed { platform_code: USize },
    SymbolMissing,
    PlatformMismatch,
    IncompatibleVersion,

    /// The supplied path could not be converted to the platform's
    /// native encoding. On Windows, this fires when the path
    /// contains non-ASCII bytes (v1 lacks a UTF-8 to UTF-16
    /// transcoder) or exceeds the `MAX_PATH` (260 wchars) the
    /// classic Windows API accepts. Unix has no analogue: the OS
    /// loader treats the path as opaque bytes and never returns
    /// this variant.
    PathEncodingUnsupported,
}
```

`mock/crates/hilavitkutin-linking/src/backend/windows.rs` rework: split `ascii_to_wide` into two outcomes so the caller can distinguish "unterminated / too long / non-ASCII" from "OS loader failure". The simplest shape keeps the helper signature stable and changes only the call-site map:

```rust
let Some(wide) = ascii_to_wide(path) else {
    return Outcome::Err(LinkError::PathEncodingUnsupported);
};
```

That's the minimal change. Every `None` from `ascii_to_wide` is by construction an encoding-or-length issue; `PathNotFound` would only ever fire if `ascii_to_wide` returned `Some` and `LoadLibraryW` returned NULL with a `GetLastError()` of `ERROR_FILE_NOT_FOUND`, but the existing code already returns `LoadFailed { platform_code }` for that case, so `PathNotFound` was always unreachable from the Windows backend's load path. We do not change the Unix backend; its `is_null_terminated` check still maps to `PathNotFound` because Unix paths are opaque bytes and a missing NUL terminator is genuinely "we cannot find a file at this path".

## What stays untouched

- `LinkError::PathNotFound` variant signature. Still returned from Unix path validation.
- `ascii_to_wide` signature and body. Only the caller's error-mapping changes.
- `LoadLibraryW` failure path. Still maps to `LoadFailed { platform_code }`.
- `IncompatibilityError` enum. No changes.
- No new public API beyond the new variant.

## Decision

Adopt as proposed. One round, one crate, additive enum variant + one call-site map change. No ABI break (the enum is `#[non_exhaustive]`, so the new variant is forward-compatible per Rust semver convention).
