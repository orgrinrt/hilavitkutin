# `hilavitkutin-conformance`

> The reference runner, the digest and the laws a runner of hilavitkutin work units is held to.

A scheduler gets to split the records into morsels, spread them over cores, fuse units into one loop and rest whatever had nothing change under it, as long as none of that changes what a frame computes. What a frame computes is written down here once, in about the plainest form there is, so any runner and any plan can be compared with it.

`Oracle` is a one-thread reference runner that takes value-form units through `RecordOp::apply` alone, so a unit written for the scheduler runs under it as it stands, and it keeps its stores in storage the caller lends it (`Lent`), which is how it gets by without an allocator. `Fold` writes what a frame left in the stores down as one `Word`, the same on every target. The laws (`model`, `split` and `isolation`) are functions over runs written down that way, and each returns a `Verdict` naming where it first failed. The broken runners a law has to refuse ship beside the faithful one under `oracle::mutant`, for checking that a law does fail when the runner is wrong.

## Usage

It goes in as a dev-dependency beside `hilavitkutin-api`, and takes nothing of the engine with it. The usual way is to run the same unit list for a frame under `Oracle` and under the runner being held, write both down as words, and hand the streams to a law:

```rust
use hilavitkutin_conformance::{Verdict, Word, model};

let w = Word::from_raw;
assert_eq!(model(&[w(1), w(2)], &[w(1), w(2)]), Verdict::Holds);
```

The crate's own tests do this over small unit lists, and are probably the best place to start reading.

## Status

Under active development like the rest of the workspace, so names may still move. Needs nightly, for `adt_const_params`. `no_std`, and no allocator.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/hilavitkutin/blob/dev/LICENSE)
