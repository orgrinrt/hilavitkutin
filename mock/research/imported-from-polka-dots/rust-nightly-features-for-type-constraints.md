# Rust Nightly Features for Type-Level Layout Constraints

**Date:** 2026-03-13
**Purpose:** Research into Rust unstable features that enable
compile-time layout enforcement through the trait system — specifically
for the column value constraint problem in polka-dots' columnar
execution engine. Also serves as a full inventory of the 2026 Rust
project goals for design awareness.

**Problem statement:** Column values in our 128-bit stride model must
satisfy: `size ≤ 15 bytes`, `alignment divides 16`, `no internal padding`,
`repr(C)`. These must be **hard trait-level contracts**, not const-asserts
in derive macros or convention. The stable Rust type system cannot
express `where { size_of::<Self>() <= 15 }` in trait bounds.

**Features covered in depth:**
- `min_specialization` — restricted trait specialization
- `min_generic_const_args` — const expressions in generic parameters
- `const_trait_impl` — traits callable in const contexts
- `adt_const_params` — ADT types as const generic parameters
- `generic_const_exprs` — arbitrary const expressions in generics (dead)
- `unsized_const_params` — unsized types as const parameters

**Additional features of interest:**
- `reflection-and-comptime` — compile-time type reflection
- `tail-call-loop-match` — explicit tail calls and `loop_match`
- `parallel-front-end` — parallel rustc front end
- `build-std` — rebuild stdlib from source

---

## Table of Contents

1. [The Constraint Problem](#1-the-constraint-problem)
2. [Feature Survey](#2-feature-survey)
   2.1 `min_specialization`
   2.2 `min_generic_const_args`
   2.3 `const_trait_impl`
   2.4 `adt_const_params`
   2.5 `generic_const_exprs`
   2.6 `unsized_const_params`
3. [Additional Features of Interest](#3-additional-features-of-interest)
   3.1 `reflection-and-comptime`
   3.2 `tail-call-loop-match`
   3.3 `parallel-front-end`
   3.4 `build-std`
4. [Stable Rust Limitations](#4-stable-rust-limitations)
5. [Which Features Solve What](#5-which-features-solve-what)
6. [Risk Assessment](#6-risk-assessment)
7. [Adoption Recommendation](#7-adoption-recommendation)

---

## 1. The Constraint Problem

### 1.1 The column value layout

Our columnar execution model uses a fixed 128-bit (16-byte) stride per
column entry: 1 byte flags + 15 bytes payload. All columns are
contiguous arrays for cache-line-aligned access and LLVM
autovectorisation. Column values are always domain newtypes (e.g.,
`VersionStr`, `OurBool`), never raw primitives.

The invariants that must hold for every column value type:

```
size_of::<T>()  <= 15        // fits in payload slot
align_of::<T>() divides 16   // no misalignment in stride
no internal padding           // pack/unpack is bit-exact
repr(C)                       // deterministic layout
```

### 1.2 Why const-asserts are insufficient

We already use `const _: () = assert!(size_of::<T>() <= 15)` as a
guard, but this is a convention — nothing in the trait system forces an
implementor to include it. A derive macro can automate the assert, but
the macro is the enforcement mechanism, not the type system. If someone
implements the trait manually (or the macro has a bug), the invariant
is unguarded.

The design principle: **traits are the enforcement mechanism**. If a
constraint can't be expressed as a trait bound, the design is
incomplete.

### 1.3 What we need from the language

The ideal expression:

```rust
trait ColumnValue: Copy + 'static
where
    Check<{ size_of::<Self>() <= 15 }>: IsTrue,
    Check<{ 16 % align_of::<Self>() == 0 }>: IsTrue,
{
    // ...
}
```

This requires const expressions involving `Self` in where-clause
positions — which stable Rust cannot do. The `size_of::<Self>()`
call is not a valid const generic argument on stable because `Self`
is not a concrete type at definition time.

### 1.4 The sealed-trait workaround

An alternative explored: sealed `SlotSize` trait with fixed
implementations (`Slot1`, `Slot2`, `Slot4`, `Slot8`, `Slot12`,
`Slot15`), where column values declare `type Slot: SlotSize`. This
moves the constraint into the type system via a finite set of valid
sizes.

**Verdict:** Right principle (sealed trait set constrains valid
values), wrong abstraction (the slot is a layout detail, not a
semantic property of the value type). The pack/unpack ceremony adds
noise to every column value definition. May still be a fallback if
nightly features prove too unstable.

---

## 2. Feature Survey

### 2.1 `min_specialization`

**Feature gate:** `#![feature(min_specialization)]`
**Tracking issue:** rust-lang/rust#31844
**2026 project goal:** Yes — `specialization.md` proposes stabilising
a "concrete type specialisation" subset following the "always
applicable" rule.

#### What it does

Allows a more specific `impl` to override a less specific one when
both apply. Restricted to cases where the specialising impl is
"always applicable" — meaning it doesn't add bounds that could fail.
This prevents the unsoundness of full `specialization`.

```rust
trait Dispatch {
    fn method(&self) -> &'static str { "default" }
}

impl<T> Dispatch for T { }          // blanket default
impl Dispatch for ConcreteType {    // specialised override
    fn method(&self) -> &'static str { "concrete" }
}
```

#### Current status

- **Nightly:** Fully usable. Has been on nightly for years.
- **Stdlib usage:** Yes — libstd and libproc_macro use it internally.
  They switched from full `specialization` to `min_specialization`
  (PR #68970) specifically because the full version is unsound.
- **Known soundness issue:** The `#[rustc_unsafe_specialization_marker]`
  attribute (used for auto traits like `Send`) has a known lifetime-
  related soundness hole (#79457). However, this attribute is
  `rustc_`-prefixed and not available to user code without
  `#[allow(internal_features)]`.
- **Stabilisation outlook:** The 2026 goal proposes a further
  restricted "always applicable" subset with formal modeling in
  a-mir-formality. Stabilisation depends on the next-generation
  trait solver. No concrete date but active work.

#### Relevance to our problem

Not a direct solve for layout constraints, but essential for our
trait-heavy architecture:

- Default trait method implementations that concrete types can override
- Blanket impls with specialised paths for known types
- Enables the "layered default + override" pattern across our SDK
  trait hierarchy

#### Risk for us

**Low.** Our column value types are `Copy + 'static` with no lifetime
parameters, so the marker trait soundness hole is irrelevant. The
feature is battle-tested in the stdlib. If it regresses on nightly,
it would break the stdlib itself — rapid fix is guaranteed.

---

### 2.2 `min_generic_const_args`

**Feature gate:** `#![feature(min_generic_const_args)]`
**Tracking issue:** rust-lang/rust#132980
**2026 project goal:** Yes — part of the `const-generics` goal.
Stabilisation is a listed milestone.

#### What it does

Allows const expressions (not just literals or freestanding const
items) as generic arguments. This is the successor to the dead
`generic_const_exprs` — deliberately scoped to a minimal, sound
subset.

The key enabler for us: const expressions involving type parameters
can appear in generic positions, including where-clause bounds.

```rust
struct Check<const B: bool>;
trait IsTrue {}
impl IsTrue for Check<true> {}

// This is the pattern we need:
fn foo<T>() where Check<{ size_of::<T>() <= 15 }>: IsTrue { ... }
```

#### Current status

- **Nightly:** Usable. Implementation is 11/18 sub-tasks complete.
- **What works:** Path-based const args, simple const expressions,
  associated const equality bounds.
- **What doesn't yet:** Struct construction expressions with rest
  patterns, some CTFE edge cases with erased lifetimes in ValTree.
- **Soundness:** No known soundness issues flagged.
- **Stdlib usage:** Not yet.
- **Stabilisation outlook:** Active development, prototype landed.
  2026 project goal targets stabilisation. Months away, not years.

#### Relevance to our problem

**This is the direct solve.** With `min_generic_const_args`, we can
write:

```rust
trait ColumnValue: Copy + Sized + 'static
where
    Check<{ size_of::<Self>() <= 15 }>: IsTrue,
    Check<{ 16 % align_of::<Self>() == 0 }>: IsTrue,
{
    // ...
}
```

Any type that doesn't satisfy these constraints simply cannot implement
the trait — the where clause fails at monomorphisation. This is a
**hard contract in the trait definition itself**, not a convention or
macro-generated assert.

#### Risk for us

**Medium.** The feature is actively developed and a 2026 goal, but
only 61% complete. The specific patterns we need (const expressions
with `size_of`/`align_of` and comparison operators in where clauses)
are among the simpler cases — they don't involve struct construction
or lifetime erasure, which are the incomplete parts. Still, nightly
features can regress between toolchain versions.

**Mitigation:** We can keep const-assert fallbacks alongside the
trait bounds. If a nightly regression breaks the where-clause form,
the const-asserts still catch violations — we just temporarily lose
the trait-level guarantee until the regression is fixed.

---

### 2.3 `const_trait_impl` (const traits)

**Feature gate:** `#![feature(const_trait_impl)]`
**Tracking issue:** rust-lang/rust#143874 (new, supersedes #67792)
**2026 project goal:** Yes — dedicated `const-traits` goal.

#### What it does

Allows trait methods to be callable in const contexts. The syntax
recently changed from `#[const_trait] trait Foo` to
`const trait Foo`.

```rust
const trait Add {
    fn add(self, rhs: Self) -> Self;
}

const fn double<T: ~const Add + Copy>(x: T) -> T {
    x.add(x)
}
```

The `~const` bound means "const if the caller is const, runtime
otherwise" — enabling traits that work in both contexts.

#### Current status

- **Nightly:** Usable but experimental. Implementation was
  **rewritten from scratch** with new syntax (PR #143879).
- **Soundness:** No specific soundness bugs, but design is still
  being finalised.
- **Known blockers:** `#[derive_const]` for proc macros (#118304),
  performance stress testing, RFC not yet merged, style guide
  decisions pending.
- **Stdlib usage:** Previously used in libcore but **temporarily
  removed** during the rewrite.
- **Old tracking issue:** Was labeled `S-tracking-perma-unstable`
  and closed. The new issue represents a fresh start.
- **Stabilisation outlook:** Multiple release cycles away. The 2026
  goal targets stabilisation but this is ambitious given the rewrite.

#### Relevance to our problem

Not a direct solve for layout constraints, but valuable for:

- Making `pack`/`unpack` operations on column values callable at
  compile time
- Enabling const evaluation of trait-based column metadata
- Const-evaluable trait methods for type registration and schema
  introspection

#### Risk for us

**Medium-high.** The implementation was just rewritten, meaning it's
less battle-tested than `min_specialization`. The old version was
removed from stdlib. Syntax just changed. However, the patterns we'd
use (simple const trait methods on `Copy` types) are the simplest
case — we're not pushing boundaries with complex const generics
interaction.

---

### 2.4 `adt_const_params`

**Feature gate:** `#![feature(adt_const_params)]`
**Tracking issue:** rust-lang/rust#95174
**2026 project goal:** Yes — part of `const-generics` goal.

#### What it does

Allows ADT types (structs, enums) as const generic parameters, not
just primitives (`usize`, `bool`, `char`). Types must implement
`ConstParamTy`.

```rust
#[derive(ConstParamTy, PartialEq, Eq)]
struct Layout { size: usize, align: usize }

struct Validated<const L: Layout>;
```

#### Current status

- **Nightly:** Usable. The compiler warns "incomplete and may not be
  safe to use and/or cause compiler crashes," but the originally
  cited ICEs (#131052, #129095, #128232) are **all fixed**.
- **11 open ICEs remain** — mostly triggered by **feature
  combinations**, not `adt_const_params` alone.
- **Symbol mangling:** Unresolved. RFC 3161 proposes a structural
  mangling scheme but is not yet merged. This is a **hard blocker
  for stabilisation** — symbol names for generic functions
  instantiated with ADT const values are not stable across compiler
  versions.
- **RFC:** Not yet published for the feature itself.
- **Stabilisation outlook:** Blocked on RFC publication, symbol
  mangling finalisation, and ICE fixes. 2026 goal but significant
  work remains.

#### Known ICEs — detailed analysis

The three originally cited ICEs are **all closed/fixed**:

| Issue | Trigger | Root cause | Status |
|-------|---------|-----------|--------|
| #131052 | `&'static [*mut u8; 3]` as const param | Missing early rejection of non-valtree-compatible types | **Fixed** |
| #129095 | Byte literal length > array size | `unwrap()` on out-of-bounds index in valtree construction | **Fixed** |
| #128232 | `fn() -> u32` as const param | Function pointers not valtree-compatible, not rejected early | **Fixed** |

**11 open ICEs** — the patterns to avoid:

| Issue | Trigger pattern | Key interaction |
|-------|----------------|-----------------|
| #153733 | async fn + pin_ergonomics | `min_generic_const_args` + `adt_const_params` + `pin_ergonomics` |
| #153524 | `const A: [u8; N]` with inference | `generic_const_parameter_types` + `min_generic_const_args` |
| #152891 | `&'static [&'static str]` with lifetime params | `unsized_const_params` interaction |
| #151511 | Rustdoc + assoc const equality from supertrait | `min_generic_const_args` + `unsized_const_params` + `generic_const_parameter_types` |
| #151112 | Dependent const param types: `const M: Inner<N>` | `generic_const_parameter_types` |
| #151079 | `-Zmir-opt-level=5` | `min_generic_const_args` under aggressive MIR opts |
| #143117 | Same as #151112 | `generic_const_parameter_types` |
| #129556 | Supertrait with shared const generic default | `generic_const_exprs` interaction |
| #126123 | Complex const fn with arrays/matrices | `generic_const_exprs` interaction |
| #125564 | `&'static str` + incremental + `where [(); A.len()]` | `generic_const_exprs` + incremental compilation |

**Key patterns:**
1. Most open ICEs involve `generic_const_parameter_types` or
   `generic_const_exprs` — features we are NOT using.
2. Dependent const param types (`const A: [u8; N]` where `N` is
   another const param) are a consistent crash source.
3. `-Zmir-opt-level=5` triggers valtree bounds checks — use default
   MIR opt levels.
4. `unsized_const_params` interactions are unstable — don't combine.

**Safe usage rules for our codebase:**
- Use only types that implement `ConstParamTy` (integers, `bool`,
  `char`, simple structs/enums deriving `ConstParamTy`)
- No raw pointers or function pointers as const params
- No dependent const param types (`const A: Foo<N>`)
- No combination with `generic_const_exprs` or
  `generic_const_parameter_types`
- No `unsized_const_params` interaction
- Default MIR optimisation levels only
- No async fn in const-param-bearing contexts

#### Relevance to our problem

Enables richer const generic types. Where `min_generic_const_args`
gives us `Check<{ expr }>: IsTrue` with bool, `adt_const_params`
lets us encode layout metadata as structured const params:

```rust
#[derive(ConstParamTy, PartialEq, Eq)]
struct ColumnLayout {
    size: usize,
    align: usize,
    stride: usize,
}

// Column definition carries its layout as a const param
struct TypedColumn<T, const LAYOUT: ColumnLayout> { ... }
```

This also solves headaches from earlier design phases where we needed
to thread layout information through the type system but were limited
to integer const generics.

#### Interaction with `min_generic_const_args`

These two features are **complementary**:
- `adt_const_params` expands **which types** can be const parameters
- `min_generic_const_args` expands **which expressions** can appear
  as const arguments

mGCA explicitly lists supporting struct construction expressions as
a step — needed to construct ADT const param values in generic
position. Some ICEs exist at the intersection (#153733, #153524,
#151079), but these are in corner cases (async, pin, aggressive MIR
opts) that don't overlap with our usage.

#### Risk for us

**Medium.** The originally cited ICEs are fixed. The remaining ICEs
are in feature combinations we won't use. Symbol mangling is
unresolved but only matters for cross-crate linking stability — we
pin our toolchain so symbol names are consistent within a build. The
real risk is nightly regression, mitigated by toolchain pinning and
the safe usage rules above.

---

### 2.5 `generic_const_exprs`

**Feature gate:** `#![feature(generic_const_exprs)]`
**Tracking issue:** rust-lang/rust#76560
**2026 project goal:** **No.**

#### What it does (theoretically)

Allows arbitrary const expressions in generic positions:

```rust
fn foo<const N: usize>() -> [u8; N + 1] { ... }
```

#### Current status

- **Nightly:** Usable but labeled `S-tracking-impl-incomplete`.
  "Still far from ready."
- **Dead path:** Not a 2026 project goal. No active champion.
  Effectively superseded by `min_generic_const_args`, which
  deliberately restricts scope to avoid the soundness problems
  that made this feature intractable.
- **Soundness:** Open design questions collected in
  project-const-generics repo.

#### Relevance to our problem

None. Use `min_generic_const_args` instead.

#### Risk for us

**Do not use.** No stabilisation path, no active development, known
to be superseded. Also a trigger for multiple `adt_const_params` ICEs
when combined — another reason to avoid.

---

### 2.6 `unsized_const_params`

**Feature gate:** `#![feature(unsized_const_params)]`
**Tracking issue:** rust-lang/rust#128028

#### What it does

Extends const generics to unsized types (e.g., `const S: str`).
Split out from `adt_const_params`.

#### Current status

- Under active development but far from stabilisation.
- No RFC written.
- Open questions about pointer identity and padding.

#### Relevance to our problem

None. Our column values are all `Sized + Copy`.

---

## 3. Additional Features of Interest

These are not directly related to the column value constraint problem
but are useful for the broader architecture if we're designing with
nightly in mind.

### 3.1 `reflection-and-comptime`

**2026 project goal:** Yes — `reflection-and-comptime.md`.
**Feature gate:** Not yet defined (architectural groundwork phase).

#### What it does

Compile-time reflection and comptime evaluation — the ability to
inspect type information (fields, variants, attributes) at compile
time via const fn. Think `comptime` from Zig but within Rust's
type system.

**Important caveat:** The goal explicitly excludes putting type info
back INTO the type system (no dependent types). It's read-only
introspection for codegen purposes.

#### Relevance to us

**High for schema introspection.** Our schema-as-traits model
(`Column<In<IR>, As<T>>`) requires trait-level metadata about column
types — field names, types, layout. Currently this metadata must be
manually declared or generated by proc macros. Comptime reflection
would let us derive it automatically:

```rust
// Hypothetical — derive column metadata from struct fields
const fn column_count<T: Reflect>() -> usize {
    T::fields().len()
}
```

Also relevant for:
- Automatic `ColumnSlices`/`ColumnSlicesMut` tuple generation
- WorkUnit read/write type validation
- Schema compatibility checking at compile time

#### Status

Architectural groundwork. No implementation yet. The 2026 goal is
about laying foundations, not shipping a usable feature. Design with
it in mind but don't depend on it.

---

### 3.2 `tail-call-loop-match`

**2026 project goal:** Yes — `tail-call-loop-match.md`.
**Feature gate:** `#![feature(explicit_tail_calls)]` (become keyword),
`#![feature(loop_match)]` (loop_match construct).

#### What it does

Two related features:

1. **Explicit tail calls** (`become`): Guarantees tail call
   elimination — the caller's stack frame is reused.

2. **`loop_match`**: A structured control flow construct for state
   machines that compiles to a jump table rather than a loop+match:

```rust
loop_match {
    State::Init => {
        // ...
        continue State::Running;
    }
    State::Running => {
        // ...
        continue State::Done;
    }
    State::Done => {
        break result;
    }
}
```

#### Relevance to us

**Directly useful for morsel processing inner loops.** Our fused
chain execution is fundamentally a state machine: each operator in
the chain processes a morsel and transitions to the next. Currently
this would be a loop with match dispatch. `loop_match` would let the
compiler generate a direct jump table — eliminating branch
misprediction overhead in the hot path.

Also useful for:
- Pipeline stage dispatch (build → stage → deploy)
- DSL expander state machine
- WorkUnit chain traversal

The `become` keyword is less immediately useful (we're not doing
deep recursion in hot paths) but good to have for recursive AST
traversal in the DSL.

#### Status

`explicit_tail_calls` has been on nightly for a while.
`loop_match` is newer. Both are 2026 goals. The inner-loop benefit
is real but not critical — branch prediction on modern CPUs is good
enough for our data volumes. Design with it in mind, adopt when
stable enough.

---

### 3.3 `parallel-front-end`

**2026 project goal:** Yes — `parallel-front-end.md`.
**Feature gate:** `-Zthreads=N` (compiler flag, not a code feature).

#### What it does

Parallelises the rustc front end (parsing, name resolution, macro
expansion, type checking) across multiple threads.

#### Relevance to us

**Compilation speed for trait-heavy codebases.** Our architecture
uses extensive generics, trait bounds, and monomorphisation. The
compiler spends significant time in type checking and trait
resolution. Parallel front end would reduce compile times,
especially for the mock workspace with 17 crates.

#### Status

Already functional on nightly behind `-Zthreads`. The 2026 goal is
about promoting it to stable. No code changes needed on our side —
just a compiler flag.

**Adopt immediately** via `RUSTFLAGS` or `.cargo/config.toml`:

```toml
# .cargo/config.toml
[build]
rustflags = ["-Zthreads=8"]
```

---

### 3.4 `build-std`

**2026 project goal:** Yes — `build-std.md`.
**Feature gate:** `-Zbuild-std` (cargo flag).

#### What it does

Rebuilds the standard library from source with the same configuration
as your crate. This means:

- Your target features apply to std (SIMD, CPU-specific opts)
- LTO can inline across the std boundary
- Unused std components are eliminated

#### Relevance to us

**Directly useful once we're on nightly.** Benefits:

1. **LTO across std:** Our hot inner loops call `core::mem::size_of`,
   `core::mem::align_of`, `core::slice` methods. With build-std,
   these are subject to the same LTO pass as our code — enabling
   cross-boundary inlining and dead code elimination.

2. **Target CPU features:** We can rebuild std with
   `-C target-cpu=native`, ensuring SIMD intrinsics in std match
   our hardware.

3. **Smaller binaries:** Unused std components (networking, I/O for
   library crates) are eliminated.

4. **Our nightly features propagate into std:** If we enable
   `min_specialization` etc., building std from source ensures
   consistency.

#### Status

Functional on nightly behind `-Zbuild-std`. The 2026 goal is about
stabilisation. Requires `rust-src` component installed.

```toml
# .cargo/config.toml
[unstable]
build-std = ["core", "alloc", "std"]
build-std-features = ["panic_immediate_abort"]  # optional, smaller binary
```

---

## 4. Stable Rust Limitations

### 4.1 What stable CAN do

- `const fn` — including `size_of::<T>()`, `align_of::<T>()` for
  concrete types
- `const _: () = assert!(...)` — eagerly evaluated freestanding
  const assertions
- Sealed traits with finite implementation sets
- Associated consts in traits
- Const generic parameters: `bool`, `usize`, `char`, integer types

### 4.2 What stable CANNOT do

- `size_of::<Self>()` in where-clause const expressions — `Self` is
  not concrete at trait definition time
- Const expressions as generic arguments (only literals and const
  items)
- Specialisation of any kind
- Calling trait methods in const contexts
- Using where clauses to assert properties of `Self` that depend on
  layout
- ADT types as const generic parameters

### 4.3 The fundamental gap

The stable type system has no way to say "this trait can only be
implemented by types with size ≤ 15". You can approximate with:

- **Sealed traits:** Enumerate valid types, but this is a closed set
- **Const asserts in default methods:** Only fire if the method is
  called, which is not guaranteed
- **Derive macros:** Generate const asserts, but the macro is the
  enforcement, not the trait

None of these are hard contracts in the trait definition itself.

---

## 5. Which Features Solve What

### 5.1 Layout constraint enforcement (the primary problem)

**`min_generic_const_args`** is the direct and complete solution:

```rust
#![feature(min_generic_const_args)]

struct Check<const B: bool>;
trait IsTrue {}
impl IsTrue for Check<true> {}

trait ColumnValue: Copy + 'static
where
    Check<{ core::mem::size_of::<Self>() <= 15 }>: IsTrue,
    Check<{ 16 % core::mem::align_of::<Self>() == 0 }>: IsTrue,
{
    // Column value contract — any impl that doesn't satisfy
    // the size/alignment constraints fails at monomorphisation.
}
```

**Fallback if unavailable:** Sealed `SlotSize` trait with finite
implementations. Works on stable, but wrong abstraction level.

### 5.2 Structured layout metadata

**`adt_const_params`** enables encoding layout as typed const params:

```rust
#[derive(ConstParamTy, PartialEq, Eq)]
struct ColumnLayout {
    size: usize,
    align: usize,
    stride: usize,
}

struct TypedColumn<T, const LAYOUT: ColumnLayout> { ... }
```

This replaces scattered `usize` const params with a single structured
type, and also solves the problem of threading layout information
through generic contexts that we hit in earlier design phases.

### 5.3 Trait hierarchy flexibility

**`min_specialization`** enables:

- Default implementations that specific types can override
- Blanket impls with specialised fast paths
- Layered SDK trait design without diamond problems

### 5.4 Compile-time trait evaluation

**`const_trait_impl`** enables:

- Const-evaluable `pack`/`unpack` for column values
- Compile-time schema introspection
- Const-time column metadata computation

### 5.5 Inner loop optimisation

**`tail-call-loop-match`** enables:

- Jump-table dispatch for fused chain execution
- Branch-prediction-friendly state machines

### 5.6 Build infrastructure

**`parallel-front-end`** + **`build-std`** enable:

- Faster compilation of our trait-heavy codebase
- LTO across the std boundary for hot inner loops
- Target-CPU-optimised stdlib

### 5.7 Feature interaction map

```
min_generic_const_args  → enforces WHAT types are valid (layout bounds)
adt_const_params        → structures HOW layout metadata is carried
min_specialization      → controls HOW default behaviour layers
const_trait_impl        → enables WHERE trait methods can run (const)
tail-call/loop_match    → optimises HOW inner loops dispatch
parallel-front-end      → speeds up compilation
build-std               → enables cross-boundary LTO
reflection-and-comptime → future: automatic schema introspection
```

---

## 6. Risk Assessment

### 6.1 Risk matrix

| Feature | Breakage risk | Soundness risk | Removal risk | Recovery cost |
|---------|--------------|----------------|-------------|---------------|
| `min_specialization` | Very low (stdlib) | Low (`Copy + 'static`) | Near zero | N/A |
| `min_generic_const_args` | Medium (61% done) | Low (none known) | Low (2026 goal) | Fall back to const-asserts + sealed traits |
| `const_trait_impl` | Medium-high (rewritten) | Low | Low (2026 goal) | Remove `const` from trait defs |
| `adt_const_params` | Medium (ICEs in combos) | Low (if rules followed) | Low (2026 goal) | Fall back to `usize` const params |
| `parallel-front-end` | None (compiler flag) | N/A | Low | Remove flag |
| `build-std` | Low (cargo flag) | N/A | Low | Remove flag |
| `tail-call-loop-match` | Low (opt-in syntax) | Low | Low (2026 goal) | Rewrite as loop+match |
| `reflection-and-comptime` | N/A (not ready) | N/A | N/A | N/A |

### 6.2 `adt_const_params` — controlled adoption

The open ICEs are **all in feature combinations we don't use**. Safe
adoption requires following these rules:

**DO:**
- Use only types deriving `ConstParamTy` (simple structs/enums with
  integer/bool/char fields)
- Stick to `min_generic_const_args` for const expressions, not
  `generic_const_exprs`
- Pin nightly toolchain
- Use default MIR optimisation levels

**DO NOT:**
- Use raw pointers or function pointers as const params
- Use dependent const param types (`const A: Foo<N>`)
- Combine with `generic_const_exprs`
- Combine with `unsized_const_params`
- Combine with `generic_const_parameter_types`
- Use in async fn contexts with `pin_ergonomics`
- Use `-Zmir-opt-level=5`

With these rules, the feature is usable. The symbol mangling blocker
only affects cross-toolchain linking stability — we pin our toolchain,
so symbols are consistent within a build.

### 6.3 Nightly stability considerations

- **Pinning toolchain:** Use `rust-toolchain.toml` to pin a known-good
  nightly. Only upgrade after testing.
- **CI matrix:** Test against pinned nightly + latest nightly to catch
  regressions early.
- **Feature isolation:** Gate nightly features behind `cfg` if
  possible, though for trait definitions this is impractical.

### 6.4 The "stdlib anchor" principle

`min_specialization` is the safest nightly feature because the Rust
stdlib itself depends on it. Any nightly that breaks it breaks the
entire toolchain — such regressions are caught and fixed within hours.
The other features lack this anchor but are 2026 project goals with
active teams.

---

## 7. Adoption Recommendation

### 7.1 Adopt now — core features

| Feature | Gate | Reason |
|---------|------|--------|
| `min_specialization` | `#![feature(min_specialization)]` | Stdlib-anchored, essential for trait hierarchy, `Copy + 'static` types avoid the soundness hole |
| `min_generic_const_args` | `#![feature(min_generic_const_args)]` | Direct solve for layout constraints, 2026 goal, no soundness issues |
| `const_trait_impl` | `#![feature(const_trait_impl)]` | Const-evaluable trait methods, essential for compile-time schema work |
| `adt_const_params` | `#![feature(adt_const_params)]` | Structured const generics, 2026 goal, original ICEs fixed, controlled adoption with documented rules |

### 7.2 Adopt now — build infrastructure

| Feature | Configuration | Reason |
|---------|--------------|--------|
| `parallel-front-end` | `-Zthreads=8` | Free compilation speedup, no code changes |
| `build-std` | `-Zbuild-std` | LTO across std, target-CPU opts |

### 7.3 Design with in mind, adopt when ready

| Feature | Reason |
|---------|--------|
| `tail-call-loop-match` | Inner loop dispatch optimisation, adopt when `loop_match` is stable enough for hot paths |
| `reflection-and-comptime` | Future schema introspection, architectural groundwork only in 2026 |

### 7.4 Do not adopt

| Feature | Reason |
|---------|--------|
| `generic_const_exprs` | Dead path, superseded, triggers ICEs in combination with features we use |
| `unsized_const_params` | Irrelevant (`Sized + Copy` types only), triggers ICEs with `adt_const_params` |
| `generic_const_parameter_types` | Triggers ICEs with `adt_const_params`, not needed |

### 7.5 Operational requirements

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly"
components = ["rust-src"]  # needed for build-std
# Pin to a known-good date after testing:
# channel = "nightly-2026-03-15"
```

```toml
# .cargo/config.toml
[build]
rustflags = ["-Zthreads=8"]

[unstable]
build-std = ["core", "alloc", "std"]
```

```rust
// lib.rs or crate root
#![feature(min_specialization)]
#![feature(min_generic_const_args)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]
```

### 7.6 Multi-scale design note

These features serve all three target scales:

- **polka-dots (KB):** Trait-enforced column layout, specialised
  dispatch for small datasets, structured layout metadata
- **saalis (200K-1M rows):** Same trait contracts, specialisation
  for batch-optimised paths, const-evaluable schema introspection
- **ECS game engine (10M entities, 60fps):** Compile-time layout
  guarantees, zero-cost abstractions, jump-table dispatch for
  entity processing, LTO across std for hot paths

The nightly features don't just solve today's problem — they're
foundational to the trait-driven architecture across all scales.

---

## Appendix A: Complete 2026 Rust Project Goals

All 66 goals (status: Proposed — RFC not yet accepted). 7 thematic
roadmaps group related goals.

### Roadmaps

| Roadmap | Theme |
|---------|-------|
| Beyond the `&` | Ownership and borrowing ergonomics beyond references |
| The Borrow Checker Within | Internal borrow checker improvements |
| Constify all the things | Expanding const evaluation capabilities |
| Just add async | Async Rust improvements |
| Project Zero | Reducing compilation time and improving tooling |
| Rust for Linux | Supporting Rust in the Linux kernel |
| Safety-Critical Rust | Making Rust viable for safety-critical domains |

### All 66 goals — sorted by composite relevance

Scored on two axes, then combined:

- **Us (0-5):** How directly useful this is to polka-dots, saalis,
  and the ECS engine — considering our trait-heavy architecture,
  columnar execution model, plugin system, and DSL.
- **General (0-5):** How broadly impactful this is for the Rust
  ecosystem — language expressiveness, developer experience,
  compilation speed, safety tooling.
- **Combined:** `(Us × 2 + General) / 3` — weighted toward our
  needs but tempered by general maturity signal. A feature that's
  critical for us AND broadly important is more likely to be
  maintained, stabilised, and well-tested than one that only we care
  about.

| Rank | Goal | Description | Us | General | Combined | Notes |
|------|------|-------------|:--:|:-------:|:--------:|-------|
| 1 | const-generics | `min_generic_const_args`, `adt_const_params`, `generic_arg_infer` | 5 | 5 | **5.0** | Direct solve for layout constraints. Universally requested. |
| 2 | specialization | Concrete type specialisation (sound subset) | 5 | 5 | **5.0** | Foundation of our trait hierarchy. One of Rust's most-wanted features. |
| 3 | next-solver | Next-generation trait solver | 5 | 5 | **5.0** | Prerequisite for safe specialisation and complex trait resolution. Unlocks everything. |
| 4 | const-traits | Stabilise const trait methods and bounds | 5 | 4 | **4.7** | Const pack/unpack, compile-time schema. Enables `const fn` to call trait methods. |
| 5 | parallel-front-end | Parallel rustc front end | 4 | 5 | **4.3** | Immediate build speed win. Benefits every Rust project. |
| 6 | build-std | Stabilise `-Zbuild-std` | 4 | 4 | **4.0** | LTO across std, target-CPU opts. Important for embedded and perf-critical work. |
| 7 | reflection-and-comptime | Compile-time reflection and comptime | 5 | 4 | **4.7** | Future schema introspection, but groundwork-only in 2026. Score reflects potential. |
| 8 | tail-call-loop-match | Explicit tail calls and `loop_match` | 4 | 4 | **4.0** | Jump-table dispatch for inner loops. Useful for interpreters, state machines, compilers. |
| 9 | scalable-vectors | Sized hierarchy for scalable/SIMD vectors | 4 | 4 | **4.0** | SIMD types for morsel processing. Critical for ARM SVE and RISC-V V. |
| 10 | a-mir-formality | Formal Rust type system model | 4 | 4 | **4.0** | Validates specialisation soundness. Foundation for proving type system properties. |
| 11 | mir-move-elimination | Eliminate unnecessary MIR moves | 3 | 4 | **3.3** | Better codegen in our inner loops. Broadly improves all Rust performance. |
| 12 | crate-slicing | Parallel compilation within a crate | 3 | 4 | **3.3** | Faster builds for large crates. Complements parallel-front-end. |
| 13 | incremental-system-rethought | Rebuild incremental compilation | 3 | 4 | **3.3** | Faster incremental builds. Benefits every Rust developer. |
| 14 | polonius | Polonius Alpha (next-gen borrow checker) | 3 | 4 | **3.3** | More flexible borrowing. Fixes long-standing borrow checker limitations. |
| 15 | macro-improvements | `macro_rules!` improvements | 3 | 4 | **3.3** | DSL macro expressiveness. Broadly requested for macro ergonomics. |
| 16 | supertrait-auto-impl | Auto-implement supertraits | 3 | 3 | **3.0** | Less boilerplate in our trait hierarchy. Moderate general demand. |
| 17 | cargo-semver-checks | Merge cargo-semver-checks into Cargo | 3 | 4 | **3.3** | SDK API stability enforcement. Important for library ecosystem. |
| 18 | improve-cg_clif-performance | Cranelift backend performance | 2 | 4 | **2.7** | Faster debug builds for us. Big win for dev iteration across ecosystem. |
| 19 | in-place-init | Guaranteed emplacement initialisation | 3 | 3 | **3.0** | Large struct init without copies. Important for embedded and Linux kernel. |
| 20 | expansion-time-evaluation | Compile-time eval during macro expansion | 3 | 3 | **3.0** | DSL macro improvements. Enables proc-macro-like power in `macro_rules!`. |
| 21 | stabilize-try | `Try` trait | 3 | 4 | **3.3** | Error handling in pipelines. Widely requested for `?` on custom types. |
| 22 | rtn | TAIT + Return Type Notation stabilisation | 3 | 4 | **3.3** | Trait return types. Unblocks async-in-traits and complex trait patterns. |
| 23 | pub-priv | Public/private dependencies in Cargo | 2 | 4 | **2.7** | SDK API boundaries. Important for library authors ecosystem-wide. |
| 24 | cargo-cross-workspace-cache | Share build artifacts across workspaces | 2 | 4 | **2.7** | Multi-workspace builds. Widely requested for monorepo workflows. |
| 25 | open-enums | Non-exhaustive extensible enums | 3 | 3 | **3.0** | Could affect our extension model. Useful for FFI and versioned APIs. |
| 26 | field-projections | Project wrapper-type fields to inner fields | 2 | 3 | **2.3** | Pin/MaybeUninit ergonomics. Important for async and unsafe patterns. |
| 27 | cargo-plumbing | Cargo plumbing commands for scripting | 2 | 3 | **2.3** | Build pipeline integration. Useful for CI/CD tooling. |
| 28 | stabilization-of-sanitizer-support | MemorySanitizer, ThreadSanitizer | 2 | 4 | **2.7** | Testing unsafe code. Critical for safety-critical domains. |
| 29 | dictionary-passing-style-experiment | Dictionary-passing as alt to monomorphisation | 2 | 3 | **2.3** | Monitor only — could undermine our monomorphisation strategy, or provide useful alternative. |
| 30 | unsafe-fields | `unsafe` field declarations | 2 | 3 | **2.3** | Invariant enforcement on struct fields. Useful for repr(C) types. |
| 31 | stabilize-never-type | Never type (`!`) | 2 | 4 | **2.7** | Useful for exhaustive matching. Long-awaited stabilisation. |
| 32 | arbitrary-self-types | Stabilise arbitrary self types for methods | 2 | 3 | **2.3** | Custom receiver types. Enables smart-pointer method dispatch. |
| 33 | cargo-script | Stabilise single-file cargo script | 1 | 4 | **2.0** | Not our use case, but great for scripting and prototyping ecosystem-wide. |
| 34 | cargo-lints | Stabilise Cargo's linting system | 1 | 4 | **2.0** | Not directly useful, but improves Cargo quality ecosystem-wide. |
| 35 | high-level-ml | ML optimisations in the compiler | 1 | 3 | **1.7** | Marginal compile speed for us. Interesting research direction. |
| 36 | borrowsanitizer | Runtime borrow checking sanitiser | 1 | 4 | **2.0** | Useful for testing unsafe code. Important for Rust's safety story. |
| 37 | assumptions_on_binders | Where clauses on higher-ranked binders | 2 | 2 | **2.0** | Complex trait bounds. Niche but unblocks some trait patterns. |
| 38 | pin-ergonomics | Pin ergonomics experiments | 1 | 3 | **1.7** | No Pin in our hot path. Broadly useful for async ecosystem. |
| 39 | reborrow-traits | Trait-based reborrowing | 1 | 3 | **1.7** | Smart pointer ergonomics. Moderate general demand. |
| 40 | move-trait | Immobile types, guaranteed destructors | 1 | 3 | **1.7** | Not our use case. Important for Linux kernel and embedded. |
| 41 | manually-drop-attr | `#[manually_drop]` attribute | 1 | 2 | **1.3** | Niche. Drop control for specific patterns. |
| 42 | redesigning-super-let | `super let` for temporary lifetime extension | 1 | 2 | **1.3** | Niche ergonomic improvement. |
| 43 | overloading-for-ffi | Function overloading for FFI | 1 | 2 | **1.3** | Not our use case. Important for C++ interop. |
| 44 | stabilizing-f16 | `f16` half-precision float | 1 | 3 | **1.7** | Not our use case. Important for ML and GPU compute. |
| 45 | libc-1.0 | libc crate 1.0 release | 0 | 4 | **1.3** | No direct use. Foundation crate for the ecosystem. |
| 46 | interop-problem-map | C++/Rust interop problem space | 1 | 3 | **1.7** | Not our use case. Important for industry adoption. |
| 47 | library-api-evolution | Evolving std API across editions | 0 | 3 | **1.0** | No direct use. Important for long-term API stability. |
| 48 | libtest-json | libtest JSON output | 1 | 2 | **1.3** | Marginal CI improvement. Useful for test tooling. |
| 49 | interactive-cargo-tree | TUI for dependency graph | 1 | 2 | **1.3** | Nice-to-have debugging tool. |
| 50 | safe-unsafe-for-safety-critical | Normative docs for sound unsafe | 0 | 3 | **1.0** | No direct use. Important for safety-critical Rust adoption. |
| 51 | safety-critical-lints-in-clippy | Safety-critical Clippy lints | 0 | 3 | **1.0** | No direct use. Important for automotive/aerospace Rust. |
| 52 | improve-std-unsafe | Improve unsafe docs in std | 0 | 3 | **1.0** | No direct use. Improves std quality. |
| 53 | experimental-language-specification | Language spec case study | 0 | 3 | **1.0** | No direct use. Important for language formalisation. |
| 54 | stabilize-fls-releases | FLS release cadence | 0 | 2 | **0.7** | No direct use. Matters for Ferrocene/safety-critical. |
| 55 | typesystem-docs | Type system implementation docs | 0 | 2 | **0.7** | No direct use. Helps compiler contributors. |
| 56 | mcdc-coverage-support | MC/DC coverage | 0 | 3 | **1.0** | No direct use. Required for safety-critical certification. |
| 57 | stabilize-cargo-sbom | Cargo SBOM precursor | 0 | 3 | **1.0** | No direct use. Important for supply chain security. |
| 58 | rust-for-linux-compiler-features | Linux kernel compiler features | 0 | 3 | **1.0** | No direct use. Important for Rust-in-Linux. |
| 59 | user-research-team | Dedicated user research team | 0 | 2 | **0.7** | Meta goal — no technical impact. |
| 60 | async-future-memory-optimisation | Reduce async future memory footprint | 0 | 3 | **1.0** | No async in our hot path. Important for async ecosystem. |
| 61 | async-statemachine-optimisation | Optimise async state machine codegen | 0 | 3 | **1.0** | No async in hot path. Important for async performance. |
| 62 | afidt-box | `Box` notation for dyn async fn in trait | 0 | 2 | **0.7** | dyn banned in our architecture. Useful for async trait ecosystem. |
| 63 | ergonomic-rc | Alias + move expressions for Rc | 0 | 2 | **0.7** | No Rc in hot path. Niche ergonomic. |
| 64 | aarch64_pointer_authentication_pauthtest | AArch64 pointer auth on Linux ELF | 0 | 2 | **0.7** | Platform-specific security feature. |
| 65 | wasm-components | First-class WebAssembly components | 0 | 3 | **1.0** | No WASM target. Important for web ecosystem. |
| 66 | open-namespaces | Open namespace support | 1 | 1 | **1.0** | Niche. Marginal use case for us. |

### Score distribution

| Combined score | Count | Characterisation |
|---------------|-------|------------------|
| **4.0 - 5.0** | 10 | Core to our architecture AND broadly impactful — these are the features we build on |
| **3.0 - 3.9** | 10 | Useful for us, strong ecosystem demand — design-aware, adopt opportunistically |
| **2.0 - 2.9** | 14 | Moderate utility — nice to have, don't design around |
| **1.0 - 1.9** | 18 | Peripheral — ecosystem improvements that don't affect our design |
| **0.0 - 0.9** | 14 | Irrelevant to our work — meta goals, platform-specific, or conflicts with our architecture |

### Top 10 — the features we build our architecture on

These are the goals where both our specific need and the general Rust
ecosystem converge. High combined scores mean the feature is likely to
be well-maintained, actively developed, and on a real path to
stabilisation.

1. **const-generics (5.0)** — THE solve for type-level layout
   constraints. Universally requested. Active work, 2026 target.
2. **specialization (5.0)** — Foundation of layered trait defaults.
   Most-wanted Rust feature. Depends on next-solver.
3. **next-solver (5.0)** — The keystone. Without it, specialisation
   stays unsound and complex trait resolution hits limits. Every
   trait-heavy codebase benefits.
4. **const-traits (4.7)** — Const-evaluable trait methods. Enables
   compile-time schema introspection. Strong ecosystem demand.
5. **reflection-and-comptime (4.7)** — Automatic schema metadata
   derivation. Groundwork-only in 2026 but the potential is
   transformative. Strong ecosystem demand (proc-macro replacement).
6. **parallel-front-end (4.3)** — Immediate compilation speedup.
   No code changes. Benefits everyone.
7. **build-std (4.0)** — LTO across std boundary, target-CPU opts.
   Important for performance-critical and embedded work.
8. **tail-call-loop-match (4.0)** — Jump-table state machines for
   inner loops. Broadly useful for interpreters and compilers.
9. **scalable-vectors (4.0)** — Sized hierarchy for SIMD types.
   Critical for ARM SVE, RISC-V V. Our morsel processing benefits
   directly.
10. **a-mir-formality (4.0)** — Formal type system model. Validates
    that our specialisation usage is sound. Foundation for all type
    system extensions.

## Appendix B: The `Check`/`IsTrue` pattern

The standard pattern for encoding const bool assertions as trait
bounds, usable once `min_generic_const_args` stabilises (or on
nightly now):

```rust
/// Marker type parameterised by a const bool.
struct Check<const B: bool>;

/// Implemented only for `Check<true>`.
trait IsTrue {}
impl IsTrue for Check<true> {}

/// Usage: any bound `Check<{ expr }>: IsTrue` fails compilation
/// when `expr` evaluates to `false`.
trait ColumnValue: Copy + 'static
where
    Check<{ core::mem::size_of::<Self>() <= 15 }>: IsTrue,
    Check<{ 16 % core::mem::align_of::<Self>() == 0 }>: IsTrue,
{
    // Implementation must satisfy both constraints or the
    // where clause is unsatisfiable → compile error.
}
```

This pattern converts runtime-checkable properties into compile-time
trait bounds. The `Check` struct and `IsTrue` trait are zero-sized —
they exist purely for the type checker and are erased completely.

## Appendix C: Monitoring plan

Features to watch across nightly releases:

| Feature | What to monitor | Where |
|---------|----------------|-------|
| `min_specialization` | Lifetime soundness fix (#79457), next-gen solver progress | rust-lang/rust#31844 |
| `min_generic_const_args` | Remaining 7/18 tasks, stabilisation PR | rust-lang/rust#132980 |
| `const_trait_impl` | RFC status, stdlib re-adoption, `#[derive_const]` | rust-lang/rust#143874 |
| `adt_const_params` | Symbol mangling RFC, remaining ICEs, RFC publication | rust-lang/rust#95174 |
| Next-gen solver | Prerequisite for safe specialisation | rust-lang/rust (chalk/next-solver) |
| `scalable-vectors` | Sized hierarchy — affects SIMD types for morsel processing | 2026 goal |
| `tail-call-loop-match` | `loop_match` maturity for inner loop dispatch | 2026 goal |
| `reflection-and-comptime` | Architectural progress toward schema introspection | 2026 goal |
| `dictionary-passing-style` | Potential impact on our no-dyn monomorphisation strategy | 2026 goal (experiment) |
