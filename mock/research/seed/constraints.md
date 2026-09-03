# Constraints: Cross-Cutting Rules

These are not domains but rules that govern every domain. Most are enforced
by lints; all are enforced by design review.

## Architectural bans

No alloc: no Vec, Box, String, or the alloc crate anywhere in any
hilavitkutin crate; all storage is consumer-provided backing memory or
stack arrays. `dyn` is banned entirely, no exceptions. `std::any` is banned:
no TypeId, no type erasure through Any (the erased static-shape addressing
in [[storage]] is a designed descriptor mechanism, not Any). The type
taxonomy is trait, newtype, repr(C) enum, or primitive; raw String/Vec/Box
appearing anywhere must be documented newtypes. Runtime spawn is banned; the
pool pre-allocates. Linker-magic registration is banned ([[contracts]]).
DuckDB and the morsel literature are references, not dependencies: own
implementation throughout.

The `&self` receiver rule on Context is listed here as well as in
[[contracts]] because it is cross-cutting correctness: `&mut self` anywhere
in the Context API breaks every fused fiber through noalias write
reordering.

## Design principles

The founding principle set, kept as the working discipline: contracts before
implementations (traits first, always); ZSTs are the logic layer; no dyn
escape hatches, redesign instead; every heap allocation documented (and in
this engine, absent); columnar storage maps to schema-as-traits
consumer-side; repr(C) plus fn pointers as the minimal ABI surface at FFI
boundaries only; registration complete before work begins; the compiler
metaphor forces data-transformation thinking; no ad-hoc logic where a
reusable abstraction serves; never a pragmatic shortcut, change the design
not the implementation; source code lies, trust the design record; no rule
without a lint, no lint without a test; rules live in the workflow
templates; typed IDs everywhere; production quality everywhere including
scaffolding; responsible tooling attribution; topic files frozen once
committed; deprecation, never addendum-editing of locked artifacts.

## Caps are defaults, never policy

Every fixed capacity in the engine either routes through the
`Capacity`/`Dim<N>` plan-dimension pattern (consumer-tunable at the type
level) or carries a tracked lift condition naming what unblocks it (A1-2).
Fixed caps exist only where the toolchain currently rejects the
parameterised form (`generic_const_exprs` refuses field access on generic
constants, and the trait solver overflows on config-driven sizes); they are
the proven-infeasible fallback, not an accepted end state, and the
cap-lifting arc retires them as the toolchain and the sketch-proven shapes
allow. A1 orders this work before further code accretes on fixed sizes.

## Toolchain constraint notes (elevated to canon)

Facts about building this design under the pinned nightly, carried as canon
because any rebuild meets them again:

1. Type-level N-way partition of a heterogeneous carrier requires the
   forbidden full `specialization`; the canonical mechanism is const data
   plus const-gated DCE over a flat carrier. Partition lives in const
   evaluation, never in the carrier type.
2. Type-keyed projection out of heterogeneous lists uses inferred index
   witnesses, never type-equality specialization.
3. `generic_const_exprs` ceilings: no field access on generic constants; a
   complexity limit on inline const blocks in bounds (worked around with
   associated-const carrier structs).
4. Accumulator appends saturate at reserved capacity as a soundness guard;
   capacity equals record count at build; a silently dropped append means
   check live-versus-capacity first.
5. Engine-owned mutable meta state lives in the MetaBlock ([[scheduler]]).
6. Platform tiers are os and no_os only ([[foundations]]).
7. The clock is a builder-slot provider; value-carrying providers need
   dedicated slots.

Unstable-feature policy follows the workspace vetting regime: full
`specialization` is forbidden (structurally unsound); the founding spec's
nightly-gate list is superseded by the vetted per-feature tables in the
workspace rules, and additions go through the vetting procedure.

## The auto-vectorisation contract

The design eliminates six of the eight vectorisation killers by
construction: opaque calls (no-dyn monomorphisation), heap allocation
(no-alloc plus the 16-byte value cap), non-contiguous access (columnar
storage), interior mutability and non-Copy types (the Copy bound),
misaligned data (64-byte alignment). Loop-carried dependencies are partially
eliminated by the separate Read/Write declarations; complex control flow
remains the op author's domain. The design is the SIMD contract: no explicit
intrinsics in the common case, LLVM auto-vectorises when the killers are
absent.

## Enforcement lints

The founding lint set, enforced through the workspace lint machinery:
no-raw-primitives in data-facing positions, no-dyn, no-std-any,
arvo-types-only in our own data-facing structs, no-unwrap outside tests,
plus the stack-wide primitive-vocabulary and import-boundary lints the
workspace defines per crate. Lint severities never go down; a blocking gate
is the design.

## Quality regime

Red tests are the measurement, not the problem: tests encode the full
canonical design, parts not yet built stay red, and nothing is stopgapped
green (the strict-by-design rule). Edge cases become tests the moment they
are found. Deviations from canon are recorded the moment they ship, each
with disposition and trigger, and resolve through evidence-then-bless
([[governance]]). Algorithm and performance forks are benched, never argued;
bench artifacts commit alongside the decision they decide.
