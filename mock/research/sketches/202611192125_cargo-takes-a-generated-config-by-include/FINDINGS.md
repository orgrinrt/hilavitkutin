# Sketch findings: cargo takes a generated config file through one `include` line

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the pragma round can keep everything it generates for cargo in a file of its own beside `config.toml`, so a consumer's `config.toml` gains one line and is never written into again

## Hypothesis

The pragmas of workspace reach (LTO, incremental, the backend, the link driver, the C compiler variables) are cargo configuration, read before any build script runs. The consumer-setup round settled that a `config.toml` which exists is never written over. One line in it, `include = [{ path = "hilavitkutin-build.toml", optional = true }]`, would let a script keep a file of its own next to it, and cargo would read that file's profile tables as its own.

One binary crate, a workspace of its own, with `.cargo/config.toml` holding only the include line. The generated text is `generated/hilavitkutin-build.toml`. Built under `nightly-2026-05-28` (`cargo 1.98.0-nightly`), with no `-Z` flag for the include.

## What was tried, in order

### 1. The line with no file beside it: taken, and nothing changes

`cargo build --offline --release -v`, run in this directory before `.cargo/hilavitkutin-build.toml` existed:

```
     Running `.../rustc --crate-name include_taker --edition=2024 src/main.rs ... -C opt-level=3 -C embed-bitcode=no ... -C strip=debuginfo ...`
    Finished `release` profile [optimized] target(s) in 0.10s
```

No LTO argument. An optional include of a file that is not there is no error, so the line can be committed before the first build writes the file. This is also the case that must fail to show the key: without the file, cargo passes no `-C lto`.

### 2. The file placed: the profile key is on the rustc line

`generated/hilavitkutin-build.toml` copied to `.cargo/hilavitkutin-build.toml`, then the same command:

```
   Compiling include-taker v0.0.0 (...)
     Running `.../rustc --crate-name include_taker --edition=2024 src/main.rs ... -C opt-level=3 -C lto=thin ... -C strip=debuginfo ...`
    Finished `release` profile [optimized] target(s) in 3.34s
```

Cargo rebuilt the crate by itself, since the profile changed, and `-C lto=thin` is in the line.

## The result

On the pinned toolchain the `include` key of a cargo config file needs no unstable flag, takes a path relative to the file that holds it, and with `optional = true` accepts a file that is not there yet. A profile table in the included file is read as the workspace's own.

The build that writes the file runs before cargo has read it, as with the wrapper: the first build is without the keys and the next one takes them.

## What is NOT established

Which of two files wins where `config.toml` and the included file set the same key was not tried.

A `[target.<triple>]`, an `[env]` and an `[unstable]` table in the included file were not tried here; the sketch `202611191615_cargo-keys-for-driver-backend-and-cc` shows each taken from a file handed over with `--config`, which cargo reads the same way.

A config file in a directory above this one that also includes a file of the same name was not tried.
