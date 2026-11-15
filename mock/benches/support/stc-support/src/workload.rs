//! The workloads: which kernel, which shape, and why each is realistic.
//!
//! A workload is a row here and the arms are generic over it, so adding one
//! is a row plus a kernel type and no arm changes.

use crate::kernel::{Heavy, Kernel, Light, Mix};

/// What the fiber produces. A fixed vocabulary of the bench: a commutative
/// fold, where any record sequence gives the same result, or an
/// order-sensitive append, where the output sequence is the result.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    Fold,
    Append,
}

/// One workload row.
#[derive(Copy, Clone, Debug)]
pub struct WorkloadRow {
    /// The id carried in [`crate::ConvInput::kind`].
    pub id:     u32,
    /// The name bench sections use.
    pub name:   &'static str,
    pub shape:  Shape,
    /// Why the workload stands for real work, quoted in the findings.
    pub reason: &'static str,
}

pub const FOLD_LIGHT: u32 = 0;
pub const FOLD_HEAVY: u32 = 1;
pub const APPEND: u32 = 2;

/// Every workload the bench runs. The short-fiber sections run `fold_light`
/// at record counts of one to four morsels; they are a size band of that
/// row, not a row of their own.
pub const WORKLOADS: [WorkloadRow; 3] = [
    WorkloadRow {
        id:     FOLD_LIGHT,
        name:   "fold_light",
        shape:  Shape::Fold,
        reason: "a sum over one column (a total, a count of live records), the most common \
                 reduction, which turns memory-bound once the column leaves cache",
    },
    WorkloadRow {
        id:     FOLD_HEAVY,
        name:   "fold_heavy",
        shape:  Shape::Fold,
        reason: "a commutative fold whose per-record kernel costs tens of nanoseconds (a score \
                 or a hash per record), so compute dominates memory traffic",
    },
    WorkloadRow {
        id:     APPEND,
        name:   "append",
        shape:  Shape::Append,
        reason: "a transform appended to an accumulator (an event log, an ordered output \
                 buffer), where the output sequence is part of the result",
    },
];

/// The row for an id, or `None` for an id no row carries.
pub fn row(id: u32) -> Option<&'static WorkloadRow> {
    WORKLOADS.iter().find(|w| w.id == id)
}

/// The row for a section name, or `None` for a name no row carries.
pub fn by_name(name: &str) -> Option<&'static WorkloadRow> {
    WORKLOADS.iter().find(|w| w.name == name)
}

/// A visitor over the workload's kernel type, so an arm resolves the id to a
/// monomorphised path once, outside the timed region.
pub trait WorkloadVisitor {
    type Out;
    fn fold<K: Kernel>(self) -> Self::Out;
    fn append<K: Kernel>(self) -> Self::Out;
}

/// Resolve `id` to its kernel and shape and call the visitor on them.
///
/// # Panics
///
/// On an id no row carries: the orchestrator builds every input from a row,
/// so an unknown id is a broken bench section, named in the message.
pub fn visit<V: WorkloadVisitor>(id: u32, v: V) -> V::Out {
    match id {
        FOLD_LIGHT => v.fold::<Light>(),
        FOLD_HEAVY => v.fold::<Heavy>(),
        APPEND => v.append::<Mix>(),
        other => panic!("workload id {other} names no row of WORKLOADS"),
    }
}
