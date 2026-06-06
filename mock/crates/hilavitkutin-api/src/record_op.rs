//! Opt-in per-record transform contract for within-fiber linear fusion.
//!
//! A `WorkUnit` that is a pure per-record map (reads one column, writes one
//! column, value to value, no cross-record state) may additionally implement
//! `RecordOp` to declare that map. The engine folds a linear read-after-write
//! chain of such maps into one fused unit whose body keeps every intermediate
//! in a register (dead-store elimination removes the intermediate column
//! traffic), matching a hand-fused loop. The fold is a compile-time composition
//! of concrete monomorphised calls, no function pointers and no dynamic
//! dispatch.
//!
//! `RecordOp` is orthogonal to `WorkUnit::execute`: implementing it changes
//! nothing about the execute contract, and a simple `WorkUnit` author never
//! implements it. A unit that carries cross-record state (an accumulator) does
//! not implement `RecordOp` and stays on the ordinary per-unit dispatch path.
//!
//! The associated `In` / `Out` types are `ColumnValue` because they are exactly
//! the unit's read and write column types; the fused unit reads the chain's
//! input column and writes its output column, with the links held in registers.

use crate::column_value::ColumnValue;

/// A pure per-record map a `WorkUnit` opts into so the engine can fuse a linear
/// chain of such maps into one register-passing loop body.
///
/// `apply` is the per-record transform: it must be a pure function of its input
/// (the engine composes it with sibling maps and relies on it having no
/// cross-record or observable side effect). `In` is the unit's read column type,
/// `Out` its write column type.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not declare a per-record map for fusion",
    note = "Implement `RecordOp` on a WorkUnit that is a pure per-record map (one column in, one column out, no cross-record state) to let the engine fuse a linear chain of such units. Declare `type In`, `type Out`, and `fn apply(&self, x: In) -> Out`. A unit with cross-record state (an accumulator) does not implement `RecordOp`."
)]
pub trait RecordOp {
    /// The record value this map consumes (the unit's read column type).
    type In: ColumnValue;
    /// The record value this map produces (the unit's write column type).
    type Out: ColumnValue;

    /// Transform one record value. Must be a pure function of `x`.
    fn apply(&self, x: Self::In) -> Self::Out;
}
