**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-linking tests/basic.rs (one match arm extension)
**Source topics:** PR #52 reviewer F1 (round 202605041618 follow-up)

# Topic: extend reject_path_without_null_terminator to accept PathEncodingUnsupported

## Background

Round 202605041618 added `LinkError::PathEncodingUnsupported` and rewired the Windows backend's `ascii_to_wide` rejection arm (which checks `is_null_terminated` first) from `PathNotFound` to the new variant. The pr-reviewer-senior pass on PR #52 found that `tests/basic.rs:128-134`'s `reject_path_without_null_terminator` only accepts `PathNotFound`:

```rust
#[test]
fn reject_path_without_null_terminator() {
    match Library::load(b"/some/path") {
        Outcome::Ok(_) => panic!("un-terminated path should not load"),
        Outcome::Err(LinkError::PathNotFound) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}
```

On Unix the un-terminated path hits the explicit `is_null_terminated` check in `unix.rs:19` and produces `PathNotFound`. On Windows the same input flows through `ascii_to_wide`, which calls `is_null_terminated` first and returns `None`, which the caller now maps to `PathEncodingUnsupported` (post-202605041618). The test would panic on Windows at the `_` arm. The macOS host CI does not exercise the Windows backend, which is why the original round's test plan reported green.

The intent of the test is "un-terminated paths are rejected with a path-shape error, not a load error". Both `PathNotFound` (Unix) and `PathEncodingUnsupported` (Windows) are correct under that intent. Add the second variant to the accepted set.

## Proposed shape

```rust
#[test]
fn reject_path_without_null_terminator() {
    match Library::load(b"/some/path") {
        Outcome::Ok(_) => panic!("un-terminated path should not load"),
        Outcome::Err(LinkError::PathNotFound) => {}
        Outcome::Err(LinkError::PathEncodingUnsupported) => {}
        Outcome::Err(_) => panic!("wrong error variant"),
    }
}
```

The two arms reflect the platform-conditional shape: Unix returns the first; Windows returns the second; the test passes on both targets.

## Decision

Adopt as proposed. One-arm test extension. No source change. Test file only.
