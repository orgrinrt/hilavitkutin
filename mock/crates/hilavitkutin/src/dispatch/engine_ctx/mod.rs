//! Per-WorkUnit projected Context (B3).
//!
//! `EngineCtx<'frame, R, W>` is the value a WU's `execute` body
//! touches. It carries only the projected pointers for the stores the
//! WU declares in `R` (read) and `W` (write), enforcing the access
//! scope physically: a WU cannot reach an undeclared store because its
//! Context does not hold that pointer.
//!
//! The projection reuses a frunk-style index witness. A `Selector<T,
//! Index>` resolves a type-keyed lookup over a heterogeneous cons-list
//! without specialization: the two impls (head-match and tail-recurse)
//! are keyed on distinct `Index` types (`Here` / `There<I>`), so they
//! never overlap, and the index infers at the call site. `Project<R,
//! Indices>` carries a parallel `Indices` cons-list so each element
//! index is a trait type parameter (constrained by "this trait is
//! implemented"), dodging E0207; the free `project_reads::<R, _, _>`
//! helper pins `R` by turbofish while inference fills the index list.
//!
//! Resources project out of the scheduler bindings (`ResourceBinding`,
//! real storage from B2a). Columns project out of a per-frame column
//! pointer bundle passed in at construction, because the B2a bindings
//! column nodes are dangling placeholders (column buffers are sized by
//! the per-run record count and belong to the run-loop / plan phase).
//!
//! Accessors take `&self`, never `&mut self`, so LLVM does not reorder
//! writes across fused WUs. The unsafe read / write aliasing obligation
//! is the scheduler's: plan-time DAG analysis proves no concurrent
//! write-overlap, and WU bodies do not re-check.
//!
//! Round (file-size lint): split into a module directory. `EngineCtx`
//! stays defined directly in this file (like the scheduler's own struct
//! in `scheduler/mod.rs`): every sibling submodule is a descendant of
//! `dispatch::engine_ctx`, so each reaches its private fields with zero
//! visibility widening. Two carrier types defined in `lists` needed
//! `pub(super)` fields to cross into a different sibling that builds or
//! reads them (`AccumColPtr`'s `base`/`len`/`cap`, and `MetaRef`'s inner
//! field) — see each type's doc comment for which sibling needs it.

use core::marker::PhantomData;

use hilavitkutin_api::access::AccessSet;

use crate::dispatch::morsel::MorselRange;

mod bundles;
mod col_accum_project;
mod ctx_construct;
mod ctx_loops_has;
mod ctx_meta_resource;
mod ctx_writers;
mod lists;
mod projection;
mod selectors;

pub use bundles::{AccumBundleOf, ColBundleOf, CtxFor, ResourceBundleOf, VirtBundleOf};
pub use col_accum_project::{AccumProject, ColProject};
pub use lists::{
    AccPtrCons,
    AccPtrNil,
    AccumColPtr,
    ColPtrCons,
    ColPtrNil,
    MetaNil,
    MetaRef,
    SnapCons,
    SnapNil,
    VirtCons,
    VirtNil,
};
pub use projection::{Project, VirtualProject, project_reads};
pub use selectors::{
    AccumSelector,
    BuildMetaPtr,
    ColSelector,
    GateWith,
    MetaPtrFor,
    Selector,
    SnapSelector,
    VirtualFire,
    VirtualStampSelector,
};

// ---------------------------------------------------------------------
// Index witnesses.
//
// `Here` and `There<I>` are the disjoint index types that key the two
// `Selector` impls. Because they are distinct concrete types, the
// head-match impl (`Here`) and the tail-recurse impl (`There<I>`) never
// overlap, so the lookup compiles without specialization.
// ---------------------------------------------------------------------

/// Index witness: the matching node is the head of the list.
pub struct Here;

/// Index witness: the matching node is `I` steps into the tail.
pub struct There<I>(PhantomData<I>);

// ---------------------------------------------------------------------
// EngineCtx: the per-WU projected Context.
// ---------------------------------------------------------------------

/// Per-WorkUnit projected Context.
///
/// Holds only the projected resource and column pointers for the
/// stores the WU declares, plus the morsel range it iterates. `'frame`
/// ties the borrowed pointers to the scheduler-owned storage that lives
/// for the dispatch frame. `R` is the WU's read set, `W` its write set.
///
/// The Context is its own provider for every accessor: the eight `HasX`
/// traits resolve `type Provider = Self`.
///
/// `WAccum` (the projected write-set accumulator bundle) defaults to
/// `AccPtrNil`, the empty accumulator bundle. Accumulators are an opt-in, so
/// the default keeps an accum-free WU's `Ctx` declaration at the six prior
/// bundle params; an accum-bearing WU spells the seventh explicitly. The
/// default never masks a mismatch: `project` and `RunFiber` force `WAccum`
/// to the real projection of `W`, so a WU that declares an accumulator but
/// omits the bundle fails to compile at the projection tie.
pub struct EngineCtx<
    'frame,
    R: AccessSet,
    W: AccessSet,
    RBundle,
    RCols,
    WCols,
    WAccum = AccPtrNil,
    WVirt = VirtNil,
    MP = MetaNil,
> {
    reads:          RBundle,
    read_cols:      RCols,
    write_cols:     WCols,
    write_accums:   WAccum,
    // E4 slice 1: the projected write-virtual bundle (one `&'frame Cell<USize>`
    // stamp ref per `Virtual<T>` in `W`), and the current pass epoch. `fire<V>`
    // sets the resolved stamp to `epoch`; an `On<V>` consumer's gate (trunk_gate)
    // reads the SAME cell from the full bindings and compares to its epoch.
    // Defaults `WVirt = VirtNil`: a WU writing no virtual carries an empty bundle.
    write_virtuals: WVirt,
    // E4 slice 3: the per-unit meta pointer. `MetaNil` for consumer units (no
    // meta reference); `MetaRef<'frame>` for `OnMeta` units (a borrow of the
    // engine-owned `MetaBlock`). The `meta::<T>()` accessor exists only on a Ctx
    // whose `MP = MetaRef<'frame>`. Defaults `MP = MetaNil`.
    meta_ptr:       MP,
    epoch:          arvo::USize,
    morsel:         MorselRange,
    _frame:         PhantomData<&'frame ()>,
    _sets:          PhantomData<(R, W)>,
}
