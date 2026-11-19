# Where `prefer-dynamic` has to reach for a dylib domain

**Outcome:** WORKS with `-Cprefer-dynamic` on the dylib crate alone; without it a whole-workspace release build FAILS WITH "cannot satisfy dependencies so `std` only shows up once".

19 November 2026. Holds for: Linux x86_64, `nightly-2026-05-28`, one rlib under one dylib under one binary. Two dylib domains side by side, a dylib that depends on another, LTO and wasm are not covered.

The note `mock/research/202611191520_the-pragma-round-before-it-opens.md` has the edge form write "`prefer-dynamic` through the wrapper on every crate". This sketch asks which crates need it. `leaf` is a plain library, `domain` is `crate-type = ["dylib"]` over it, `app` is the binary.

## With the flag nowhere

```
$ cargo run -p sketch-edge-app
leaf domain
```

The dev build of the one selected package links and runs, and `readelf -d` shows both the binary and `libsketch_edge_domain.so` needing `libstd-1ee7395f7ada8841.so`. The dylib's rustc line, taken afterwards with `cargo rustc -p sketch-edge-domain -v`, carries no `-C prefer-dynamic`.

```
$ cargo build --release -v
rustc --crate-name sketch_edge_domain ... --crate-type dylib ... -C opt-level=3 -C embed-bitcode=no ...
error: cannot satisfy dependencies so `std` only shows up once
  = note: `sketch_edge_domain` was unavailable as a static crate, preventing fully static linking
error: could not compile `sketch-edge-app` (bin "sketch-edge-app") due to 19 previous errors
```

The two runs differ in profile and in which packages were selected, so this sketch does not say which of the two makes the first one link. It says that a dylib domain left to cargo alone does not link in every build a consumer runs.

## With the flag on the dylib crate alone

`domain/build.rs` hands `-Cprefer-dynamic` to the workspace wrapper this sketch inherits from `mock/.cargo/`, for its own crate and no other.

```
$ cargo build --release
   Compiling sketch-edge-domain v0.0.0
   Compiling sketch-edge-app v0.0.0
    Finished `release` profile [optimized] target(s) in 0.29s
$ cargo run --release -p sketch-edge-app
leaf domain
```

`leaf` and `app` were built without the flag.

## What this gives the pragma round

The edge form's flag belongs to the crate that is the dylib, like a linker pragma belongs to a crate that links (`202611191600_a-link-flag-reaches-the-final-crate-alone`): it goes through the wrapper from that crate's own script, and it is not a workspace choice. "On every crate" in the note is wider than this case needs. Whether a second domain or a binary that also links std by itself changes that is for a cell with two domains.
