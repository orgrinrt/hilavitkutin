//! Column classification: per-fiber column role (domain 15).
//!
//! Per-fiber column classification determines the codegen shape:
//! internal columns live register-to-register, input columns come
//! from the preceding fiber's arena, output columns spill to the
//! store-buffer-friendly tail of the dispatch function.

use arvo::strategy::Identity;
use arvo::USize;

/// How a column is used by a given fiber.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ColumnClassification {
    /// Fiber-local; register-to-register (dead-store eliminated).
    Internal,
    /// Loaded from upstream at fiber start.
    Input,
    /// Written at fiber end; flows to downstream fibers.
    Output,
}

impl Default for ColumnClassification {
    fn default() -> Self {
        Self::Internal
    }
}

/// Per-fiber column classification map.
///
/// `class[f][c]` is the classification of column `c` within fiber `f`.
/// `column_count[f]` records how many of fiber `f`'s slots are
/// populated; columns past that index are ignored.
///
/// Plan-stage output of step 11 (`classify_columns`).
#[derive(Copy, Clone, Debug)]
pub struct ColumnClassMap<const MAX_FIBERS: usize, const MAX_COLUMNS_PER_FIBER: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pub class: [[ColumnClassification; MAX_COLUMNS_PER_FIBER]; MAX_FIBERS],
    pub column_count: [USize; MAX_FIBERS],
}

impl<const MAX_FIBERS: usize, const MAX_COLUMNS_PER_FIBER: usize> // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>
{
    pub const fn new() -> Self {
        Self {
            class: [[ColumnClassification::Internal; MAX_COLUMNS_PER_FIBER]; MAX_FIBERS],
            column_count: [USize::ZERO; MAX_FIBERS],
        }
    }
}

impl<const MAX_FIBERS: usize, const MAX_COLUMNS_PER_FIBER: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>
{
    fn default() -> Self {
        Self::new()
    }
}
