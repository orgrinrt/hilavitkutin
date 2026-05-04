---
round: 202605041228
phase: TOPIC
status: frozen
supersedes_scope_decision: 202605041228_topic_arvo_rounds_1_8_adoption.md (Decision 4)
---

# Topic: typed-const completion for PR #46 deferred sites

## Frame

The first topic in this round, `202605041228_topic_arvo_rounds_1_8_adoption.md`,
scoped the work to mechanical breakage fixes and explicitly excluded
deferred polish. That scope is too narrow. The reviewer of PR #46
flagged this drift directly: `mock/crates/hilavitkutin-api/src/builder.rs:165`
ships `const D: USize = USize(0);` because, at PR-#46-merge time, arvo
had not yet exposed `USize::ZERO`. The locked SRC CL of PR #46 (lines
158, 164, 166, 170, 294) records the same: "Use whatever `USize::ZERO`
/ `USize::ONE` / const-add facade arvo currently exposes."

Arvo Rounds 1-8 ship exactly that surface. PR #46 stubbed those sites
because the substrate was not ready; PR #46 has now merged with the
stubs in place; the substrate has now caught up. Adopting the stubs
into the typed-const surface is the proper completion of PR #46, and
this round is the natural place for it.

## What arvo now exposes

Per `arvo/mock/crates/arvo-storage/src/platform.rs` the
`impl_unsigned_integer_newtype!` macro (applied to `USize` and `Cap`)
ships:

- `impl const Identity for $outer { const ZERO: Self = ...; const ONE: Self = ...; }`
- `impl const Bounded for $outer { const MIN: ...; const MAX: ...; }`
- `impl const Add / Sub / Mul / Div / Rem / Shl / Shr / BitAnd / BitOr / BitXor / Not for $outer`
- `impl const ConstPartialEq / ConstEq / ConstBitEq / ConstOrd / ConstDefault for $outer`

`USize::ZERO`, `USize::ONE`, and `R::D + USize::ONE` are all callable
in const context once the consumer crate enables
`#![feature(const_trait_impl)]`.

## Decisions

### Decision 1: Depth impl uses Identity surface

`mock/crates/hilavitkutin-api/src/builder.rs:165` migrates from
`const D: USize = USize(0);` to `const D: USize = USize::ZERO;`.
The Identity blanket on USize is the typed-constant facade that
PR #46 anticipated.

### Decision 2: Depth recursive impl uses const-add

`mock/crates/hilavitkutin-api/src/builder.rs:168` migrates from
`const D: USize = USize(R::D.0 + 1);` to
`const D: USize = R::D + USize::ONE;`. The const Add impl on USize
provides the typed-constant arithmetic that PR #46 anticipated.

### Decision 3: Doc comment matches new shape

`mock/crates/hilavitkutin-api/src/builder.rs:102` doc comment migrates
from `<()>::D == USize(0)` to `<()>::D == USize::ZERO`, and
`<(H, R)>::D == R::D + 1` to `<(H, R)>::D == R::D + USize::ONE`.

### Decision 4: Test compile-time assertion stays raw

`mock/crates/hilavitkutin/tests/scheduler_builder.rs:467` keeps
`<Cons50 as Depth>::D.0 == 50`. Migrating to a typed `==` would need
either `USize::const_eq` callsite ergonomics or a literal-typed
`USize::const_from(50)` value to compare against. The reviewer's nit
flagged the test name as well; both are cosmetic. Out of scope for
this round; tracked as future test-side polish.

### Decision 5: hilavitkutin-api needs const_trait_impl

`mock/crates/hilavitkutin-api/src/lib.rs` adds
`#![feature(const_trait_impl)]` so that the const-Add and Identity
surface can be invoked in const-context inside `Depth` impls. This
is symmetric with the hilavitkutin-str feature gate from the
sibling topic.

### Decision 6: PR #46 src CL gets an erratum reference

The reviewer of PR #46 flagged the CL-claim drift between locked
text and landed source as nit 1. This round closes the drift by
landing the typed-const adoption the locked CL anticipated. No edit
to the locked CL itself (it stays frozen per state machine); the
new round's src CL cites it explicitly.

## Sketches

None. The migrations are mechanical: one-to-one replacement of a
construction expression with a typed-const reference, plus one
feature-attribute addition. Arvo's exposed surface is verified at
`arvo/mock/crates/arvo-storage/src/platform.rs:108-111` (Identity)
and `:148-226` (const arith).

## Cross-references

- arvo Round 1 (#42, `3ba0250`): const-trait arithmetic across the
  arvo numeric stack, ships `impl const Add` etc on USize.
- arvo Round 7 (#50, `5681663`): UFixed/IFixed Bounded blanket
  forward (sibling typed-const surface, same predicate-bundling
  pattern).
- PR #46 reviewer report nit 1: explicitly identifies the drift
  this topic closes.
- PR #46 locked SRC CL `mock/design_rounds/202605011500/202605011500_changelist.src.lock.md:158-170,294`:
  the drift origin.
- `cl-claim-sketch-discipline.md`: workspace rule. The closure of
  PR #46's deferred-but-claimed surfaces is the kind of work this
  rule tracks.
