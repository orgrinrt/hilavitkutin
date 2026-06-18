# Cap-Lifting Shapes Findings

HYPOTHESIS: the engine's three hardcoded caps (GATE2_MAX_UNITS=256 and
GATE2_MAX_ACCUMS=16 in plan/grouping.rs, plan_dirty `[AtomicBool; 256]` in
scheduler/mod.rs, MAX_CORES=256 in thread/class.rs) can be lifted to
consumer-tunable values on nightly-2026-05-28 (1.98.0-nightly cced03bfd),
despite the two documented blockers: GCE rejecting FIELD ACCESS in generic
constants (the `Cfg::MAX_PLAN_AFFECTING_RESOURCES.0` form), and a
`cap_size(CU::CAP)` array-length bound re-proven through the const-gated
walk's type-level recursion overflowing the trait solver. Expected: the
Capacity associated-type pattern (the convention PlanDims already uses for
topo_order / fiber_dispatch) dodges both walls because the cap is a type and
no const expression ever sits in array-length position; the other shapes
trade GCE exposure against threading cost.

Probes compile and run against the real crates: hilavitkutin +
hilavitkutin-api as path deps, arvo dev HEAD. One cargo feature per shape
(`cargo run --features sN`); the crate-level feature gates are per-shape on
purpose, so the s1 build proves itself GCE-free (only `const_trait_impl` is
enabled for it, see main.rs cfg_attr).

## Why the failing case differs from what PlanDims does today

Both walls are properties of GCE anon consts, which the PlanDims pattern
never creates. `[AtomicBool; Cfg::MAX_PLAN_AFFECTING_RESOURCES.0]` puts a
projection-plus-field-access EXPRESSION into type position; rustc's GCE
grammar whitelists what an anon const referencing generics may contain, and
tuple-field access is outside the whitelist (exact error below; the
associated const itself is fine, the `.0` is not). The Capacity pattern
instead resolves `<C as Capacity>::Array<T>` by ASSOCIATED-TYPE projection:
the concrete impl (`Dim<N>`) binds the GAT to a literal-length `[T; N]`, so
the trait solver only normalizes a type projection and no generic constant
exists to prove well-formed. That is also why the capacity product
`MAX_CORES * GATE2_MAX_ACCUMS` must become the nested composition
`Cores::Array<Accums::Array<T>>` under s1: a product needs an expression,
nesting needs only two projections.

## s0: the documented wall, verbatim

OUTCOME: FAILS WITH

```
error: overly complex generic constant
  --> src/s0.rs:18:30
   |
18 |     plan_dirty: [AtomicBool; Cfg::MAX_PLAN_AFFECTING_RESOURCES.0],
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field access is not supported in generic constants
   |
   = help: consider moving this anonymous constant into a `const` function
   = note: this operation may be supported in the future
```

The scheduler comment's claim is exact and current on this nightly. Note the
help text: rustc itself prescribes shape s2.

## s1: Capacity associated types (the PlanDims pattern), every cap

OUTCOME: WORKS. Compiles WITHOUT generic_const_exprs in the consumer crate;
the build enables only const_trait_impl.

Three probes, all green at `Dim<256>`-shaped defaults and at a
consumer-shrunk `TinyCaps` (`Dim<8>` / `Dim<4>` / `Dim<2>`):

1. Scheduler-shaped struct. `plan_dirty:
   <C::PlanAffecting as Capacity>::Array<AtomicBool>` (the #345 field),
   `worker_ctxs: <C::Cores as Capacity>::Array<WorkerCtx>` (the MAX_CORES
   array), and the per-(core,accum) publish array as the nested 2-D
   composition `Cores::Array<Accums::Array<AtomicUsize>>` replacing the flat
   `[_; MAX_CORES * GATE2_MAX_ACCUMS]`. Runtime init through
   `Capacity::from_fn` (AtomicBool is not Copy, so `filled` does not apply;
   `from_fn` does). Reads and writes through `as_ref()` / `as_mut()`.
2. classify_cores lift: returns `<C::Cores as Capacity>::Array<CoreClass>`
   instead of `[CoreClass; MAX_CORES]`, budget read from
   `classes.as_ref().len()`.
3. The const-grouping scratch (the GATE2_MAX_UNITS wall): a masks_of ->
   final_phases_of -> phase_of shaped 3-layer `const fn` chain over a
   BundleMasks-shaped slice-taking `[const]` cons recursion, scratch typed
   `<CU as ConstCapacity>::Array<_>` built by `ConstCapacity::filled` and
   walked by `get` / `set`. Const-evaluated into `const` items at `Dim<64>`
   and `Dim<8>`, and through a 64-deep unit list at `Dim<128>`.

One upstream gap: passing the GAT scratch to the slice-taking fill needs a
CONST slice accessor, and `Capacity`'s `AsRef`/`AsMut` is not
const-callable. The sketch-local `CapSliceMut` const trait (two methods,
`&[T; N] -> &[T]` and `&mut [T; N] -> &mut [T]`, one `Dim<N>` impl whose
bodies are the built-in unsized coercion) stands in for that arvo-tensor
addition. That is the entire upstream cost of this shape.

Threading cost, counted against the engine: `plan_dirty`, `worker_ctxs`,
and `gate2_accum_live` all live on `Scheduler`, which already carries
`D: PlanDims`, so the struct-side lift is two NEW associated types on
PlanDims (PlanAffecting, AccumsPerCore; Cores and Units already exist) and
one line each in the 14 `impl PlanDims` sites (2 in src, 12 in tests).
classify_cores gains one generic parameter (2 callers). In grouping.rs, the
22 GATE2_MAX_UNITS sites and 17 GATE2_MAX_ACCUMS sites swap fixed-array
types for GAT projections plus the slice-bridge call; the grouping const fns
already carry the CU parameter, so no signature gains a new generic.

## s2: bare usize const generics via free const fns

OUTCOME: WORKS, both forms: `usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)`
(non-generic const fn, associated-const argument, the exact `cap_size`
precedent from plan/core_program.rs) and `plan_res_of::<Cfg>()` (generic
const fn). Field construction with
`[const { AtomicBool::new(false) }; usize_raw(..)]` over the generic length
also works. The field access moves into the const fn body, which is ordinary
const evaluation; the anon const at the use site is a whitelisted call
expression.

Threading cost: the `where [(); usize_raw(Cfg::MAX_PLAN_AFFECTING_RESOURCES)]:`
bound is viral. It must repeat on the struct, on every impl block, and on
every free fn naming the type (all three demonstrated). Every one of those
sites keeps a live GCE anon const, so this shape extends the engine's GCE
footprint instead of shrinking it.

## s2b: cap_size(CU::CAP) in const-fn locals through the call chain

OUTCOME: WORKS. Scratch locals `[USize; cap_size(<CU as Capacity>::CAP)]`
with the `[(); ..]:` bound repeated on each of the three chained const fns,
each also carrying the `[const]` recursion bound. Also WORKS in the
projection form faithful to the engine's threading, where the capacity
arrives as a PlanDims-style associated-type projection
(`cap_size(<D::Units as Capacity>::CAP)`) re-proven through two further
generic hops.

## s2c: cap_size(CU::CAP) arrays in the recursive trait signature

OUTCOME: WORKS, at depth 7 and at depth 64 (a macro-built 64-unit cons
list), with the generic constant inside the recursive trait's method
signature and the `[(); ..]:` bound on every cons-cell impl. The documented
trait-solver overflow did NOT reproduce in this reduced fold on this
nightly. The reduction carries one bound per recursion step; the real
const-gated walk compounds BundleMasks (4 type params) with per-unit
MaskProject witness bounds, GateWith, and the RunFiber where-blocks, so the
overflow documented at GATE2_MAX_UNITS is a property of that compounded
obligation set (or of an older nightly), not of cap_size-through-recursion
as such. Treat the engine-side claim as UNREPRODUCED-IN-REDUCTION: an
in-engine probe is required before relying on either the wall or its
absence.

## s3: macro-generated per-cap instantiations at blessed sizes

OUTCOME: WORKS, trivially (no generics, no GCE, no const traits). A macro
stamps a full concrete struct + impl per cap bundle; three instantiations
(256/256/16, 8/4/2, 1024/512/32) compile and run. The costs are structural:
the consumer picks a NAME, not a type or a number, so nothing generic over
the engine can consume the result without re-introducing a trait (which
re-raises the other shapes); each instantiation duplicates the whole surface;
and the flat `cores * accums` product is only expressible because the
lengths are literals.

## s4: associated-const indirection through a helper trait

OUTCOME: WORKS. `trait CapsUsize { const PLAN_DIRTY: usize; }` with a
blanket `impl<C: RunCfg> CapsUsize for C` whose const definition body does
the `.0` field access (ordinary const evaluation), leaving a plain
associated-const PATH `<Cfg as CapsUsize>::PLAN_DIRTY` in the anon const.
Same viral `where [(); ..]:` threading as s2; the difference is taste (path
form vs call form) plus one blanket impl replacing per-site projection fns.

## s4b: the s2c recursion with the bare-usize associated const

OUTCOME: WORKS, at depth 7 and depth 64. The path form `<CU as WCap>::W`
and the call form `cap_size(CU::CAP)` behave identically through the
recursive trait signature on this nightly; the solver does not distinguish
the two anon-const shapes here.

## COMPARISON

The field-access wall (s0) is real, exact, and current; every other probed
shape clears it. The shapes split on where the cap lives. s1 makes the cap a
TYPE and is the only shape that is GCE-free in the consumer, matches the
convention the engine already standardised post-#652 (PlanDims capacities),
rides the existing `D: PlanDims` threading so its marginal cost is two
associated types plus 14 impl lines, and covers all three caps including the
const-grouping scratch; its sole prerequisite is a small arvo-tensor
addition (const slice accessors for the ConstCapacity GAT array, proven here
by the sketch-local CapSliceMut). s2 and s4 keep the cap a bare usize NUMBER
read off RunCfg and are the minimal-diff fix for plan_dirty alone, but each
naming site pays a viral `[(); ..]:` bound and keeps GCE load-bearing at the
consumer surface, the direction the engine has been migrating away from. s3
works but is strictly dominated by s1 once s1 is available, since it
forfeits type-level consumption of the cap. The depth-64 and
projection-form probes failing to reproduce the documented grouping
overflow (s2b/s2c/s4b all green) means the GATE2_MAX_UNITS comment's wall
is either specific to the full walk's compounded obligations or stale on
this nightly, and EITHER way the s1 capacity route never constructs the
offending generic constant at all.

Two design notes for the lift round this sketch unblocks (#121/#345/#649):
a Capacity-typed knob cannot live on RunCfg without adding arvo-tensor to
hilavitkutin-api (its deps are arvo + arvo-bitmask today), so the natural
home for PlanAffecting / AccumsPerCore is PlanDims on the engine side, with
`RunCfg::MAX_PLAN_AFFECTING_RESOURCES` either retired or redefined as the
documented runtime mirror of the dimension; and the flat
`MAX_CORES * GATE2_MAX_ACCUMS` publish array must become the nested
`Cores::Array<Accums::Array<_>>` composition, because a capacity product in
type position would need exactly the GCE expression s1 exists to avoid.

Next step unblocked: the cap-lift design round (PlanDims gains
PlanAffecting + AccumsPerCore; arvo-tensor gains const slice accessors on
ConstCapacity or a sibling const trait; grouping scratch and the scheduler
fields migrate to GAT arrays; an in-engine probe re-tests the
GATE2_MAX_UNITS overflow claim against the real walk before the grouping
migration is scoped).
