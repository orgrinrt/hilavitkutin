# Design talk: bare vs arvo primitives by position (const-generics, fields, args)

**Date:** 2026-05-30
**Status:** OPEN design discussion with op. Not a round yet; topics captured live.
**Trigger:** C2 slice-3 (#647) hit repeated rustc GCE ICEs/overflows trying to
thread `Cap` const-generic plan dims through generic scheduler code.
**Goal (op):** best possible typed coverage + static checking that every
numeric sits in the correct position with the right semantics, WHILE killing
the recursion/ICE problems that come from arvo primitives in const-generic
position. Explore whether an unstable feature or architectural rethink lets us
keep the typing without the ICE, before retreating to bare usize.

## T1. Root cause (established)

`cap_size : Cap -> usize` and `cap : usize -> Cap` are inverses. A const
generic of type `Cap` cannot index an array (Rust array lengths are `usize`),
so any `[T; N]` with `N: Cap` must be written `[T; cap_size(N)]`. That
`cap_size(...)` in type position is evaluated by `generic_const_exprs` (GCE).

The failure: `cap_size(cap(K))` (the round-trip, which arises when a named
`Cap` const whose value is `cap(K)` is substituted into a `cap_size` bound, or
when an inline `{ cap(usize_param) }` is passed to a `Cap`-const-generic API)
overflows / ICEs in rustc WF evaluation on nightly-2026-05-28. Observed three
ways during slice 3: const-param-out-of-range ICE (struct const-default WF),
and two E0275 overflows (struct cap_size where-clause WF; BundleProject
cap_size bound at the concrete consumer call). The existing plan code dodges it
only because it is called from CONCRETE test fns with NAMED `Cap` consts
(`cap_size(MU)`, single fn, no nesting); from GENERIC code a named `Cap` const
is "unconstrained", forcing the `{ cap(param) }` nesting that overflows.

**Structural conclusion:** a distinct numeric newtype in const-generic
dimension position MUST convert (`cap_size`) to index arrays, and that
conversion leans on GCE, which is incomplete (WATCH feature, ICE-prone). The
only array length that needs zero const-eval is a raw `usize`. So Rust's "array
lengths are usize" + "GCE is incomplete" structurally pushes dimension const
generics toward `usize`.

## T2. The split that matters: dimension vs ConstParamTy-marker

Two different things sit in const-generic position today; the convention must
treat them differently:

- **Dimension / size const generics** (`MAX_UNITS`, `MAX_STORES`, the `N` in
  `[T; cap_size(N)]`): go through `cap_size`, hit the overflow. Candidates to
  become bare `usize`.
- **Typestate / `ConstParamTy` marker const generics** (arvo `UFixed<{IBits(I)},
  {FBits(F)}, S>`, `Bits<const N: BitWidth>`, strategy markers): the
  `adt_const_params` "harness the type system" win. Carry meaning, not size;
  never go through `cap_size`. Must NOT be forced to bare or the arvo numeric
  tower collapses.

So the rule is almost certainly "**size/count/dimension** const generics are
bare `usize`; **meaning-bearing `ConstParamTy` markers** stay arvo." Open: is
that exactly op's intent? (T6 sharpens what's lost.)

## T3. Reverses C0 (#637)

#637 lifted these engine plan/thread dims `usize -> Cap` for type-safety. That
lift is what made them unusable from generic code. The convention reverts the
const-generic POSITION to `usize` while keeping `Cap` in bodies.

## T4. Can we KEEP arvo types in const-generic position? (op's question)

### Unstable features

- `generic_const_exprs` (#76560, WATCH/contested): the source of the ICE.
  Incomplete by design; the const-generics team itself calls the model "very
  broken". Not a fix; it IS the problem.
- `min_generic_const_args` (#132980): the sound successor, but arvo sketch
  `202605291007_min-gca-feasibility` PROVED it cannot express `cap_size(N)`
  array-length patterns (a `type const` RHS cannot use a generic param). Not
  viable.
- `generic_const_args`: can express it, but needs `-Znext-solver=globally`
  (mutually exclusive with GCE at the compiler level) + a ~314-site all-or-
  nothing rewrite, with no incremental validation path. Vetted as not-now.
- `adt_const_params` (#95174, ALLOWED): is what lets `const N: Cap` exist at
  all; not the problem. The problem is GCE evaluating `cap_size` on it.
- Nothing else (const traits, unsized_const_params, lazy_type_alias) addresses
  WF evaluation of a const-fn in array position.

**Verdict:** no unstable feature currently makes `Cap`-const-generic dimensions
robust. The const-generics roadmap is the eventual fix but vetted as not-yet-
usable for this exact pattern. "Stay on Cap and wait for the toolchain" =
living with ICEs indefinitely + building engine correctness on "this nightly
happens not to ICE."

### Architectural

- `Cap` cannot be made to index an array directly (array lengths are `usize`;
  a ConstParamTy newtype can't be a length). So no definition of `Cap` removes
  the `cap_size` conversion. Structural dead end.
- Collapsing the ~12 plan dimension params into fewer would shrink the GCE
  surface but not change the Cap-vs-usize question.
- Different storage model (one max arena + runtime lengths instead of
  const-generic-N arrays) is a deep engine rethink, orthogonal, and fights the
  no-alloc const-sized design.

## T5. Recover const-generic-position static checking via a LINT (no GCE)

Key idea: the typed coverage we lose by going `usize` in const-generic position
can be re-added by a SOURCE-LEVEL lint (mockspace/viola), which checks position
+ semantics WITHOUT leaning on GCE:

- Dimension const generics must be `usize` (enforce the rule; catch arvo-type
  regressions in dim position).
- They must be named/annotated as capacities (intent), e.g. `MAX_*` / `*_CAP`,
  or carry a marker doc-attr.
- The raw `usize` value must be wrapped (`cap(N)` / `USize(N)`) before flowing
  into a Cap/USize-typed runtime context (never used "raw" where the typed
  form is semantically required).
- Inverse rule: arvo types ARE required in const-generic position for the
  meaning-bearing markers (T2), so the lint distinguishes the two classes.

This gives static checking of correct usage at the one position the type system
handles badly, using the tooling layer the workspace already trusts for
discipline the type system can't cheaply express.

## T6. What typed coverage do we actually keep vs lose?

- KEEP (high value): every RUNTIME numeric position (struct fields, fn args,
  returns, locals, consts) stays arvo-typed. This is where values flow and get
  confused; it is the bulk of numeric usage and the real safety win. Untouched.
- LOSE (low value): the type-level assertion "this compile-time dimension
  scalar is a `Cap`". A const generic is a fixed compile-time scalar with no
  runtime identity; it is almost never a confusion site, and `cap(N)` in the
  body re-enters the typed world immediately for any runtime use.
- NET: the retreat sacrifices little real safety and (per T5) a lint recovers
  the position/semantics check. Plus: faster compiles (drops GCE cap_size
  machinery from every signature), ergonomic call sites (no `{ cap(N) }`
  turbofish, usize inlines).

## T7. Scope / migration (if we adopt the usize-dim convention)

~200 `const MAX_*: Cap` params + ~340 `cap_size(...)` sites in hilavitkutin
`plan/`/`thread/`/`dispatch/`, plus arvo `arvo-sparse` (`rcm_reorder_via`,
`block_diagonal_via`), `arvo-spectral` (`k_way_partition`), `arvo-graph`
bitmatrix dims, and `arvo-tensor` (home of `cap`/`cap_size`). arvo migrates
FIRST (dep order): if the plan calls a still-`Cap` arvo fn from generic code it
re-creates the nesting at the arvo boundary. Staged: arvo -> hilavitkutin.
Lint-pack change (mockspace-hilavitkutin-stack-lints) to exempt usize
const-generic dim position + enforce T5.

## Open questions for op

1. Does the T2 split (size-dims -> usize, ConstParamTy markers -> arvo) match
   intent, or broader/narrower?
2. Accept that no unstable feature rescues Cap-const-generics now (T4), so the
   choice is really "usize-dim convention + lint" vs "live with ICEs"?
3. Is the T5 lint-recovers-the-check approach the right way to keep "best
   possible typed coverage + static checking"?
4. Migration shape: arvo-first staged epic, or scope it tighter first?

## T8. The real discriminator (grounded in arvo's own code)

The split is NOT "typed vs bare" and NOT "size-dim vs ConstParamTy-marker". It
is a MECHANISM split, and arvo already lives on both sides of it:

**Does the const-generic value get fed through a const fn in TYPE position
(array length `[T; f(N)]` or where-bound `[(); f(N)]:`)?**

Evidence from arvo (shipping, compiles):
- `UFixed<const I: IBits, const F: FBits, S>` and `Bits<const N: u16, S, Sign>`
  use TYPED (`IBits`/`FBits`) AND bare (`u16`) const generics, with inline
  const-fn args (`Uint<N> = UFixed<{ ibits(N) }, { fbits(0) }, S>`). These WORK
  with no ICE because the const value drives ASSOCIATED-TYPE selection
  (`BitsContainerFor<N, Sign>::T` picks the u8..u128 container) — there is NO
  `[T; f(N)]` array. Type-dispatch, not array-length const-eval.
- `Csr<const ROWS: Cap, const NNZ: Cap>` and the engine's `MAX_*: Cap` feed
  `[T; cap_size(N)]`. These hit GCE and ICE the moment the round-trip
  `cap_size(cap(K))` forms (generic consumer, struct default, or composition).

So the same `{ ibits(N) }` inline-const pattern that works for `UFixed` is the
same shape as the `{ cap(MU) }` that ICE'd for me. The difference is purely:
`ibits(N)` feeds type-dispatch (fine); `cap(MU)` feeds `cap_size` in a
where-bound (ICE).

## T9. The three tiers (mechanism-based)

### Tier 1 — Decidedly impossible (forced bare `usize`)

Const generics that serve as **array lengths / capacity dimensions**. Array
lengths are `usize` by language rule; a typed newtype must convert via
`cap_size(N)` in type position; that conversion is GCE-evaluated and ICEs under
composition / struct-defaults / generic instantiation. No definition of the
newtype removes the conversion (a `ConstParamTy` newtype cannot be an array
length). Structural.
- Engine `MAX_UNITS / MAX_STORES / MAX_EDGES / ...` (`[T; cap_size(N)]`).
- arvo `Csr<ROWS, NNZ>`, `Array`/`Matrix`/`Bits`-multi-limb if array-backed.
- Any `[T; cap_size(N)]` / `[(); cap_size(N)]:` consumed from generic code.

### Tier 2 — Problematic (typed, works, but taxed and generic-hostile)

Array-length const generics consumed ONLY from CONCRETE callers with NAMED
consts (`const MU: Cap = cap(8); f::<MU>()`). This is how the plan tests pass
today. Costs: (a) cannot be consumed from GENERIC code at all (named const =
"unconstrained", `{cap()}` = ICE) — which is exactly why the scheduler could
not call the plan; (b) every signature carries `[(); cap_size(N)]:` bounds
(compile-time tax); (c) the `cap`/`cap_size` dance (ergonomic tax). The
engine's `Cap` dims sit here today: callable from tests, not from the
scheduler.

### Tier 3 — Invisible (typed, works fully) — KEEP and BUILD MORE here

- (a) **Type-dispatch const generics**: value selects an associated TYPE via a
  trait, never an array length. arvo `UFixed`/`Bits` container selection.
  Proven to work with typed const generics + inline const-fn args.
- (b) **Pure marker / strategy type params**: `S: Strategy`, niche markers.
  Type tags, zero const-eval.
- (c) **ALL runtime numeric positions**: struct fields, fn args/returns,
  locals, non-generic consts, associated consts (not used as array lengths).
  `#[repr(transparent)]` arvo newtypes + full trait impls. The vast bulk of
  numerics. Untouched by any of this; this is where arvo already delivers
  "first-class primitive" feel.

## T10. Can we drop Tier-1/2 to a better tier?

- **Tier 1 -> Tier 3 (type-dispatch instead of array)?** Works only when "N of
  something" can be a CONTAINER TYPE chosen per N (arvo does this for bits:
  pick u8..u128). For arrays of arbitrary structs (engine units, sparse rows),
  there is no container type to dispatch to — it is inherently `[T; N]`. So
  engine/Csr capacity dims CANNOT drop to Tier 3. The array-length need is
  irreducible.
- **Tier 1 -> Tier 2 (consume only concretely)?** The scheduler is generic over
  the registered bundle; it MUST consume the plan dims generically. Tier 2 is
  not even available to it. So the engine dims cannot stay typed-but-concrete.
- **Tier 2 -> Tier 3 (go bare usize)?** Yes: bare usize array lengths need zero
  const-eval, work generically, compile faster, drop the `{cap()}` turbofish.
  Loses the typed const-generic — but T11 shows that loss is near-zero real
  coverage.
- **Net:** the ONLY const generics forced bare are array-length/capacity dims.
  Everything in Tier 3 (type-dispatch keys, markers, every runtime position)
  STAYS arvo-typed. The convention is therefore narrow and mechanism-defined,
  not a broad retreat.

## T11. Why the lost coverage is near-zero (and how we recover it)

A capacity/array-length const generic is a compile-time-fixed scalar with no
runtime identity. It is almost never a confusion site (you don't accidentally
pass a "store count" where a "unit count" goes — they are distinct type
parameters by name/position already). The actual runtime count (how many units
exist now) is a `Cap`/`USize` runtime VALUE in a field/arg — fully typed, Tier
3. The `cap(N)` at the body boundary re-enters the typed world for any runtime
use immediately.

Recover the position/semantics check WITHOUT GCE, via a lint:
- Array-length const generics must be bare `usize`; flag arvo types there
  (they would ICE).
- They must read as capacities (name convention / annotation).
- The raw `usize` must be wrapped (`cap(N)`/`USize(N)`) before flowing into a
  typed runtime context; flag raw use where the typed form is required.
- INVERSE: type-dispatch / marker const-generic positions MUST stay arvo-typed
  (catch a regression that bares them).

This is "build on the typestate MORE" via the tooling layer the workspace
already trusts for what the type system can't cheaply express, applied to the
one position where the type system itself ICEs.

## T12. Refined rule statement (candidate)

> Const-generic parameters that are **array lengths / capacity dimensions** are
> bare `usize` (language-forced; typed newtypes ICE there via GCE). Const
> generics that drive **type dispatch** or act as **markers** stay arvo-typed
> (`IBits`, `FBits`, `Strategy`, ...). Every **non-const-generic** numeric
> position (fields, args, returns, locals, consts) is arvo-typed, never bare.
> Bodies wrap a bare dimension via `cap(N)`/`USize(N)` at first typed use. A
> lint enforces all four clauses.

Open: is "array-length/capacity" the precise boundary, or are there
type-dispatch capacity patterns (arvo container-style) we should push the
engine toward to keep MORE dims typed? (T10 says engine unit/store arrays
can't, but worth an expert eye.)

## T13. Op direction (2026-05-30): wrap the array, hide the bare primitive

Op on option 1: if it is possible, do it. No real reason to use plain arrays;
wrap them in trait contracts / wrappers, normalise the generics+consts to cut
boilerplate, and hide the bare primitives behind typestate so they are NEVER
used directly in live code (everything flows through the traits/wrappers).
Op on option 2 (lints): not needed yet; track as very-low-priority tasks at the
bottom of the pile.

### The architecture this points to: capacity-as-a-TYPE, array-as-associated-type

Generalise arvo's `BitsContainerFor` (which dispatches bit width -> container
TYPE) to arbitrary arrays. The dimension stops being a const generic and
becomes a TYPE; the array length becomes a LITERAL inside a concrete impl;
nothing const-evaluates in type position, so GCE never runs and cannot ICE.

```rust
// arvo substrate (new): a capacity is a named type carrying its storage.
pub trait Capacity {
    type Array<T>;                 // concrete impl: = [T; 64]  (literal length)
    const CAP: Cap;                // typed capacity for the surface API
    fn empty<T: Copy>(fill: T) -> Self::Array<T>;
    fn as_slice<T>(a: &Self::Array<T>) -> &[T];
    fn as_mut_slice<T>(a: &mut Self::Array<T>) -> &mut [T];
}
// macro-generated ladder: impl Capacity for Cap8/Cap16/Cap32/Cap64/...
// each body is `type Array<T> = [T; 64]; const CAP: Cap = cap(64); ...`
// the ONLY bare `usize`/raw array in the whole stack lives in these
// macro-generated impl bodies, never in live code.
```

Why this dodges GCE entirely:
- No const generic in dimension position. `C: Capacity` is a TYPE parameter.
- The array length is a literal in a CONCRETE impl (`[T; 64]`), never a generic
  const expression, so GCE has nothing to evaluate.
- Runtime length is `C::CAP` (a typed `Cap`) or an assoc const used only as a
  loop bound, never as an array length.
- Generic consumers see only `C: Capacity`, `C::Array<T>` (assoc-type
  projection), `C::CAP` (runtime). The scheduler can thread it through generic
  code with zero `cap_size`, zero `{cap()}`, zero overflow.

### Normalisation (op's "reduce boilerplate"): bundle the dims into one type

```rust
pub trait PlanDims {
    type Units: Capacity;
    type Stores: Capacity;
    type Edges: Capacity;
    // ... the ~12 engine dims, each a Capacity TYPE
}
pub struct DefaultPlanDims;   // the engine budget, one impl
impl PlanDims for DefaultPlanDims { type Units = Cap64; type Stores = Cap64; ... }
```

Then `Scheduler<Cfg, WuVals, Vals, M, D: PlanDims = DefaultPlanDims>` carries ONE
dims TYPE param (type-level defaults work fine, no const-default ICE), and the
plan/thread/dispatch structures take `D` and use `D::Units::Array<UnitMeta>`
etc. One param instead of ~12 const generics; fully typed; GCE-free.

This is strictly MORE typestate than today: a capacity is a named type that can
carry bounds, relationships, and semantics; the bare `usize`/raw array is
quarantined to macro-generated impls that live code never names.

### What this costs / risks (to de-risk before committing)

- It is an arvo-FIRST foundational addition (`Capacity` + the ladder + the
  bundled-dims pattern), then hilavitkutin (and arvo's own `Csr`/`Array`/
  `Matrix`, which today use `Cap` const generics) adopt it. Large, staged,
  cross-repo.
- GAT (`type Array<T>`) + generic access via `as_slice`/`Index` bounds: needs a
  feasibility SKETCH to confirm (a) it genuinely never triggers GCE when
  consumed from fully-generic code (the whole point), (b) construction of
  `C::Array<T>` generically is clean (Copy fill / MaybeUninit), (c) nested
  projection `D::Units::Array<T>` resolves without solver grief.
- Open: does the engine ever need a capacity not known at the arvo ladder
  granularity (an exact N), or is a power-of-two/standard ladder + "next size
  up" acceptable? (arvo's Bits picks the minimum container that fits; same idea
  - `Capacity` for "at least N" rounding up to the ladder.)

### Verdict path

Sketch the `Capacity` + `PlanDims` pattern (arvo, throwaway) proving GCE-free
generic consumption + construction + access. If it holds, it becomes an
arvo design round (ship `Capacity` + ladder + adopt in arvo containers), then a
hilavitkutin round (engine + scheduler adopt `PlanDims`), and C2 slice 3 lands
on top cleanly. If the sketch finds a GCE/solver wall, fall back to the bare-
usize-dim convention (T12) which is proven to compile.
