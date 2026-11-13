# Findings: the frame view and the two kernel regimes as types

Outcome: WORKS. Every refusal the brief names holds as a compile error, every twin builds and runs, and the committed view is read from two scoped threads at once with no `unsafe impl`. Fixture 06's data-bound twin works too: with columns bound from data, the second writer of a column is refused at bind as a value, and a runner gets staged handles only from a checked table. The contract can rest on the data-bound form; the type-level form holds only for lists fixed in the engine's types (see the section on it). Date 2026-11-12 and 2026-11-13, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the repository's pinned `nightly-2026-05-28`, `notko-hlist` at `1a978e5c2db6c8f68bf94a6fcae266394f172b17`.

This is sketch S3 of the roadmap for building kaski's engine on the stack's contracts (job `hv-sketch-frame-view`, step HV3). It asks whether the shape the first panel settled on can be said in types: one view of the committed frame whose lifetime is the step and which every thread may read, a step kernel handed that view and its staged outputs and nothing else, and a close unit that names landed stores and never holds the view. Two kernel forms are in scope. The value kernel returns one row's values and is handed no staged handle; the block kernel is handed a staged handle per written lane, which has no read method. Stamps 7 and 8 rest on it: stamp 7 option a (authors write no unsafe and hold no slice or pointer) is contingent on S2 and S3 compiling, and stamp 8 option a (a step kernel declares what it reads, checked at compile time) wants the read list enforced.

Two crates make the split the brief asks for. `engine` stands in for hilavitkutin and holds every check; `author` stands in for a game crate and only declares lanes, a registry, kernels and a close unit. The fixtures are trybuild pairs under `author/tests/ui/`: the failing file in `fail/` with its committed `.stderr`, and its twin of the same name in `pass/`, built and run. hilavitkutin declares no canon, so nothing here was checked against canon rows; the brief, the stamps and the hilavitkutin rule on the three kinds of state are what it was read against.

## How to run it

From this directory: `cargo test`. The fixtures are `cargo test --test fixtures`; `TRYBUILD=overwrite` rewrites the diagnostics. The mask probes are `cargo test --test lengths --test lengths_128`.

## The shape

A lane is a unit struct implementing `Lane` (its item type and initial value) and one of `FrameLane` or `LandedStore`. The registry is a type alias, a `notko_hlist` list of every lane. `Frame<Reg, N>` holds N rows of every lane twice, committed and staged.

`View<'s, Reg, R>` is the committed frame for the step `'s`, restricted to the read list `R`. It is a list of shared slices and a phantom, so it is `Copy`, and `Sync` exactly when every lane's item is, which `Lane::Item` requires. Its one read is `view.get(Lane, row)`, bounded on `R: Contains<Lane>`; it has no write method. `Row` is an opaque brand built only by the engine, with no order and no integer conversion (stamp 7, F's note).

`StagedLane<'s, X>` is one write-only handle: `set(row, value)` and nothing else, neither `Copy` nor `Clone`. A block kernel receives one per member of its write list as a nested tuple and destructures it. A value kernel returns a nested tuple of values per row and the runner writes them through the handles itself.

`Landed<'c, Reg, S>` is what a close unit gets after the step lands: `get`, `set` and `rows` on the stores `S` names, which must be landed stores. There is no path from it to a view, and the view's constructor is crate-private.

The position mask: each lane's place in the registry is found by the solver (`Pick<X, P>` on a list tagged by lane, `P` inferred as `Here` or `There<..>`), read back as a count through `notko_hlist::Position<Slot>::INDEX`, and folded into `Stage::MASK: u128`, with `Stage::LEN` the list's length. The runner asserts `MASK.count_ones() == LEN` per kernel, and `A::MASK & B::MASK == 0` for two kernels of one step, in an inline `const` block in `Frame::step` and `Frame::step2`.

## Per fixture

Prediction is the brief's expected code; every code in the table's third column is what the compiler said.

| Fixture | Kernel form | Compiler said | Predicted |
|---|---|---|---|
| 01 a block kernel reads its staged handle | block | `error[E0599]: no method named `get` found for struct `StagedLane<'s, X>` in the current scope` | E0599 |
| 02 a kernel writes through the committed view | block | `error[E0599]: no method named `set` found for struct `View<'s, Reg, R>` in the current scope` | E0599 or E0596 |
| 03 a step kernel's list names a landed store | value | `error[E0277]: `Landings` is not a frame lane` | E0277 |
| 04 a kernel reads a lane absent from its list | value | `error[E0308]: mismatched types` ... `expected `Pos`, found `Vel`` | E0277 |
| 04b the same, with a two-lane read list | value | `error[E0277]: `notko_hlist::Empty` does not hold `Vel`` | E0277 |
| 05 one kernel declares one staged lane twice | block | `error[E0080]: evaluation panicked: a kernel declares one staged lane twice` | none given |
| 06 two kernels of one step declare one staged lane | block and value | `error[E0080]: evaluation panicked: two kernels of one step declare the same staged lane` | none given |
| 06c a runner opens a step from an unchecked table | data-bound | `error[E0308]: mismatched types` ... `expected `&Checked<i64, _>`, found `&[&[Column<i64>]; 1]`` | not in the brief |
| 06d a runner builds a checked table by hand | data-bound | `error: cannot construct `Checked<_, _>` with struct literal syntax due to private fields` | not in the brief |
| 11 a column handle is read as another value type | data-bound | `error[E0308]: mismatched types` ... `expected `Column<i64>`, found `Column<u32>`` | not in the brief |
| 07 the view is kept past the step | block | `error[E0521]: borrowed data escapes outside of associated function` | a lifetime error |
| 08 a close unit obtains a view | close unit | `error[E0599]: no method named `view` found for struct `Landed<'c, Reg, S>` in the current scope` | E0599 or E0277 |
| 08b a close unit names a frame lane | close unit | `error[E0277]: `Pos` is not a landed store` | not in the brief |
| 09 a staged handle is used after being handed on | block | `error[E0382]: borrow of moved value: `pos`` | E0382 or E0499 |
| 09b a staged handle is borrowed twice | block | `error[E0499]: cannot borrow `pos` as mutable more than once at a time` | E0382 or E0499 |
| 10 a staged handle is sent to two scoped threads | block | `error[E0382]: use of moved value: `pos`` | none given |

Fixture 04 fails with a different class from the one predicted, E0308 and not E0277. With a read list of one member the solver has one candidate for `R: Contains<X>`, unifies `X` with that member before it looks at the argument, and reports the argument's type as wrong. The refusal holds and the message names both lanes, but the class depends on the list's length: 04b, the same read with two lanes listed, gives the predicted E0277. A gate that greps for E0277 on a missing read would miss the one-lane case.

Every twin differs from its failing file in the offending token only, except 07, where `Some(view)` becomes `None`: the offending part there is the whole stored value, and the twin replaces it at that one spot.

Compile-pass, `author/tests/run.rs`: `the_committed_view_is_read_from_two_scoped_threads_at_once` reads two lanes of the view from two `std::thread::scope` threads in one block kernel and stages their sum. WORKS, with no `unsafe impl`: `View` is `Sync` by the auto traits of its parts. Two more runtime tests step a block and a value kernel together, run the close twice, and check the committed values.

## The position mask at lengths 8, 32 and 128

`author/tests/lengths.rs` and `lengths_128.rs` build registries of 8, 32 and 128 frame lanes. Each test reads back the mask and length of a kernel writing every lane and of two kernels writing the two halves, asserts the halves are disjoint and their union is the whole, and instantiates both on a frame so the `const` block runs.

| Length | Evaluates | Recursion limit the instantiating crate needs |
|---|---|---|
| 8 | yes | default (128) |
| 32 | yes | default (128) |
| 128 | yes | over 256: at 256 the solver stops with `error[E0275]: overflow evaluating the requirement `(Tag<L127, *mut [i32]>, ()): Pick<L127, Here>``; 300 holds |

At 128 the registry walk (`Pick`, one step per lane) runs nested in the write-list walk (`Stage`, one step per member), so the depth is about the sum of the two. The limit is an attribute in the crate that instantiates the kernel, the author's crate, though not a feature line. The test file's own `list!` macro needs a raised limit at 128 as well, for the macro's recursion. The `u128` mask has no bit past 128; a larger registry was not probed.

The mask assertion is evaluated during monomorphisation. `cargo check` does not report it: a scratch example declaring `Pos` twice passed `cargo check -p author --example check_probe` and failed `cargo build` of the same example with the E0080 above, after which the probe was deleted. trybuild builds its fixtures, so 05 and 06 hold under it. A gate that only checks does not see a kernel naming a lane twice.

## The data-bound twin of 05 and 06

Fixtures 05 and 06 decide the write lists at compile time, which needs the lists fixed in the engine's types. When bindings are data, as the plan's call 7 default has them for stored columns, no list exists for the compiler to fold, and two writers of one column can only be refused as a value. This correction came after the brief, from two outside readers (`briefs/fixup/stamps-2-spj.md` and `stamps-2-torvalds.md`, section 5 of each). `engine/src/bound.rs` builds that form beside the typed one.

A column is a `Column<T>` handle, minted by `Columns::bind` and refused with `None` once all `C` columns are bound. A kernel carries its handles as data, and a step's write table is one slice of handles per kernel. `Checked<T, K>` is the checked table. Its fields are private and `Checked::bind` is its only constructor, which refuses any column claimed twice, within one kernel's set or across two, with an `Overlap` naming the column and both claimants. `Columns::begin` takes a `&Checked` and nothing else. It gives each kernel a `ColumnStaged` holding exactly the staged columns its set names, split out of the staging buffers by safe borrows, and `set` on a column the set does not name returns `Undeclared`.

`author/tests/bound.rs`, all WORKS:

| Test | What holds |
|---|---|
| `a_second_writer_of_one_column_is_refused_at_bind` | `bind` returns `Overlap { column: 0, first: 0, second: 1 }` |
| `one_kernel_naming_one_column_twice_is_refused_at_bind` | `Overlap { column: 0, first: 0, second: 0 }`, the twin of 05 |
| `disjoint_write_sets_bind_and_step_together` | two kernels step twice through the reference runner and land the expected values |
| `a_write_to_a_column_the_set_does_not_name_is_refused` | `set` on an undeclared column returns `Undeclared { column: 1 }`, and on a declared one `Ok` |
| `a_column_past_the_capacity_is_refused_at_bind` | the fourth `bind` on a three-column store returns `None` |
| `the_staged_columns_of_two_kernels_run_on_two_scoped_threads` | both kernels' staged columns go to two `std::thread::scope` threads, with no unsafe anywhere in this form |

The compile-fail pairs 06c, 06d and 11 are in the fixture table: an unchecked table cannot open a step, a checked table cannot be written out by hand, and a handle minted for one value type does not read another.

Which of the two the contract can rest on: the data-bound form. Under call 7's default the stored columns are bound from data, so the type-level mask of 05 and 06 has no list to fold for them, and holds only where a list is fixed in the engine's types. The data-bound form is also safe without the check. The split hands each staged column to at most one kernel, so a second claimant would get nothing rather than an alias. What the check adds is that the second writer is refused at bind, by name, instead of finding at the step that its writes are refused. The price is that the three refusals the typed form makes at compile time become values. A second writer is refused at bind, a write to an undeclared column at the write, and a column past capacity at bind. Only the value-type confusion (11) and the bypasses of the check (06c, 06d) stay compile errors.

The handle's perimeter: `Column<T>` is typed by its value and not branded by the store that minted it. A handle carried to another store of the same `T` reaches the column at the same index there, and panics on the bounds check where that store is smaller. A generative brand (an invariant lifetime minted with the store) would close that. It was not built here.

## Feature lines per crate

| Crate | Feature lines | What for |
|---|---|---|
| `engine` | `const_trait_impl` | `const impl Cardinal for Slot` (`engine/src/slot.rs`). `Position<N>::INDEX` exists only for `N: const Cardinal` (`notko-hlist/src/position_const_path.rs:64`), and the mask is folded from it. |
| `author` | none | |
| `author`'s tests | none | `lengths_128.rs` carries `#![recursion_limit = "300"]`, not a feature line |
| `notko-hlist` | its own, through its default features: `const_trait_impl` and `marker_trait_attr` | `Cardinal` as a const trait and `Contains` |

The brief spells the impl `impl const Cardinal`. On this nightly, and in `notko-hlist`'s own doctests, it is written `const impl Cardinal for T`.

No `generic_const_exprs`, no `specialization`, and `min_generic_const_args` was not probed, as the brief asks.

## The one unsafe

There is one unsafe operation, in the engine, and no `unsafe impl`. `Stage::parts` (`engine/src/stage.rs`) cuts each `StagedLane`'s `&mut [Item]` from a raw slice pointer into the staging buffers:

```rust
let lane: *mut [H::Item] = *raw.lanes.pick();
// SAFETY: the pointer came from the staging buffers borrowed for 's
// (`RawLanes::new`), and the caller's contract says nothing else
// reaches this lane while the handle lives.
let head = StagedLane::new(unsafe { &mut *lane });
```

The invariant it asserts: no two live staged handles reach one lane. The `const` block in `Frame::step` and `Frame::step2` establishes it before the call. A write list names each lane once when `MASK.count_ones() == LEN`, and two kernels of one step are disjoint when `A::MASK & B::MASK == 0`. `RawLanes<'s, Reg>` carries the borrow of the staging buffers for `'s`, so no handle outlives the step. Authored kernels write no unsafe and hold no slice or pointer, which is stamp 7 option a. The soundness of the cut rests on the mask assertion, and a runner that skipped it would hand out aliasing handles.

A fully safe cut exists: pluck each member out of a list of `&mut` slices by type, which makes the remainder a different type. It turns fixtures 05 and 06 into E0277 trait errors at the declaration instead of a `const` assertion at instantiation. The brief asks for the position-mask form for those two, so the pluck was not built; it is the alternative to weigh if the unsafe cut is not wanted.

## Fallbacks

None. Each check the brief names holds in types (01 to 04, 07 to 10) or, for 05 and 06, as the `const` assertion the brief prescribes. Nothing moved to a debug-build checker.

## What was not as the brief said

- `impl const Cardinal` is spelled `const impl Cardinal for T` on the pinned nightly.
- Fixture 04 is E0308, not E0277, when the read list has one member (04b above).
- The brief limits the sketch to `notko` and `notko-hlist`, so arvo is not reachable. The mask (`u128`), the row count (`u32`), the frame length (`usize`) and the author's items (`i64`, `i32`) are bare primitives here. The production form takes arvo's types, and a registry past 128 lanes needs arvo's wider bit set.
- Fixture 06 decides a form the plan does not expect to use for stored columns, so its data-bound twin was added (section above).

## For the next step

- `notko-hlist` has `At<P>`, which goes from position to member. It has nothing that goes from member to position, which the mask needs. The sketch infers the position through its own `Pick<X, P>` on a lane-tagged list and reads the count back through `Position`. The lookup belongs upstream in `notko-hlist` beside `At`, so the engine does not carry a second list walk.
- `notko-hlist`'s `Contains` impls carry no `#[diagnostic::do_not_recommend]`, unlike `At`'s since `1a978e5`. A downstream compile-fail diagnostic on a missing member therefore prints the cargo git checkout path (fixture 04b's `.stderr` names `/root/.cargo/git/checkouts/notko-f6f26a62b445f4df/1a978e5/...`), and the committed diagnostic matches only on a machine with the same cargo home.
- The reference runner is one thread: `step2` runs its two kernels in turn, but it cuts both kernels' staged handles from one raw set, so the disjointness assertion is already what a parallel runner would rest on.
- In hilavitkutin's vocabulary a frame lane is a `Column<T>` stepped per frame; the landed store has no settled name there.
