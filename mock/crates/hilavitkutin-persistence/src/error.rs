//! Persistence error type.
//!
//! Coarse-grained failure modes shared across the crate. Each variant
//! names a class of failure the consumer can react to without pulling
//! in format-specific detail.

/// Failure modes for persistence operations.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PersistenceError {
    /// I/O failure — mmap, allocate, or protect did not succeed.
    Io,
    /// rkyv archive format violation (when real impls land).
    Archive,
    /// Internal consistency check failed.
    Invariant,
    /// Expected resource not present.
    Missing,
}
