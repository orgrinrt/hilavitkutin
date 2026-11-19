# What this copy changes against the shipped engine

`src/` is a copy of `mock/crates/hilavitkutin/src` at commit `2b137e9d` of `dev`, and `build.rs` a copy of the crate's build script. `Cargo.toml` is written for the sketch: the package is named `hilavitkutin-aligned`, doctests are off (the copied doc examples name the crate `hilavitkutin`), and the dependencies are inherited from the sketch's workspace, which spells them as `mock/Cargo.toml` does. `diff -r mock/crates/hilavitkutin/src engine-aligned/src` shows the two edits below and nothing else.

## Edit 1, `src/lib.rs`

After the shipped line 69 (`pub use plan::{DefaultPlanDims, PlanDims};`), lines 70 to 79 add `pub const ACCUM_GROUP: arvo::USize = arvo::USize(8);` with its doc comment. It is the width of one accumulator group in records. A crate constant serves the sketch only; a design would take it from the morsel, a plan dimension or the run configuration, and the sketch does not choose.

## Edit 2, `src/resource/bindings/per_core.rs`

The shipped `accum_per` (lines 98 to 103) returns `ceil(total / ncores)`. The copy's (lines 98 to 112) returns `ceil(ceil(total / G) / ncores) * G` with `G = ACCUM_GROUP`, so every core's slice is a whole number of groups and starts on a multiple of `G` from record zero. Nothing else in the crate spells the split: `accum_slice` is its one reader, and both the worker's rebase (`scheduler/worker.rs:213`) and the merge (`per_core.rs`, `MergeAccums for AccumBinding`) take their offsets from `accum_slice`, so they move together. A surplus core still gets an empty slice and appends nothing.
