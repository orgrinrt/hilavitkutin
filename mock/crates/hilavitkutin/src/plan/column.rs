//! Column classification: per-fiber column role (domain 15).
//!
//! Per-fiber column classification determines the codegen shape:
//! internal columns live register-to-register, input columns come
//! from the preceding fiber's arena, output columns spill to the
//! store-buffer-friendly tail of the dispatch function.

use arvo::strategy::Identity;
use arvo::{Cap, USize};
use arvo_tensor::cap_size;

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
pub struct ColumnClassMap<const MAX_FIBERS: Cap, const MAX_COLUMNS_PER_FIBER: Cap>
where
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    pub class: [[ColumnClassification; cap_size(MAX_COLUMNS_PER_FIBER)]; cap_size(MAX_FIBERS)],
    pub column_count: [USize; cap_size(MAX_FIBERS)],
}

impl<const MAX_FIBERS: Cap, const MAX_COLUMNS_PER_FIBER: Cap>
    ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    pub const fn new() -> Self {
        Self {
            class: [[ColumnClassification::Internal; cap_size(MAX_COLUMNS_PER_FIBER)];
                cap_size(MAX_FIBERS)],
            column_count: [USize::ZERO; cap_size(MAX_FIBERS)],
        }
    }
}

impl<const MAX_FIBERS: Cap, const MAX_COLUMNS_PER_FIBER: Cap> Default
    for ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    fn default() -> Self {
        Self::new()
    }
}
