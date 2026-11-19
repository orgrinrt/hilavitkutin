# Edits To The Copies

This sketch carries its own copies of two shipped crates, because the route it
tests changes both. `api/` is a copy of `mock/crates/hilavitkutin-api` and
`engine/` is a copy of `mock/crates/hilavitkutin`, each with its `src/`, its
`Cargo.toml` and its `build.rs`, taken from the base of this branch. Nothing
under `mock/crates/` was edited.

Every difference between a copy and the shipped crate is listed here, and the
same differences are kept as diffs under `diagnostics/`, made with
`git diff --no-index` from `mock/` between the shipped path and the copy. A
file not named below is identical to the shipped one. Both `build.rs` copies
are identical.

`kit/` and `providers/` are not copies. Each is a manifest alone whose
`[lib] path` points at the shipped source, with `build = false`, so the
workspace holds one api (the copy) and the shipped kit and providers compile
against it unchanged. Their build scripts do not run here.

## The api copy

The diff of the source is `diagnostics/edits-api-src.diff` and of the manifest
`diagnostics/edits-api-manifest.diff`.

`api/src/block.rs` is new. It holds the block form's contract: `ColList`,
`ColSlots`, `ReadsColumn`, `WritesAt`, `Records` and `Block`. This is the
change the route needs.

`api/src/attempt_blanket.rs` is new and compiles only under the features
`attempt-blanket` and `attempt-blanket-witness`. It holds attempts b1 and b2
and is no part of the route that works.

`api/src/lib.rs` gains two declarations after `pub mod adapt;`: the gated
`pub mod attempt_blanket;` and `pub mod block;`.

`api/Cargo.toml` gains the two features `attempt-blanket` and
`attempt-blanket-witness`, both empty.

## The engine copy

The diff of the source is `diagnostics/edits-engine-src.diff` and of the
manifest `diagnostics/edits-engine-manifest.diff`.

`engine/src/dispatch/engine_ctx/block.rs` is new. It implements `ColSlots` on
the column bundle nodes `ColPtrNil` and `ColPtrCons`, implements `Records` on
`EngineCtx`, and declares the wrapper `Blk<U>` with its `BuilderInput` and
`WorkUnit` impls. This is the change the route needs. The feature
`mutant-short-window` shortens `Records::len` by one record in this file.

`engine/src/dispatch/engine_ctx/attempt_ctx_of.rs` is new and compiles only
under the feature `attempt-ctx-of`. It holds attempts c1 and c2 and is no
part of the route that works.

`engine/src/dispatch/engine_ctx/mod.rs` gains three lines: the gated
`mod attempt_ctx_of;`, `mod block;` and `pub use block::Blk;`.

`engine/Cargo.toml` gains the three features `mutant-short-window`,
`attempt-ctx-of` and `attempt-ctx-of-per-shape`.

## What a round would take from this

The route that works is the two files named as needed, `api/src/block.rs` and
`engine/src/dispatch/engine_ctx/block.rs`, with their module declarations. No
shipped item changed its signature and no shipped file lost a line.
