---
round: 202605041228
phase: TOPIC
status: frozen
---

# Topic: arvo Rounds 1-8 adoption

## Frame

Arvo dev advanced 13 commits between `52261d8` (the pin hilavitkutin
carried at PR #46 merge) and `91c314e` (current arvo dev HEAD). The
window covers arvo Rounds 1-8: const-trait substrate lift, Mask<W>
chassis collapse, Bounded blanket forward, ConstHash/ConstFrom/
ConstTryFrom/ConstDeref/ConstAsRef bridges, MetaCarrier resolution,
Width const-generic lift, and audit-driven smoke tests.

Updating the hilavitkutin pin to `91c314e` exposes two consumer-side
adaptations:

1. `arvo_bitmask::Mask64` removed per `no-legacy-shims-pre-1.0.md` in
   arvo Round 3 (#44, "Mask<W> + BitMatrix<W,N> chassis collapse").
   Consumers reach for `arvo_bitmask::Mask` instead.

2. The `arvo_bits::bitfield!` macro now expands const-trait calls
   (`<u32 as BitPrim>::mask_low`, `<Bits<32> as ConstPartialEq>::const_eq`).
   Consumer crates that invoke `bitfield!` need
   `#![feature(const_trait_impl)]` to call these conditionally-const
   functions from the macro-generated `const fn` bodies.

Both are mechanical adaptations the substrate's identity demands
(toolbox-not-policer + always-optimal-internals). The work is the
pin bump plus a two-line src diff.

## Decisions

### Decision 1: pin bump scope

`cargo update` on every arvo / notko package referenced from
`mock/Cargo.toml`. `arvo`, `arvo-bits`, `arvo-bitmask`, `arvo-graph`,
`arvo-sparse`, `arvo-spectral`, `arvo-comb`, `arvo-hash`,
`arvo-refit`, `notko`. No `Cargo.toml` ref change (the branch ref
stays at `dev`); only `Cargo.lock` updates the recorded commit.

### Decision 2: Mask64 to Mask migration site

`mock/crates/hilavitkutin-api/src/id.rs:7` is the only reachable
consumer of `Mask64`. The line `use arvo_bitmask::Mask64;` becomes
`use arvo_bitmask::Mask;`. Any reference to the type in the same
file needs the migration applied at every callsite.

### Decision 3: const_trait_impl feature gate

`mock/crates/hilavitkutin-str/src/lib.rs` already declares
`#![feature(adt_const_params)]` (warning: declared-but-unused per
the cargo check output). The new `#![feature(const_trait_impl)]`
declaration is added alongside the existing feature attributes.
Other crates that invoke `bitfield!` (if any) get the same feature
declaration.

### Decision 4: round scope is mechanical adoption only

The round does not introduce new substrate behavior, does not
rename anything else, does not refactor unrelated code. The
discipline is the same as #283 (hilavitkutin arvo Round D
adoption): a single mechanical sweep matching upstream signature
changes, plus the lockfile bump that captures them.

## Sketches

None. Per `cl-claim-sketch-discipline.md`, sketches are warranted
when a design has trait-solver-cycle risk, generic-const-expr risk,
repr(transparent) layout risk, or "does rustc actually accept this"
shape. None applies here. The work is mechanical: one `use` rename
plus one feature attribute. Both validated by `cargo check
--workspace` post-apply.

## Cross-references

- arvo Round 3 (#44, `c8c77af4`): the chassis collapse that removed
  `Mask64`.
- arvo Round 1 (#42, `3ba0250`): the const-trait lift that propagates
  through `bitfield!` expansions.
- `no-legacy-shims-pre-1.0.md`: the workspace rule that authorised
  arvo's clean `Mask64` removal without a deprecation alias.
- #283 (`hilavitkutin: arvo Round D adoption`): prior-art for the
  mechanical-adoption round shape.
