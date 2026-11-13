# Runner-generic work unit, findings

Chosen arm: E2, one small trait per handed thing. Both arms keep the unit byte-identical under all five runners, but E2 got there with nothing added on the runner side, while E1 needed a where-clause on its context, a fresh borrow per row, a function pointer in place of a type parameter for the permuted layout, and a reach and an event named by every declaration whether used or not. At growth E2 edited no existing runner and E1 edited all of them.

Bounds the fused and threaded runners needed beyond the first two: fused, none. Threaded, three, all on the runner's own split helper and none on the unit or the vocab: in E2 `S::Prod<Col<'a>>: Sync`, `S::Prod<ColMut<'a>>: Send` and `E: Send`; in E1 `S::Prod<Arr<N>>: Sync`, `S::Prod<ColMut<'a>>: Send` and `E: Send`. They come down to the members' values being `Send + Sync` and the event being `Send`, which `i32` and `Crossed` are without a word written. Every runner in both arms leans on `Elem: 'static` with `Value: Copy + 'static`, declared once in `owed`.

Growth edit counts: E1 edited 5 of 5 existing runner crates, each gaining the same three-line `draw` on its context impl, and 3 hand-written fixture contexts (6 files with their twins). E2 edited 0 of 5. In both arms the unit hash did not move.

The question was whether fusion and threading can arrive in the plumbing with no change to an author. They can, under either arm: the unit crate is the same bytes under the ordered, permuted, fused, threaded and registry runners, and all ten runners end on the bytes and the ordered events of a plain array model. What separates the arms is the cost on the runner side and what growth touches.

```
holds for: toolchain nightly-2026-05-28, target x86_64-unknown-linux-gnu,
           notko-hlist at rev 1a978e5c2db6c8f68bf94a6fcae266394f172b17 with default features off,
           world EAST = 16, NORTH = 16, TICKS = 6, members i32,
           asm reading: release, codegen-units = 1
```

## What is here

`owed` holds the type-level pieces the arms share and that belong in notko-hlist (listed at the end). `plumb` is runner plumbing with no opinion on either arm: the grid, the wrap, the seeded permutation, the event log sorted by tick and row, the column and record cores, the registry of 64 stores, and the seeded draw the growth probe added. Each arm has a vocab crate, a unit crate depending on the vocab alone, five runner crates, a check crate, a fixtures crate and a growth crate. `shared/` holds the test bodies both checks include, the plain model among them.

The three kernels: (a) `integrate`, the control, `Pos += Vel`; (b) `smooth`, a window over the two-axis key that reads `Pos` at an offset computed from `Lag`, within a reach of 3 east and 2 north, and writes `Heat`; (c) `cross`, which emits `Crossed` when the step carries `Pos` over 1000. The model checks that the world exercises every kernel: more than four events, spread over several ticks, and all 35 offsets of the window reached.

## Per arm and per runner

| runner | E1 | E2 |
| --- | --- | --- |
| ordered | WORKS | WORKS |
| permuted, records in a seeded order | WORKS | WORKS |
| fused, one pass per row | WORKS | WORKS |
| threaded, two scoped threads | WORKS | WORKS |
| registry, 64 stores, the unit reads 4 | WORKS | WORKS |
| growth, the sixth with the fourth kernel | WORKS | WORKS |

Neither (b) nor (c) stalled under either arm.

Tests: 12 in the workspace, all green. Per check crate three: every runner against the model's bytes and events, the world exercising every kernel, and a negative control where a one-bit change to the model must be seen. Per growth crate two: the sixth runner against the model with the kick, and the kick moving the world. Per fixtures crate one trybuild run of five failing files and five twins.

## Feature lines and the recursion limit

No crate carries `#![feature]` or `recursion_limit`. Every library crate is `#![no_std]` except `e1/run-threaded` and `e2/run-threaded`, which take std for scoped threads, and the check and fixtures crates, which hold only tests. The 64-store registry is 64 levels deep in its runner crate and needed no `recursion_limit`.

What the registry did need is a different shape for whole-store operations. A by-value `Clone`, `Default` or splat over a 64-deep product recurses with a stack frame per level holding the rest of the product, and it overflowed the default 2 MB test thread in release as well as in debug; 16 MB ran it. Writing the operations in place (`copy_from`, `splat_in` over `MaybeUninit` with field writes, `made` on top of it, and `mem::swap` between the double buffers) runs it on the default stack. Anything deep in notko-hlist that moves a whole product by value has the same exposure.

## Unit hashes

b3sum after `cargo fmt`, in `unit-hashes.base.txt`, and the same after the growth probe in `unit-hashes.grown.txt`:

```
81ddf3371ec0d509bc093e43136fd0a15cb3deda39847ac5c6282b7bf50d4ff4  e1/unit/Cargo.toml
4bbb50d0f29569c26874683639c5eea801bbc8d9fcaa3178577efda6dc669c5a  e1/unit/src/lib.rs
d4c1c4c60b1cc042a88cbb6144b9e969d64b8169eeca3ec9a57c1e0fe86af1af  e2/unit/Cargo.toml
a94d24609f56e0635361f4d8a7cccb8071622e627628f6162ee21158f473830d  e2/unit/src/lib.rs
```

## What E1 cost on the runner side

The context is the runner's associated type `Cx<'a, D: Decl>`, and a context that borrows a sink of the declaration's event needs `D::Event: 'a` on the associated type itself, or E0309 (`diagnostics/e1-context-borrows-event-sink.txt`). The where-clause is the first runner's need written into the one trait every runner shares.

The context has one lifetime for both the columns it reads and the sink it writes, and a product over a carrier is invariant in that lifetime, so columns borrowed for the whole tick hold the sink for the whole tick too, E0499 (`diagnostics/e1-one-lifetime-context-ties-sink-to-columns.txt`). Every E1 runner borrows the store afresh per row instead.

The context type carries only the declaration, so the places of its reads inside the store cannot be a type parameter of it. The permuted runner erases its projection into a function pointer in `AosFnCore` where E2's `AosCore` keeps the places as a type. Every declaration names a reach and an event even when it has none (`Still`, `()`), and kernels with no event write into a scratch log.

## Growth probe

The new capability is a seeded draw for the kernel's key at the tick, a pure function of both, so every layout and every split agrees. The fourth kernel, `jitter`, kicks `Vel` by the draw taken modulo 3, minus 1. The sixth runner is the ordered one running all four kernels, and it ends on the bytes and events of the model with the same kick.

E2: the vocab gained a four-line trait `Draw`. The growth crate holds the kernel, which takes `&impl Draw` beside its reads, and a runner whose handle implements `Draw`. No existing runner was touched, and none would be until it wants to run a kernel that takes a draw.

E1: the vocab gained `fn draw(&self) -> u32` on `Ctx`. Every existing runner then failed with E0046 (`diagnostics/e1-growth-new-capability-breaks-every-runner.txt`), and so did the three fixture contexts. Each got the same three lines. A default body would have kept them compiling by giving every runner a draw it does not have. Bounding the kernel on a separate trait for the context instead would avoid the edits, but that is E2's shape inside E1.

In both arms `plumb` gained the `draw` function, and the shared test body was split so the growth checks reuse the model (`shared/model.rs`, `shared/same_bytes.rs`, `shared/grown.rs`). Neither is a runner crate. `runner-hashes.base.txt` holds the hashes of every runner, vocab and plumb file from before the probe; `b3sum --check` against it fails on exactly the five E1 runners, the two vocabs and plumb.

## Compile-fail pairs

Each failing file sits beside a twin differing in the offending token only, both run by trybuild in `e1/fixtures` and `e2/fixtures`, with the diagnostic committed as the `.stderr` beside the failing file.

| pair | expected | E1 | E2 |
| --- | --- | --- | --- |
| read outside the list | E0277 | E0277 | E0277 |
| impl adding a bound | E0276 | E0276 | E0276 |
| asking for the row | E0599 | E0599 | E0599 |
| a handle kept past the call | a lifetime error | E0597 | E0597 |
| a member named twice, read once | ambiguity | E0283 | E0283 |

Read outside the list reports at the bottom of the recursion, ``the trait bound `Empty: Has<Heat, _>` is not satisfied``, with notes walking back up to the list the kernel declared. The member named twice lists the two candidate impls of `Has`, the `Here` one and the `There` one, and under E1 suggests a turbofish, ``cx.get::<Pos, P>(Pos)``, which the kernel rules forbid. The row in E1 is refused on `&<X as Runner>::Cx<'_, K>` and in E2 on `&impl Reads<L>`; the twin is the same body written against the runner's concrete type, which is the runner-side code that may ask.

## Asm of kernel (b)

An ad-hoc reading of the release asm, not a measurement; the call listing is `asm/calls.txt`.

Ordered runner, both arms: no call to `smooth` survives, the kernel is inlined into `run`, and the only calls left are to `integrate` through a register, `memcpy` and the bounds-check panic.

Fused runner, both arms: no call to `smooth` survives either, and `integrate` is the one kernel still called, directly.

`integrate` is a plain non-generic function in the unit crate, so without `#[inline]` or LTO it stays a call; `smooth` and `cross` are generic over their handles and are instantiated inside the runner.

## Owed to notko-hlist

What `owed` holds, and both arms lean on, that the pinned notko-hlist does not offer:

- the selector `Has<M, P>`, a member to its place with the place inferred, `pick` and `pick_mut` through a carrier;
- the projection `Project`, a product of a list over a carrier `Prod<C>`, with `read_row`, `write_row`, `cols`, `cols_mut`, `split_mut`, `copy_from`, `splat_in` and `made`;
- the sub-list projection `SubOf<S, Ps>`, with `sub`, `put_row` and `put_vals`;
- `Carrier` and the four carriers `Val`, `Col`, `ColMut` and `Arr`, with the capabilities `ByRow`, `ByRowMut` and `Dup`;
- `Vals<L>`, the values of a list at one row, read and written by member value.

## Where this differs from the brief

`owed` is a crate both vocabs re-export rather than a part of each vocab, so the pieces owed to notko-hlist are written once. Members are bare `i32` and positions bare `usize` and `u32`, because the brief excludes arvo. The kernels are free functions, so the impl adding a bound is a context or handle impl. The fixture asks for the row only; no runner here works in morsels, so there is no morsel range to ask for. The window kernel's key has two axes, per the coordinator's correction.
