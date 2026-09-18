//! `EngineCtx` construction: the crate-internal `from_projected` builder
//! and the public `project` constructor.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change. `EngineCtx`'s fields stay private (unchanged): this module is a
//! descendant of `dispatch::engine_ctx`, where the struct is defined, so it
//! reaches them without any visibility widening.

use core::marker::PhantomData;

use hilavitkutin_api::access::AccessSet;

use super::col_accum_project::ColProject;
use super::projection::Project;
use super::selectors::BuildMetaPtr;
use super::{AccumProject, EngineCtx, VirtualProject};
use crate::dispatch::morsel::MorselRange;
use crate::meta::MetaBlock;

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    /// Construct a Context from pre-built projected bundles.
    ///
    /// Crate-internal only. The bundle types are caller-chosen here, so
    /// this constructor does not by itself prove that the bundles are the
    /// projection of `R` / `W`. The public `project` constructor derives
    /// them from the access sets and is the only way an external caller
    /// builds a Context; this internal entry exists so `project` (and the
    /// run-loop) can assemble the value once the tie is established. Never
    /// make this `pub`: a `pub` bundle-taking constructor would let a
    /// caller pair a non-empty access set with a mismatched bundle,
    /// satisfying the `Contains` proof while resolving through an
    /// unrelated bundle and hitting the nil base-case panic.
    #[inline]
    pub(crate) fn from_projected(
        reads: RBundle,
        read_cols: RCols,
        write_cols: WCols,
        write_accums: WAccum,
        write_virtuals: WVirt,
        meta_ptr: MP,
        epoch: arvo::USize,
        morsel: MorselRange,
    ) -> Self {
        Self {
            reads,
            read_cols,
            write_cols,
            write_accums,
            write_virtuals,
            meta_ptr,
            epoch,
            morsel,
            _frame: PhantomData,
            _sets: PhantomData,
        }
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    /// Project a Context from the scheduler bindings and a per-frame column
    /// source.
    ///
    /// This is the public constructor. The projected bundles are not
    /// caller-chosen: `RBundle` is forced to be the resource projection of
    /// `R` over the `bindings`; `RCols` is the column projection of `R` and
    /// `WCols` the column projection of `W`, both over the `cols` source. A
    /// caller therefore cannot pair a non-empty access set with an empty or
    /// mismatched source; the `Project` / `ColProject` bounds are
    /// unsatisfiable when the source lacks a declared store, so the
    /// construction fails at compile time rather than panicking at the nil
    /// base case during a later accessor call.
    ///
    /// Columns are projected per side, not over `R union W`: the read
    /// accessor resolves over `RCols` (the columns in `R`), the write
    /// accessor over `WCols` (the columns in `W`). A column that is both
    /// read and written appears once in each bundle, both pulling the same
    /// pointer from the `cols` source. Projecting per side keeps each
    /// bundle free of duplicate column types, so the type-keyed index
    /// witness resolves uniquely; a single `R union W` bundle would list a
    /// read-write column twice and make the inferred index ambiguous.
    ///
    /// The projection tie is enforced by the type system. A Read set
    /// containing `Resource<u32>` cannot be projected from an empty
    /// source: the `Project` bound is unsatisfiable, so the construction
    /// is rejected at compile time rather than reaching the nil
    /// base-case panic during a later accessor call.
    ///
    /// The write-set accumulator bundle `WAccum` is projected from the
    /// `bindings` source, not `cols`: an accumulator handle retains a
    /// `'frame` borrow of the binding's live-length cell, so it ties to the
    /// `'frame`-lived bindings, while the column source can stay shorter. The
    /// `AccumProject<'frame, W, WAIdx, Out = WAccum>` bound forces `WAccum` to
    /// be the real projection of `W`, so the Context's accumulator parameter
    /// cannot be mismatched.
    ///
    /// ```compile_fail
    /// use hilavitkutin::dispatch::engine_ctx::{EngineCtx, SnapNil, ColPtrNil};
    /// use hilavitkutin::meta::MetaBlock;
    /// use hilavitkutin::dispatch::morsel::MorselRange;
    /// use hilavitkutin_api::access::{Cons, Empty};
    /// use hilavitkutin_api::store::Resource;
    /// use arvo::USize;
    /// use arvo::strategy::{Additive, Identity};
    ///
    /// type ReadU32 = Cons<Resource<u32>, Empty>;
    ///
    /// // The Read set declares `Resource<u32>`, but the resource source
    /// // is empty (`SnapNil`). `SnapNil: Project<ReadU32, _>` does not
    /// // hold (no `Selector<u32, _>` on `SnapNil`), so this does not
    /// // compile.
    /// let _ctx: EngineCtx<'_, ReadU32, Empty, _, _, _> =
    ///     EngineCtx::project(&SnapNil, &ColPtrNil, &MetaBlock::default(), <USize as Identity<Additive>>::IDENTITY, MorselRange::new(<USize as Identity<Additive>>::IDENTITY, <USize as Identity<Additive>>::IDENTITY));
    /// ```
    #[inline]
    pub fn project<A, C, RIdx, RCIdx, WCIdx, WAIdx, WVIdx>(
        bindings: &'frame A,
        cols: &C,
        meta_block: &'frame MetaBlock,
        epoch: arvo::USize,
        morsel: MorselRange,
    ) -> EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
    where
        A: Project<R, RIdx, Out = RBundle>,
        C: ColProject<R, RCIdx, Out = RCols>,
        C: ColProject<W, WCIdx, Out = WCols>,
        A: AccumProject<'frame, W, WAIdx, Out = WAccum>,
        A: VirtualProject<'frame, W, WVIdx, Out = WVirt>,
        MP: BuildMetaPtr<'frame>,
    {
        let reads = <A as Project<R, RIdx>>::project(bindings);
        let read_cols = <C as ColProject<R, RCIdx>>::col_project(cols);
        let write_cols = <C as ColProject<W, WCIdx>>::col_project(cols);
        // The accumulator bundle projects from the `'frame` bindings (it
        // retains a borrow of each live-length cell), not the column source.
        let write_accums = <A as AccumProject<'frame, W, WAIdx>>::acc_project(bindings);
        // The write-virtual bundle also projects from the `'frame` bindings: each
        // entry is a borrow of a `VirtualBinding<T>` stamp cell. The same cells
        // the trunk-gate reads, so a fire here is observed by an `On<T>` gate.
        let write_virtuals = <A as VirtualProject<'frame, W, WVIdx>>::virt_project(bindings);
        // E4 slice 3: build the per-unit meta pointer from the engine-owned
        // block. `MetaNil` ignores it (consumer units); `MetaRef` captures it
        // (`OnMeta` units), gaining the gated `meta::<T>()` accessor.
        let meta_ptr = <MP as BuildMetaPtr<'frame>>::build(meta_block);
        EngineCtx::from_projected(
            reads,
            read_cols,
            write_cols,
            write_accums,
            write_virtuals,
            meta_ptr,
            epoch,
            morsel,
        )
    }
}
