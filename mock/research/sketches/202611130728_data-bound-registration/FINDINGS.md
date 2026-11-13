# Sketch findings: a binding table filled from data can stand beside the typed contract, and it costs the typed contract three things

**Date:** 2026-11-13
**Outcome:** WORKS, with three limits the typed contract does not have
**Unblocks:** the choice between typed and data-bound registration that round `202611130617-h6mb` owes (its doc changelist, item C10). The cost half of the same question is the bench `registration_binding` in `mock/benches/`, and its numbers are in `mock/research/202611130728_data-bound-registration-findings.md`.

## Hypothesis

The contract registers stores and units by type, and everything the plan reads is a constant of the build. The question was what a second registration would be made of, one where columns and units are declared by values that arrive at run time, on the erased, shape-described addressing chosen for resources on 2 July (`mock/design_rounds/202607161000/202606210600_topic.hybrid-decision.md`).

The belief going in was that the column side is easy and the unit side is where it gets expensive in kind: a table can hold a pointer and a shape for a column, but a unit is code, and code does not arrive as data. What would settle it is a crate that builds without `std`, without allocation, without `dyn` and without `TypeId`, in which declared units run over declared columns and compute what the typed form computes, plus the compiler's own words wherever the typed form's guarantees stop holding.

## What was tried, in order

The crate is `src/lib.rs`, its own workspace, `no_std` outside the tests. `cargo test` runs ten unit tests and three compile-fail doctests. Two features each turn on one function that must not compile, so the refusals below can be read again with `cargo build --features <name>`.

### 1. A column table with a checked cast back

`Table<'s, CAP>` holds up to `CAP` declared columns over storage it borrows. A declaration is a `ColumnDecl`, the shape of one element (size and alignment) and the stride from one element to the next, over a byte region. `Table::column::<X>(slot)` hands out a typed view only when the slot exists, the declared shape equals the shape of `X`, and the column is packed. Every other case is a `Refused` value: `Full`, `Misaligned`, `StrideUnderSize`, `NoSuchSlot`, `Shape { declared, asked }`, `NotPacked`.

This compiled at the first attempt and needed nothing the stack forbids.

### 2. A shape match alone is not enough to hand out a typed view

A `column` that takes any `X: Copy` and checks the shape is unsound, and no test of the values would show it: `bool` has the shape of `u8`, and a `u8` column read as `bool` holds 254 bit patterns that are not a `bool`. A shape says how many bytes an element has and how they are aligned. It does not say which bit patterns are values.

So the view is bounded on `Plain`, an unsafe marker trait for types with no invalid bit pattern and no padding, implemented for the integers, the floats and arrays of them. With the bound in place `declare` is a safe function, since any bytes are plain values. Asking for a `bool` view no longer compiles (`--features refused-bool-view`):

```
error[E0277]: the trait bound `bool: Plain` is not satisfied
   --> src/lib.rs:397:28
    |
397 |     let _ = table.column::<bool>(slot);
    |                   ------   ^^^^ the trait `Plain` is not implemented for `bool`
    |                   |
    |                   required by a bound introduced by this call
    |
    = help: the following other types implement trait `Plain`:
              [X; N]
              f32
              f64
              i128
              i16
              i32
              i64
              i8
            and 7 others
note: required by a bound in `Table::<'s, CAP>::column`
   --> src/lib.rs:207:22
    |
207 |     pub fn column<X: Plain>(&self, slot: Slot) -> Result<Column<'_, X>, Refused> {
    |                      ^^^^^ required by this bound in `Table::<'s, CAP>::column`
```

The same holds for a reference, an enum, a `NonZero` and any struct with padding. This is the first limit: a data-declared column holds plain values only, where a typed column holds any `ColumnValue`.

### 3. A shape cannot tell two types of one layout apart

`f32` has the shape of `u32`, so a `u32` column is handed out as `f32`. Both are plain, so nothing is unsound, and the test `a_shape_cannot_tell_two_types_of_one_layout_apart` records it as a limit and not as a refusal. Telling them apart takes a name in the declaration that the reader also knows, which a shape does not carry. The typed contract has this for nothing, since the type is the name.

### 4. A unit declared from data is a choice among compiled bodies

A `UnitDecl` names one of the bodies the build compiled, by its place in `Bodies<CAP, N>`, and the slots it reads and writes. A body is a function pointer, `for<'t, 's> fn(&'t Table<'s, CAP>, &UnitDecl, Window) -> Result<(), Refused>`, since which declarations call it is not known until they arrive. A body resolves its columns through the table on entry and then loops over the window. `run` walks the morsels and calls the bodies phase by phase. A declaration naming a body the build does not have is `Refused::NoSuchBody`.

So data chooses and wires units, and it does not make new ones. This is the second limit, and it is the one that matters for the engine: a unit behind a pointer is opaque to the compiler at its call site, so it cannot be fused with its neighbour or inlined into the morsel loop, which is what the typed form's monomorphised dispatch is for.

### 5. One plan function serves both, and only the typed one is a constant

`phases` assigns each unit a phase from its read and write masks: a reader runs in a later phase than the writer of its column, whatever the order of declaration, and two writers of one column or a cycle are refused as a `PlanRefused` value. It is one `const fn`. A typed registration evaluates it in a constant (`TYPED_PLAN`) and a table evaluates the same function when its declarations arrive; the test `the_typed_plan_and_the_table_plan_are_one_function` holds the two to one answer.

A plan over declarations that arrive at run time cannot be a constant (`--features refused-constant-plan`):

```
error[E0435]: attempt to use a non-constant value in a constant
   --> src/lib.rs:405:56
    |
405 |     const PLAN: Result<[u8; 2], PlanRefused> = phases(&reads, &writes);
    |                                                        ^^^^^ non-constant value
    |
help: consider using `let` instead of `const`
```

This is the third limit. In the typed form two writers of one column is a build error. In the table form it is a value at run time, on the machine of whoever supplied the declarations.

### 6. Shown able to fail

The test `a_plan_that_ignores_the_edge_computes_something_else` is the standing case: both units in one phase with the reader declared first, and the output differs from the typed result.

The shape check was also broken on purpose, to compare alignment only. The suite stayed green, which was a finding about the suite: its two wrong-shape cases (`u64` and `[u8; 4]` on a `u32` column) both differ in alignment. A third case of the same alignment and another size, `[u32; 2]`, was added, and under the broken check it failed:

```
thread 'tests::a_view_of_another_shape_is_refused' (18501) panicked at src/lib.rs:572:9:
assertion `left == right` failed
  left: Some(NotPacked)
 right: Some(Shape { declared: Shape { size: 4, align: 4 }, asked: Shape { size: 8, align: 4 } })
```

The check was put back and the suite is green with the new case in it.

## The result

A registration filled from data can be built inside the stack's rules. For the three parts:

A data-declared column works and is safe to declare, on the condition that its values are plain. The cast back from the erased form is a shape comparison and a stride comparison per lookup.

A data-declared unit works as a choice among bodies compiled into the build, wired to slots by data. It runs behind a function pointer.

The plan over data-declared units is the same function the typed form would evaluate, run when the declarations arrive. Its refusals become run-time values.

What the typed contract has and the table does not: any `ColumnValue` as a column's value, the type as the column's name, units the compiler can see through, and a plan that is checked when the program is built.

The three limits do not fall on one line. The first and the third are about stores and the plan, and a registration that declares only stores from data, with units staying typed, takes the first limit and neither of the others, as long as the typed units name which slots they read. That is the middle arm of the bench (`reg_bound_columns`).

## What is NOT established

Nothing here is a cost. Whether a lookup per morsel, a body behind a pointer or a run-time stride is cheap is the bench's to say, and only for the workload it ran.

The sketch is not wired to the engine. The engine's dispatch is a walk over a type-level list of units, and its per-core programs are monomorphised. No table counterpart of that walk was attempted, and nothing here shows how data-declared units would share a frame with typed ones, how they would be split over workers, or what a plan recompute costs when declarations change between frames.

The view's `get` and `set` are unsafe, as the block form's accessors were before the revision. Whether the revision's safe exclusive writes (the branded index) extend to a data-declared column was not tried. The brand is a lifetime tied to a morsel, so nothing obvious stands in the way, and it is still untested.

Only flat record columns were declared. Record domains, the frame-buffered kind, accumulators and resources were not.

A named declaration (to tell `f32` from `u32`, or one `u32` column from another) was not built. It would need an interned name on both sides, which `hilavitkutin-sym` has.

Declarations crossing a dynamic library boundary were not tried. The 2 July resolution chose the erased addressing with that case in view, and this sketch stays inside one build.

The bench resolves bindings once per morsel. A table whose columns do not move during a frame could be resolved once per frame or once per build, which the sketch does not model either way.
