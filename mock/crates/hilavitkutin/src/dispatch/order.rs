//! Const topological dispatch order from access masks (domain 17 keystone).
//!
//! The dispatch order must be a compile-time fact for LLVM to devirtualise the
//! per-fiber walk (proven: sketch `202606071000_const-driven-carrier`, a const
//! order dispatched through a local fn-pointer array objdumps to zero `blr`; a
//! runtime permutation does not devirtualise). This module computes that order
//! at compile time from the work units' access masks.
//!
//! `topo_order` is the Kahn topological sort over the access matrix, all `const`
//! so the result is a `const ORDER` the dispatch site walks. The per-unit
//! `AccessMask`s feed in from the carrier via the const mask fold (built on the
//! shipped `MaskProject` / `WitnessIndex` / `Locate` machinery in
//! `plan::project`); the order assembly over the carrier type is proven in
//! sketch `202606071130_const-order-assoc`. This first slice lands the const
//! topological sort itself; the carrier fold and the `Scheduler::run` rewire
//! that dispatches in this order follow in the same round.

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::Capacity;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_api::WorkUnit;

use crate::plan::access::AccessMask;
use crate::plan::project::MaskProject;

/// Compute the topological dispatch order over `N` work units from their read
/// and write access masks.
///
/// `order[k]` is the carrier index dispatched at topological step `k`. The
/// dependency edge `i -> j` (unit `i` must run before unit `j`) holds when unit
/// `i` writes a store unit `j` reads, i.e. `writes[i]` overlaps `reads[j]`
/// (`i != j`). Kahn's algorithm with a lowest-index tie-break makes the order
/// deterministic: among the units whose dependencies are all satisfied, the one
/// with the smallest carrier index runs next.
///
/// All `const`: the order is computed at compile time from the access masks
/// (themselves a `const` function of the work units' `AccessSet` types), so the
/// dispatch site can dispatch in a statically-known order, the devirtualisation
/// precondition. A cyclic input (no valid topological order) stops early,
/// leaving the unreached tail at zero; the plan stage rejects cycles before
/// dispatch, so a well-formed pipeline always fills every position.
pub const fn topo_order<
    CS: Capacity,
    const N: usize, // lint:allow(no-bare-numeric) reason: array-length const-generic root for the const-order computation (no-bare-primitives.md exception 2); const fn cannot range over Capacity::Array (trait methods are not const); tracked: #649
>(
    reads: [AccessMask<CS>; N],
    writes: [AccessMask<CS>; N],
) -> [USize; N] {
    // In-degree per unit: how many unsatisfied predecessors it has.
    let mut indeg = [0usize; N]; // lint:allow(no-bare-numeric) reason: internal in-degree counters; tracked: #72
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            if i != j && writes[i].overlaps(&reads[j]).0 {
                indeg[j] += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut order = [USize::ZERO; N];
    let mut done = [false; N];
    let mut out = 0;
    while out < N {
        // Lowest-index not-done unit with no unsatisfied predecessor.
        let mut pick = N;
        let mut k = 0;
        while k < N {
            if !done[k] && indeg[k] == 0 {
                pick = k;
                break;
            }
            k += 1;
        }
        // No ready unit: a cycle. Leave the tail zeroed; the plan rejects
        // cycles upstream, so this path is unreachable for a valid pipeline.
        if pick == N {
            break;
        }
        done[pick] = true;
        order[out] = USize(pick); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal topo cursor; tracked: #72
        out += 1;
        // Relax successors of the picked unit.
        let mut j = 0;
        while j < N {
            if j != pick && !done[j] && writes[pick].overlaps(&reads[j]).0 {
                indeg[j] -= 1;
            }
            j += 1;
        }
    }
    order
}

/// Fold a `WorkUnitBundle` cons-list's per-unit read and write access masks into
/// fixed `[AccessMask<CS>; N]` arrays, in carrier order, at compile time.
///
/// The carrier is the bundle type (`Cons<W, T>` / `Empty`, the builder's
/// prepend order). `Witnesses` is the parallel per-unit `(ReadIdx, WriteIdx)`
/// `Locate`-over-`Stores` witness cons-list, inferred at the call site exactly
/// as `BundleProject` infers it. Each unit's read and write masks come from the
/// shipped const `MaskProject` fold (`mask.set(WitnessIndex::INDEX)`); `fill`
/// writes them at the unit's carrier position. All `const`, so `carrier_order`
/// can run the whole thing in a const context.
pub const trait CarrierMasks<Stores, Witnesses, CS: Capacity> {
    /// Number of work units in the carrier.
    const LEN: usize; // lint:allow(no-bare-numeric) reason: carrier-length const-generic root (no-bare-primitives.md exception 2); tracked: #649

    /// Write each unit's read/write mask at its carrier position, from `pos`.
    fn fill(reads: &mut [AccessMask<CS>], writes: &mut [AccessMask<CS>], pos: USize);
}

impl<Stores, CS: Capacity> const CarrierMasks<Stores, Empty, CS> for Empty {
    const LEN: usize = 0; // lint:allow(no-bare-numeric) reason: empty-carrier length; tracked: #649

    #[inline]
    fn fill(_reads: &mut [AccessMask<CS>], _writes: &mut [AccessMask<CS>], _pos: USize) {}
}

impl<Stores, W, T, RI, WI, WT, CS: Capacity> const CarrierMasks<Stores, Cons<(RI, WI), WT>, CS>
    for Cons<W, T>
where
    W: WorkUnit,
    W::Read: [const] MaskProject<Stores, RI, CS>,
    W::Write: [const] MaskProject<Stores, WI, CS>,
    T: [const] CarrierMasks<Stores, WT, CS>,
{
    const LEN: usize = 1 + <T as CarrierMasks<Stores, WT, CS>>::LEN; // lint:allow(no-bare-numeric) reason: carrier-length successor; tracked: #649

    #[inline]
    fn fill(reads: &mut [AccessMask<CS>], writes: &mut [AccessMask<CS>], pos: USize) {
        let i = pos.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal array index; tracked: #121
        reads[i] = <W::Read as MaskProject<Stores, RI, CS>>::project_mask(AccessMask::empty());
        writes[i] = <W::Write as MaskProject<Stores, WI, CS>>::project_mask(AccessMask::empty());
        <T as CarrierMasks<Stores, WT, CS>>::fill(reads, writes, USize(i + 1)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: carrier-position successor; tracked: #121
    }
}

// The same fold over the value-carrying `WuCons` / `WuNil` list that the
// scheduler actually retains (its `wu_values`). `WuCons<W, Tail>` encodes the
// same W types as the `Cons<W, T>` bundle in the same order, so `Scheduler::run`
// can compute the const ORDER from its `WuVals` type without retaining the
// builder's `Wus` bundle type.
impl<Stores, CS: Capacity> const CarrierMasks<Stores, Empty, CS> for WuNil {
    const LEN: usize = 0; // lint:allow(no-bare-numeric) reason: empty-carrier length; tracked: #649

    #[inline]
    fn fill(_reads: &mut [AccessMask<CS>], _writes: &mut [AccessMask<CS>], _pos: USize) {}
}

impl<Stores, W, T, RI, WI, WT, CS: Capacity> const CarrierMasks<Stores, Cons<(RI, WI), WT>, CS>
    for WuCons<W, T>
where
    W: WorkUnit,
    W::Read: [const] MaskProject<Stores, RI, CS>,
    W::Write: [const] MaskProject<Stores, WI, CS>,
    T: [const] CarrierMasks<Stores, WT, CS>,
{
    const LEN: usize = 1 + <T as CarrierMasks<Stores, WT, CS>>::LEN; // lint:allow(no-bare-numeric) reason: carrier-length successor; tracked: #649

    #[inline]
    fn fill(reads: &mut [AccessMask<CS>], writes: &mut [AccessMask<CS>], pos: USize) {
        let i = pos.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal array index; tracked: #121
        reads[i] = <W::Read as MaskProject<Stores, RI, CS>>::project_mask(AccessMask::empty());
        writes[i] = <W::Write as MaskProject<Stores, WI, CS>>::project_mask(AccessMask::empty());
        <T as CarrierMasks<Stores, WT, CS>>::fill(reads, writes, USize(i + 1)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: carrier-position successor; tracked: #121
    }
}

/// Compute the const topological dispatch `ORDER` over the carrier bundle.
///
/// `N` is the carrier length (concrete at the call site: the engine passes
/// `D::Units`'s capacity). The bundle's per-unit access masks are folded via
/// `CarrierMasks` and the order computed by `topo_order`, all at compile time,
/// so the dispatch site dispatches in a statically-known order (proven to
/// devirtualise: sketches `202606071000` / `202606071130`).
pub const fn carrier_order<
    Bundle,
    Stores,
    Witnesses,
    CS: Capacity,
    const N: usize, // lint:allow(no-bare-numeric) reason: array-length const-generic root (no-bare-primitives.md exception 2); tracked: #649
>() -> [USize; N]
where
    Bundle: [const] CarrierMasks<Stores, Witnesses, CS>,
{
    let mut reads = [AccessMask::empty(); N];
    let mut writes = [AccessMask::empty(); N];
    <Bundle as CarrierMasks<Stores, Witnesses, CS>>::fill(&mut reads, &mut writes, USize::ZERO);
    topo_order::<CS, N>(reads, writes)
}

/// Compute the dispatch `ORDER` over `<U as Capacity>::Array<USize>`: the
/// GCE-safe form the generic `Scheduler::run` uses.
///
/// A const `[USize; N]` order (`carrier_order`) cannot be evaluated as a
/// const-generic in the generic `run` without `generic_const_exprs` overflowing
/// well-formedness; the `Capacity::Array` associated type sidesteps that (the
/// engine's array convention, per `plan::dims`). `U` sizes the order to the unit
/// capacity. The live prefix `order[0 .. live]` holds the real units in
/// topological order; capacity placeholders (empty masks) are appended after, so
/// the dispatch site walks only the live prefix (`topo_count`). Computed at
/// runtime, but LLVM const-folds it from the post-monomorphisation constant masks
/// and devirtualises the indexed dispatch (proven: sketch 202606071400, zero
/// `blr`). Empty-aware Kahn (placeholders never interleave among real units).
pub fn carrier_order_dyn<Bundle, Stores, Witnesses, U, CS>() -> <U as Capacity>::Array<USize>
where
    U: Capacity,
    CS: Capacity,
    Bundle: CarrierMasks<Stores, Witnesses, CS>,
{
    let mut reads_a = <U as Capacity>::filled(AccessMask::<CS>::empty());
    let mut writes_a = <U as Capacity>::filled(AccessMask::<CS>::empty());
    <Bundle as CarrierMasks<Stores, Witnesses, CS>>::fill(
        reads_a.as_mut(),
        writes_a.as_mut(),
        USize::ZERO,
    );
    let reads = reads_a.as_ref();
    let writes = writes_a.as_ref();
    let n = reads.len();
    // In-degree per unit (one edge i -> j iff writes[i] overlaps reads[j]).
    let mut indeg_a = <U as Capacity>::filled(USize::ZERO);
    {
        let indeg = indeg_a.as_mut();
        let mut i = 0;
        while i < n {
            let mut j = 0;
            while j < n {
                if i != j && writes[i].overlaps(&reads[j]).0 {
                    indeg[j] = USize(indeg[j].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: in-degree successor; tracked: #72
                }
                j += 1;
            }
            i += 1;
        }
    }
    let mut done_a = <U as Capacity>::filled(USize::ZERO);
    let mut order_a = <U as Capacity>::filled(USize::ZERO);
    {
        let indeg = indeg_a.as_mut();
        let done = done_a.as_mut();
        let order = order_a.as_mut();
        let mut out = 0;
        // Topological pass over the REAL units (non-empty mask). Placeholder
        // entries (both masks empty) are held out and appended below, so they
        // never interleave among real units under the lowest-index tie-break.
        loop {
            let mut pick = n;
            let mut k = 0;
            while k < n {
                let is_placeholder = reads[k].is_empty().0 && writes[k].is_empty().0;
                if done[k].0 == 0 && !is_placeholder && indeg[k].0 == 0 {
                    pick = k;
                    break;
                }
                k += 1;
            }
            if pick == n {
                break;
            }
            done[pick] = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: done flag; tracked: #72
            order[out] = USize(pick); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal topo cursor; tracked: #72
            out += 1;
            let mut j = 0;
            while j < n {
                if j != pick && done[j].0 == 0 && writes[pick].overlaps(&reads[j]).0 {
                    indeg[j] = USize(indeg[j].0 - 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: in-degree decrement; tracked: #72
                }
                j += 1;
            }
        }
        // Append remaining (placeholders + any cycle remainder) in index order.
        let mut k = 0;
        while k < n {
            if done[k].0 == 0 {
                order[out] = USize(k); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal topo cursor; tracked: #72
                out += 1;
                done[k] = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: done flag; tracked: #72
            }
            k += 1;
        }
    }
    order_a
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo_tensor::Dim;
    use crate::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
    use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
    use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
    use hilavitkutin_api::store::Column;
    use hilavitkutin_api::work_unit::{Always, WorkUnit};

    // Store-bit indices for the test pipelines.
    const S0: USize = USize::ZERO;
    const S1: USize = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store-bit index; tracked: #72
    const S2: USize = USize(2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store-bit index; tracked: #72
    const S3: USize = USize(3); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store-bit index; tracked: #72
    const S4: USize = USize(4); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store-bit index; tracked: #72

    // Store capacity for the tests: a small fixed Dim (the store-count cap).
    type CS = Dim<8>; // lint:allow(no-bare-numeric) reason: test store-capacity Dim literal; tracked: #649

    const fn mask(bit: USize) -> AccessMask<CS> {
        AccessMask::empty().set(bit)
    }

    // Anti-topological carrier (the prepend case): registered A, B, C, prepended
    // into carrier [C, B, A]. C reads S2 (B's output), B reads S1 (A's output),
    // A reads S0 writes S1. Carrier index 0 = C, 1 = B, 2 = A. A flat carrier
    // walk would run C first, reading uninitialised S2; the topo order must put
    // A (index 2) first, then B (1), then C (0).
    #[test]
    fn reorders_anti_topological_chain() {
        let reads = [mask(S2), mask(S1), mask(S0)]; // C, B, A
        let writes = [mask(S3), mask(S2), mask(S1)]; // C, B, A
        let order = topo_order::<CS, 3>(reads, writes); // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        assert_eq!(order, [USize(2), USize(1), USize(0)]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // Already-topological registration (producer first): carrier [A, B, C].
    // The order is the identity; a valid registration order is preserved.
    #[test]
    fn preserves_topological_chain() {
        let reads = [mask(S0), mask(S1), mask(S2)]; // A, B, C
        let writes = [mask(S1), mask(S2), mask(S3)]; // A, B, C
        let order = topo_order::<CS, 3>(reads, writes); // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        assert_eq!(order, [USize::ZERO, USize(1), USize(2)]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // Independent units (no shared stores): no edges, so the order is the
    // identity (lowest-index-first tie-break across the all-ready set).
    #[test]
    fn independent_units_keep_registration_order() {
        let reads = [mask(S0), mask(S1)];
        let writes = [mask(S2), mask(S3)];
        let order = topo_order::<CS, 2>(reads, writes); // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        assert_eq!(order, [USize::ZERO, USize(1)]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // A diamond: A (S0->S1) feeds both B (S1->S2) and C (S1->S3); D (S2,S3->S4)
    // joins. Carrier in reverse registration [D, C, B, A] = indices D0 C1 B2 A3.
    // Valid topo: A first, then B and C (lowest index first), then D. Expected
    // [3, 1, 2, 0]: A(3), then among ready {C1, B2} lowest is C(1), then B(2),
    // then D(0).
    #[test]
    fn diamond_resolves_join_last() {
        let reads = [
            mask(S2).set(S3), // D reads S2, S3
            mask(S1),         // C reads S1
            mask(S1),         // B reads S1
            mask(S0),         // A reads S0
        ];
        let writes = [
            mask(S4), // D writes S4
            mask(S3), // C writes S3
            mask(S2), // B writes S2
            mask(S1), // A writes S1
        ];
        let order = topo_order::<CS, 4>(reads, writes); // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        assert_eq!(order, [USize(3), USize(1), USize(2), USize::ZERO]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // The order is genuinely a `const fn`: it evaluates in a const context.
    #[test]
    fn evaluates_in_const_context() {
        const READS: [AccessMask<CS>; 2] = [mask(S1), mask(S0)]; // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        const WRITES: [AccessMask<CS>; 2] = [mask(S2), mask(S1)]; // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        const ORDER: [USize; 2] = topo_order::<CS, 2>(READS, WRITES); // lint:allow(no-bare-numeric) reason: test carrier length; tracked: #72
        assert_eq!(ORDER, [USize(1), USize::ZERO]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // Column store markers for the carrier-order integration test. Zero-size
    // (the test never executes the units, only derives their access masks).
    #[derive(Copy, Clone)]
    struct Inv;
    #[derive(Copy, Clone)]
    struct Av;
    #[derive(Copy, Clone)]
    struct Bv;

    type One<T> = Cons<Column<T>, Empty>;

    // WuA: Inv -> Av. WuB: Av -> Bv. WuB depends on WuA (shares Av).
    struct WuA;
    impl BuilderInput for WuA {
        type Init = Self;
        type Dispatch = UnitDispatch<Self>;
    }
    impl WorkUnit<Always> for WuA {
        type Read = One<Inv>;
        type Write = One<Av>;
        type Hint = (Immediate, Atomic, Normal);
        type Ctx<'frame> = EngineCtx<
            'frame,
            One<Inv>,
            One<Av>,
            PtrNil,
            ColPtrCons<Inv, ColPtrNil>,
            ColPtrCons<Av, ColPtrNil>,
        >;
        fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
    }

    struct WuB;
    impl BuilderInput for WuB {
        type Init = Self;
        type Dispatch = UnitDispatch<Self>;
    }
    impl WorkUnit<Always> for WuB {
        type Read = One<Av>;
        type Write = One<Bv>;
        type Hint = (Immediate, Atomic, Normal);
        type Ctx<'frame> = EngineCtx<
            'frame,
            One<Av>,
            One<Bv>,
            PtrNil,
            ColPtrCons<Av, ColPtrNil>,
            ColPtrCons<Bv, ColPtrNil>,
        >;
        fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
    }

    // The carrier-order integration: from a real WorkUnit bundle in builder
    // prepend order (register WuA then WuB -> bundle `[WuB, WuA]`), the const
    // `carrier_order` derives the masks via the shipped `MaskProject`/`Locate`
    // fold and computes the topological dispatch order. WuB reads Av (WuA's
    // write), so WuA (carrier index 1) must dispatch before WuB (index 0):
    // ORDER == [1, 0].
    #[test]
    fn carrier_order_reorders_real_bundle() {
        type Stores = Cons<Column<Inv>, Cons<Column<Av>, Cons<Column<Bv>, Empty>>>;
        type Bundle = Cons<WuB, Cons<WuA, Empty>>;
        let order = carrier_order::<Bundle, Stores, _, CS, 2>(); // lint:allow(no-bare-numeric) reason: carrier length; tracked: #72
        assert_eq!(order, [USize(1), USize::ZERO]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // The same over the value-carrying `WuCons` list the scheduler retains as
    // `wu_values`: register WuA then WuB -> `WuCons<WuB, WuCons<WuA, WuNil>>`.
    // `carrier_order` derives the same topological ORDER `[1, 0]` from the value
    // list's TYPE, so `Scheduler::run` needs no separate bundle type param.
    #[test]
    fn carrier_order_over_wucons_value_list() {
        type Stores = Cons<Column<Inv>, Cons<Column<Av>, Cons<Column<Bv>, Empty>>>;
        type Carrier = WuCons<WuB, WuCons<WuA, WuNil>>;
        let order = carrier_order::<Carrier, Stores, _, CS, 2>(); // lint:allow(no-bare-numeric) reason: carrier length; tracked: #72
        assert_eq!(order, [USize(1), USize::ZERO]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }

    // The GCE-safe `carrier_order_dyn` over `<U as Capacity>::Array`: the order
    // the generic `Scheduler::run` computes. `U = Dim<8>` sizes it to the unit
    // capacity; the live prefix `[0, 2)` is the real units in topological order
    // (WuA before WuB), placeholders appended after.
    #[test]
    fn carrier_order_dyn_reorders_prefix() {
        type Stores = Cons<Column<Inv>, Cons<Column<Av>, Cons<Column<Bv>, Empty>>>;
        type Carrier = WuCons<WuB, WuCons<WuA, WuNil>>;
        type U = Dim<8>; // lint:allow(no-bare-numeric) reason: test unit-capacity Dim literal; tracked: #649
        let order = carrier_order_dyn::<Carrier, Stores, _, U, CS>();
        let prefix = &order.as_ref()[..2]; // lint:allow(no-bare-numeric) reason: live unit count; tracked: #72
        assert_eq!(prefix, &[USize(1), USize::ZERO]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected-order literals; tracked: #72
    }
}
