# Findings: capacity-as-a-TYPE, array-as-associated-type

**Date:** 2026-05-30
**Toolchain:** nightly-2026-05-28
**Context:** numeric-position convention (research note
`202605301130_numeric-position-convention.md`, T13). Op direction: wrap arrays,
hide the bare primitive behind typestate, never use bare arrays/usize in live
code, normalise dims to cut boilerplate.
**Outcome:** WORKS, and with NO `#![feature(...)]` gates.

## Hypothesis

Express the engine's fixed-capacity arrays so the dimension is a TYPE (not a
`Cap` const generic) and the array length is a LITERAL inside a concrete impl.
Then no const expression is evaluated in type position, `generic_const_exprs`
never runs, and the storage can be constructed / filled / indexed / walked from
fully-generic code without the `cap_size(cap(N))` ICE/overflow that killed every
`Cap`-const-generic attempt tonight.

## Result: WORKS

`cargo +nightly-2026-05-28 run` prints
`WORKS: default=[3, 2, 1, 0] big=[5, 4, 3, 2, 1, 0]`. Confirmed:

1. `trait Capacity { type Array<T>; const CAP: Cap; const N: usize; fn
   empty/as_slice/as_mut_slice }` with a macro-generated rung ladder
   (`Cap4/Cap8/Cap16`) binding literal-length arrays (`type Array<T> = [T; 4]`).
2. `trait PlanDims { type Units: Capacity; type Stores: Capacity; }` bundles the
   dims into ONE type param. `DefaultPlanDims` is the analogue of
   `DefaultRunCfg` (one impl = the engine budget).
3. `struct Plan<D: PlanDims>` with fields of nested projections
   `<D::Units as Capacity>::Array<usize>` compiles and constructs generically.
4. A fully-generic `build_and_walk<D: PlanDims>` constructs the plan, fills a
   topo permutation through `as_mut_slice`, and walks it through `as_slice` and
   `D::Units::N` (runtime loop bound). This is the exact shape (generic build +
   generic walk over fixed-capacity storage) that ICE'd under the
   `Cap`-const-generic form.
5. Consumed both from a non-generic caller with `DefaultPlanDims` (mirrors
   `Scheduler::builder()` defaulting) and through a nested-generic `outer<D>`
   wrapper. The typed surface `D::Units::CAP` (a `Cap`) is reachable and
   distinct per dims.

**Crucially: no feature gates.** GATs, associated consts, and literal-length
arrays in concrete impls are all stable. The pattern compiles outside the entire
`generic_const_exprs` machinery, so the ICE class cannot recur by construction.

## Why this is GCE-free by construction

- No const generic in dimension position: `C: Capacity` / `D: PlanDims` are TYPE
  params.
- Array lengths are literals in CONCRETE impls (`[T; 4]`), never generic const
  expressions, so the GCE evaluator has nothing to run.
- Runtime length `C::N` is used only as a loop bound, never as an array length.
- Generic code only ever sees `C::Array<T>` (assoc-type projection) and `C::N`
  (assoc const, runtime). No `cap_size`, no `{cap()}`, no where-bound const
  expressions.

This is the same escape arvo's `BitsContainerFor` already uses (bit width ->
container TYPE), generalised from "pick a scalar container" to "pick an array
type".

## What this does NOT yet cover (for the architect review + arvo round)

- **Toy `Cap`**: stand-in newtype; the real one is `arvo::Cap`. Trivial swap.
- **Ladder granularity**: rungs are exact (`Cap4/8/16`). The engine needs either
  a full ladder or an "at least N -> round up to next rung" selector (arvo Bits
  picks the minimum container that fits; same idea). Open: is round-up
  acceptable for plan arrays, or do some need exact N? (Round-up wastes a little
  stack; plan arrays are small metadata.)
- **Non-Copy construction**: `empty<T: Copy>(fill)` covers the engine's POD
  metadata arrays (Copy). Non-Copy element types would need a MaybeUninit-based
  constructor on `Capacity`. Confirm the engine's arrays are all Copy (they
  appear to be: `UnitMeta`, masks, ids).
- **Refactoring arvo's existing `Csr`/`Array`/`Matrix`**: today they use `Cap`
  const generics + `[T; cap_size(N)]`. Moving them to the `Capacity` form is the
  bulk of the arvo-side work and needs to preserve their algorithms + bench
  characteristics. Not probed here.
- **Indexing ergonomics at scale**: this sketch uses `as_slice`/`as_mut_slice`
  accessors. Whether the engine's many access patterns want `Index`/`IndexMut`
  bounds, iterator adapters, or typed `USize` indexing is an API-design question
  for the arvo round.
- **Two-dimensional / ragged storage** (bit matrices, CSR row/nnz pairs): the
  sketch covers 1-D arrays. CSR-shaped storage needs a `Capacity` pair or a
  2-D analogue; confirm the pattern composes.

## Verdict

The architecture is feasible and structurally ICE-proof. Proceed to: (1)
architect review of the boundary + ladder-granularity + arvo-container-refactor
feasibility + migration shape; (2) if it holds, an arvo design round shipping
`Capacity` + the ladder + adopting it in arvo containers; (3) a hilavitkutin
round adopting `PlanDims`; (4) C2 slice 3 lands clean on top. Fallback if a
later wall appears: the bare-usize-dim convention (T12), already proven to
compile.

## Addendum (2026-05-30): 2-D extension proven + architect review

The architect review (feature-dev:code-architect) chose T13 over T12 and raised
ONE objection: the engine is predominantly multi-dimensional, and `[[T;C];R]`
does not impl `AsRef<[T]>`, so the 2-D case needed its own proof.

Resolved in this same sketch: a 2-D matrix is the COMPOSITION of two 1-D
capacities, `R::Array<C::Array<T>>` — no `Capacity2D` trait. Added a
`type Array<T>: AsRef<[T]> + AsMut<[T]>` bound to `Capacity` (stable, no
const-eval); nested access is `.as_ref()[r].as_ref()[c]`. Generic construct +
set + get + diagonal-walk over `Matrix<R: Capacity, C: Capacity, T>` compiles
and runs (`diag=33`), still with no feature gates.

One real residue: the 2-D constructor carries a `where <C as Capacity>::Array<T>:
Copy` bound (the outer `empty([inner; R])` needs the inner row Copy). It is
trivially satisfied for every engine POD element type and discharges at each
concrete use; it just has to be stated on nested constructors. Minor.

Architect's other load-bearing points: `Csr`/`PlanInputs` "two dims" are two
independent 1-D capacities (already handled, not 2-D); `Capacity::N` replaces
`cap_size(N)` loop bounds 1:1; all engine plan arrays are Copy; migration is
arvo-first (Capacity + ladder in arvo-tensor, refactor Csr/Array/Matrix), then
hilavitkutin `PlanDims`, then C2 slice 3. Bench characteristics unaffected
(same array sizes; assoc-const length vs const-fn length are identical to LLVM).

Net: T13 is feasible for 1-D AND 2-D, structurally GCE-free, and delivers richer
typed coverage than either today's `Cap`-const-generics or the T12 bare-usize
fallback. Ready for op decision -> arvo design round.

## Addendum 2 (2026-05-30): op refinement — one generic `Dim<const N: usize>`

Op's refinement: drop the macro rung ladder; make a single generic marker
`Dim<const DIM: usize>` impl the (non-generic) `Capacity` trait, using `DIM`
directly as the array length and mapping `CAP: Cap = cap(DIM)`.

Validated in the sketch (still no feature gates, `WORKS: ... diag=33`):
- `struct Dim<const DIM: usize>; impl<const DIM: usize> Capacity for Dim<DIM> {
  type Array<T> = [T; DIM]; const CAP: Cap = cap(DIM); const N: usize = DIM; ... }`
- The implementing TYPE is generic over `const DIM: usize`; the `Capacity`
  TRAIT stays non-generic, so consumers still bind `C: Capacity` with NO const
  param (the type-dispatch property is preserved — `D::Units` is a type).
- `[T; DIM]` is plain min-const-generics (no `cap_size`, no GCE). `cap(DIM)` in
  the associated-const VALUE position works (the one bit that was unproven).
- `Dim<4>`, `Dim<8>`, `Dim<16>` used in `PlanDims` impls; `Dim<N>` for any
  exact N. The whole 1-D + 2-D suite still compiles and runs.

This is strictly better than the rung ladder: no macro, no fixed rungs, and the
earlier open "exact-N vs round-up-to-rung" question is MOOT — you get exact N
for free. The only bare `usize` literal in live code is the `N` in
`Dim<N>` inside a `PlanDims` impl (the "declare the budget once" site); live
code threads `D: PlanDims` / `C: Capacity`, never a length.

Final shape for the arvo round: a non-generic `Capacity` trait + a single
generic `Dim<const N: usize>` marker (real name TBD; `Cap` is taken by the
value newtype) + the `PlanDims` bundling pattern. Naming and the
`arvo-tensor` home are the round's first topics.
