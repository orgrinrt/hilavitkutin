//! Column classification: per-fiber column role (domain 15).
//!
//! Per-fiber column classification determines the codegen shape:
//! internal columns live register-to-register, input columns come
//! from the preceding fiber's arena, output columns spill to the
//! store-buffer-friendly tail of the dispatch function.

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::Capacity;

use crate::plan::dims::PlanDims;

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
/// populated; columns past that index are ignored. Sized by the fiber
/// and column-per-fiber capacities projected from one `D: PlanDims`.
///
/// Plan-stage output of step 11 (`classify_columns`).
pub struct ColumnClassMap<D: PlanDims> {
    pub class: <D::Fibers as Capacity>::Array<
        <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>,
    >,
    pub column_count: <D::Fibers as Capacity>::Array<USize>,
}

impl<D: PlanDims> ColumnClassMap<D>
where
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
{
    pub fn new() -> Self {
        Self {
            class: <D::Fibers as Capacity>::filled(
                <D::ColumnsPerFiber as Capacity>::filled(ColumnClassification::Internal),
            ),
            column_count: <D::Fibers as Capacity>::filled(USize::ZERO),
        }
    }
}

impl<D: PlanDims> Copy for ColumnClassMap<D>
where
    <D::Fibers as Capacity>::Array<<D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>>:
        Copy,
    <D::Fibers as Capacity>::Array<USize>: Copy,
{
}

impl<D: PlanDims> Clone for ColumnClassMap<D>
where
    <D::Fibers as Capacity>::Array<<D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>>:
        Copy,
    <D::Fibers as Capacity>::Array<USize>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for ColumnClassMap<D>
where
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> core::fmt::Debug for ColumnClassMap<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ColumnClassMap").finish_non_exhaustive()
    }
}
