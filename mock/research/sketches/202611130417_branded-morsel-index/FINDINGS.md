# Findings: the branded morsel index

Outcome: WORKS. `each` hands a block kernel its morsel's columns and an index under one brand, the index has no equality, order, hash, formatting, arithmetic or integer conversion, `read` and `write` at it are safe, and there is no `unsafe` anywhere in the sketch: both crates carry `#![forbid(unsafe_code)]` (`engine/src/lib.rs:8`, `author/src/lib.rs:6`). Every refusal the brief names holds as a compile error and every twin builds and runs. Date 2026-11-13, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the repository's pinned `nightly-2026-05-28`, target `x86_64-unknown-linux-gnu`, trybuild `=1.0.122`.

This is sketch S2 of the roadmap for building kaski's engine on the stack's contracts (job `hv-sketch-branded-index`, step HV2b). It gates the block kernel only (V3, HV15): the value, window, fold and emitter kernels are handed values and handles and have nowhere to put an index. Stamp 7 option a, that authors write no unsafe and hold no slice or pointer, rests on it for the block kernel; S3 (`202611122242_frame-view-typestate`) carries it for the rest.

Two crates make the split, as in S3. `engine` stands in for hilavitkutin and holds the brand, the morsel and the one-thread runner; `author` stands in for a game crate and only writes block kernels. The fixtures are trybuild pairs under `author/tests/ui/`, the failing file in `fail/` with its committed `.stderr` and its twin of the same name in `pass/`, built and run. hilavitkutin declares no canon (`mockspace.toml` has no `canon_paths`), so nothing here was read against canon rows; the brief, the roadmap entry and stamp 7 are what it was read against.

The sketch needs no feature line in either crate and pulls in nothing but trybuild. No `generic_const_exprs`, no `specialization`. Rows and lengths are bare `usize` and the authors' items bare `i64`, since the brief reaches no arvo; the production form takes arvo's types.

## How to run it

From this directory: `cargo test`. The clean run is `test-output.txt`. The fixtures alone are `cargo test --test fixtures`, and `TRYBUILD=overwrite` rewrites the diagnostics. The mutants are `cargo test -p author --test fixtures --features mutant-bare` and the same with `mutant-covariant`, each expected red, recorded in `mutants/`. The listing in `asm/checks.txt` is from `cargo rustc -p author --release --lib -- --emit asm`. Paths in the recorded outputs are the machine they were recorded on.

## The shape

`Morsel<'m, R, W>` is one run of rows of a read column and a write column, built only by `Morsels`, which refuses two columns of unequal length with `Unequal` (`engine/src/morsel.rs:90`). A block kernel implements `BlockKernel<R, W>` and is handed one morsel at a time by `run`.

`Morsel::each` (`engine/src/morsel.rs:25`) takes a closure bounded `for<'b> FnOnce(Block<'_, 'b, R, W>) -> O`. The brand `'b` is universally quantified, so the closure has to hold for every `'b`, and the output `O` is chosen outside the binder and cannot name it. `Block` has `indices()`, which yields `Index<'b>`, `read(index)` and `write(index, value)`. The brand is `PhantomData<fn(&'b ()) -> &'b ()>` (`engine/src/index.rs:11`), invariant and still `Send` and `Sync`.

`Index<'b>` (`engine/src/index.rs:26`) is a private `usize` and the brand, deriving `Clone` and `Copy` and nothing else. It is `Copy` on purpose: a kernel reads and then writes at one index, and keeps one across rows (`Previous`, `author/src/lib.rs:53`).

What makes the reads safe: every block of a brand is the one `each` built, its write column is cut to the read column's length (`engine/src/morsel.rs:32`), and every index of that brand comes from its `indices`, which runs below that length. The reads and writes index the slices with ordinary bounds checks all the same (`engine/src/morsel.rs:63`, `:69`), so the brand is not what keeps them in range; it is what lets the optimiser see they are, below.

## Per fixture

The predicted column is the brief's. Every code in the third column is what the compiler said.

| Fixture | Compiler said | Predicted |
|---|---|---|
| 01 two indices compared with `==` | `error[E0369]: binary operation `==` cannot be applied to type `engine::Index<'_>`` | E0369 |
| 02 two indices compared with `<` | `error[E0369]: binary operation `<` cannot be applied to type `engine::Index<'_>`` | E0369 |
| 03 an index cast `as usize` | `error[E0605]: non-primitive cast: `engine::Index<'_>` as `usize`` | E0605 |
| 04 an index formatted with `{:?}` | `error[E0277]: `engine::Index<'_>` doesn't implement `Debug`` | E0277 on `Debug` |
| 05 an index hashed through `Hash::hash` | `error[E0277]: the trait bound `engine::Index<'_>: Hash` is not satisfied` | E0277 on `Hash` |
| 06 an index converted by `i128::from` | `error[E0277]: the trait bound `i128: From<engine::Index<'_>>` is not satisfied` | not in the brief |
| 07 two indices added | `error[E0369]: cannot add `engine::Index<'_>` to `engine::Index<'_>`` | not in the brief |
| 08 an index of one `each` written through another's block | `error[E0521]: borrowed data escapes outside of closure`, twice | a lifetime error |
| 08b the same, read through the other's block | `error[E0521]: borrowed data escapes outside of closure`, twice | not in the brief |
| 09 an index returned out of its `each` | `error: lifetime may not live long enough` | not in the brief |
| 10 an index stashed in the kernel's state for the next morsel | `error[E0521]: borrowed data escapes outside of closure` and `... outside of method` | not in the brief |
| 11 an index written out as a struct literal | `error[E0451]: fields `at` and `brand` of struct `engine::Index` are private` | not in the brief |
| 12 the orphan route: `impl From<Mine> for Index<'b>`, building one in its body | `error[E0451]: fields `at` and `brand` of struct `engine::Index` are private` | not in the brief |
| 13 an index made by `mem::zeroed` | `error: usage of an `unsafe` block`, the `forbid(unsafe_code)` lint | not in the brief |

Every class is the one predicted. The lifetime refusals come in two wordings: E0521 where the index meets a closure's or a method's outer scope (08, 08b, 10), and an uncoded `lifetime may not live long enough` where it is the closure's return (09). A gate grepping for one code would miss the other.

Fixture 05 calls `Hash::hash` as a path. As a method, `probe.hash(..)`, the refusal would be an E0599 on the method instead; that form was not committed.

The twins of 01 to 07 and 09 differ from their failing file in one line: `let probe = index;` becomes `let probe = block.read(index);`, and the operation under test, the next line, is the same text on the value read at the index. The twins of 08 and 08b swap which of two indices is used, `kept` (the outer `each`'s) and `mine` (the inner one's), in the two lines that name them. Three are exceptions, named. In 10 the twin's kernel state is `Probe<i64>` and not `Probe<Index<'k>>`, besides the binding, because the state has to hold what is stashed. In 11 the twin takes the minted `index` in place of the struct literal, the one way to have an index. In 12 the twin keeps the same `impl From<Mine> for Index<'b>` and its body is `loop {}` in place of the literal, which shows the impl itself is admitted. 13's twin is the same file without the `#![forbid(unsafe_code)]` line.

## The seals, and the orphan route

The index has no unsafe precondition to put in a type: the sketch has no `unsafe` operation, and a forged index costs a panic at the bounds check and never undefined behaviour. What the type still has to deny is a forged index that is in range for the wrong morsel, since that reads a wrong row quietly.

Inside safe code the seal is the private fields. A struct literal is refused (11). The orphan rule admits `impl<'b> From<Mine> for Index<'b>`, a foreign trait for the foreign type with a local type parameter: the twin of 12 builds. Its body still cannot build an index, for the same E0451. No trait in the engine is implemented generically over an author's type, so no blanket impl gives that route anything more.

Outside safe code there is no seal in the type. 13's twin makes an index with `mem::zeroed` and writes at it, and it runs. The only refusal is the `forbid(unsafe_code)` in the author's crate, a crate attribute and not part of `Index`. Zeroed is the one forgery whose value is defined whatever the layout, and it is row 0 of every morsel; a forged index past the end would hit the bounds check at `engine/src/morsel.rs:63` or `:69` (follows from the code, not recorded).

## The two mutants

`mutant-bare` makes `Index<'b>` a bare `usize` (`engine/src/index.rs:33`). Nine failing fixtures then build (01 to 05, 08, 08b, 09, 10), and 06, 07, 11 and 12 fail with another error than the committed one (`mutants/mutant-bare.txt`). 13 holds, its refusal being the lint.

`mutant-covariant` makes the brand `PhantomData<&'b ()>` (`engine/src/index.rs:15`). Only 08b builds; 01, 02, 07, 08, 09 and 10 still fail, with changed wording (`mutants/mutant-covariant.txt`). So invariance is not what keeps an index inside its `each`. The higher-ranked binder does that alone: 09 and 10 hold under the covariant brand. Where invariance is load-bearing is a read through another `each`'s block. `read` takes `&self`, a shared borrow is covariant in the block, and the compiler shortens both brands to a common region and accepts the call. `write` takes `&mut self`, which is invariant in the block whatever the brand, so 08 holds under the mutant too. Before 08b was added the suite was red under this mutant only because diagnostics were worded differently, with no failing fixture building, which a blessed rerun would have turned green (observed, not recorded). A brand that is only covariant passes every write fixture and fails on the first read.

## The hole the brand does not close

The index has no order, but `indices` yields in one, and an author can count along it. `Indices` is an `Iterator`, so `nth(2)` hands back the index at position 2, an integer reaching an index by another road: `author/tests/run.rs:81`, run in `counting_along_the_indices_reaches_a_position`, WORKS. Taking `Iterator` away would not close it, since a counter in the loop body does the same. The brand denies operations on the index; it does not deny an author knowledge of the order the engine iterates in. The thing to hold for V3 is that the order is unspecified in the contract, and a permuting runner (as the runner sketch has, `202611122252_runner-generic-workunit`) is what catches a kernel leaning on it.

## Bounds checks in release

An ad-hoc reading of the release asm, not a measurement; the listing is `asm/checks.txt`. With the engine's small functions `#[inline]`, `Double` and `BelowMax` carry no per-row bounds check, one `slice_index_fail` each at entry for the cut of the write column (once per morsel), and `Previous` carries that and one `panic_bounds_check` for the read at the index kept from the row before, which the optimiser cannot tie to the loop. Before the `#[inline]`s, `Indices::new` and `next` were called out of line across the crate border, and each kernel kept two `panic_bounds_check` calls, one per column (observed, not recorded). The cut at `engine/src/morsel.rs:32` did not change that on its own.

## What was not as the brief said

- The lifetime refusal is E0521 for the brief's case of a brand used in another `each` (08), and the read form of it (08b) is the only fixture an invariant brand is needed for.
- The brief's five refusals were joined by seven: `From` conversion, arithmetic, the read path, an index kept past its `each`, one kept across morsels, and the two safe forging routes and the unsafe one.

## For the next step

- V3's block kernel can take this shape as it is: `each` with a higher-ranked brand, an invariant `Index`, `read` and `write` at it. Its signature stays unlocked until HV15, as the roadmap says, since the morsel here is one read and one write column and not the store's layout.
- The block kernel's contract says its iteration order is unspecified, and the laws run it under a permuting runner.
- A gate over these fixtures accepts both lifetime wordings, E0521 and the uncoded one.
- The engine's index arithmetic stays inlinable across the crate border, or the per-row checks come back.
