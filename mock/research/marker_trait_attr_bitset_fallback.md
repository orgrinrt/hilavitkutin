# `feature(marker_trait_attr)` keystone risk and bitset fallback shape

**Date:** 2026-05-05.
**Round of origin:** 202605042200, audit C4 remediation.
**Status:** Research note only. Not built. Captured for the doc CL to cite as the substrate's planned response if `feature(marker_trait_attr)` becomes unviable.

## Why this note exists

Round-4's typestate substrate (Kit + AccessSet + ContainsAll) is built on `#[marker]` traits with permitted overlapping impls. The recursive HList shape requires both impls of `Contains<X>` (head-match and tail-recurse) to coexist; without `#[marker]` they collide on coherence (E0119), with `#[marker]` they resolve via the trait solver picking any matching occurrence.

`feature(marker_trait_attr)` (rust-lang/rust#29864) has been unstable since 2017. Unlike `feature(const_trait_impl)` (which has substantial recent stabilisation activity) or `feature(adt_const_params)` (also tracking toward stabilisation), `marker_trait_attr` has had no visible stabilisation movement. Its coherence semantics have been periodically relitigated, and a future rustc could tighten or reshape marker-trait coherence rules in a way that breaks the substrate's pattern.

Round-4 accepts this risk explicitly. This note documents the planned fallback if the keystone fails: an alternative AccessSet encoding using a compile-time bitset over a registered store-table.

## Failure mode the fallback responds to

Concrete trigger conditions:

- A future rustc removes or restricts `feature(marker_trait_attr)`.
- A coherence-rules change makes overlapping marker-trait impls unsound in some way that affects the recursive Contains shape.
- Cross-crate marker-trait resolution becomes inconsistent (current resolution is per-crate; a global change would break our pattern).

In any of these cases, the substrate's `Contains` and `ContainsAll` traits stop type-checking, and the typestate proof becomes unreachable. Round-4 design ships under the bet that this does not happen during the substrate's pre-1.0 lifetime; if it does, the bitset fallback is the planned response.

## Bitset fallback shape (general)

The fallback replaces the recursive HList AccessSet with a compile-time bitset over a per-app store-table indexed by `ConstParamTy` markers. High-level shape:

1. **Store-table indexing.** Each store marker (StringInterner, Clock, LeafA, etc.) gets a `ConstParamTy`-implementing index assigned at registration time. The substrate provides a `StoreIndex` ConstParamTy newtype.
2. **AccessSet as bitset.** `AccessSet` becomes a `Bits<N, S>` where N is the count of registered stores in the app's StoreIndex. Each bit position corresponds to a store-table slot.
3. **Contains as bit-test.** `Contains<X>` becomes a const-trait method that checks whether the bit at `X::INDEX` is set.
4. **ContainsAll as bitwise-AND.** `ContainsAll<L>` becomes `(stores & L) == L`. Decidable in const at the trait-solver level, no recursion needed.

The shape sits inside arvo's existing `Bits<const N: Width, S: Strategy>` substrate, which is already shipped and stable in the workspace.

### Sketch-level pseudocode

```rust
// app-side: StoreIndex enumerates every registered store marker.
#[derive(ConstParamTy)]
pub struct StoreIndex(pub Bits<8>);

impl StoreIndex {
    pub const STRING_INTERNER: Self = Self(Bits::from(0));
    pub const CLOCK:           Self = Self(Bits::from(1));
    pub const LEAF_A:          Self = Self(Bits::from(2));
    // etc.
}

// substrate-side: AccessSet is a bitset over StoreIndex.
pub struct AccessSet<const BITS: Bits<256, Cold>>;

// Contains<X> for X: HasIndex projects to bit-test:
pub trait Contains<X: HasIndex> {}
impl<const BITS: Bits<256, Cold>, X: HasIndex> Contains<X> for AccessSet<BITS>
where
    BITS::HasBit<{ X::INDEX }>:,  // or equivalent const-eval guard
{
}
```

(The exact const-generic guard mechanism depends on which `feature(generic_const_exprs)` capabilities are stable when the fallback is needed. The approach is sketch-level only.)

### Trade-offs

The bitset fallback would have the following characteristics:

- **Pro: no recursion.** The trait solver does not walk a Cons chain; ContainsAll is a single const-eval bitwise AND.
- **Pro: natural dedup.** Bitsets are sets by construction; setting the same bit twice is idempotent.
- **Con: per-app StoreIndex enumeration.** Apps must enumerate every store marker they touch in a single ConstParamTy. Cross-crate kit composition becomes harder; a kit cannot independently choose its store indices because they are app-level facts.
- **Con: const-generics dependence.** The fallback leans heavily on `feature(adt_const_params)` and possibly `feature(generic_const_exprs)`. The latter has its own instability story; if `marker_trait_attr` is gone AND `generic_const_exprs` is still unstable, the fallback may need adjustment.
- **Con: bigger redesign.** Switching encodings is not a drop-in change. Kit, WorkUnit, AccessSet, ContainsAll all reshape. Doc CL gets a new revision.

## When the fallback would be triggered

Round-4 design proceeds without building the fallback. The fallback is documented here so that:

1. A future agent or human reading the substrate after a `marker_trait_attr` rule change has a starting point for the redesign.
2. The doc CL can cite this note when capturing the keystone risk, demonstrating that the substrate has a planned response, not just acknowledgement.

If the trigger fires before round-4 ships, this note becomes a real design round. If after, it becomes the round-N plan. Pre-1.0 churn is acceptable per `no-legacy-shims-pre-1.0.md`.

## Other nightly-feature dependencies (risk-class context)

Round-4 substrate depends on multiple nightly features. The risk profile differs per feature:

| Feature | Risk profile | Substrate impact if removed |
|---------|--------------|------------------------------|
| `marker_trait_attr` | High. 9-year unstable, no visible track. | AccessSet substrate redesigned; this note's bitset fallback is the response. |
| `const_trait_impl` | Lower. Active stabilisation track. | Numeric primitives lose const-callable; substantial but not architectural. |
| `adt_const_params` | Lower. Active stabilisation track. | ConstParamTy newtypes need workarounds; primitive-level concern. |
| `generic_const_exprs` | High. Still volatile. | Some const-generic guards reshape; case-by-case. |

The doc CL groups these by risk class rather than singling out `marker_trait_attr`. This note is the substrate-keystone-specific response; analogous notes would be written for any other high-risk dependency if and when its substrate role becomes load-bearing.

## Cross-references

- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit topic, finding C4.
- `mock/research/sketches/202605050530_deep_stacking/sketch.rs`. S1; uses `marker_trait_attr` directly.
- `mock/research/sketches/202605050700_deep_stacking_d5/sketch.rs`. S1b; same.
- `mock/research/sketches/202605050615_kit_taxonomy/sketch.rs`. S5; same.
- `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md`. Frames the willingness to depend on unstable features for runtime / correctness wins.
- arvo `mock/crates/arvo-bits/`. Where `Bits<const N: Width, S: Strategy>` lives; the substrate-level primitive the fallback would lean on.
