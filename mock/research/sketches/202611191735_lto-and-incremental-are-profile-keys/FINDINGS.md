# LTO and incremental are profile keys, written by config and not by the wrapper

**Outcome:** WORKS as keys of a cargo config file; a per-crate `-Clto` through rustc's arguments FAILS WITH "options `-C embed-bitcode=no` and `-C lto` are incompatible".

19 November 2026. Holds for: Linux x86_64, `nightly-2026-05-28`, the release profile, a workspace of one library and one binary. Other profiles, fat LTO and a workspace with a `cdylib` are not covered.

The note `mock/research/202611191520_the-pragma-round-before-it-opens.md` left open where the LTO and incremental pragmas fall: with the flags a crate's script hands the wrapper, or with the workspace choices a config file carries. This sketch asks cargo both ways.

## By a config file

`configs/thin.toml` sets `[profile.release]` `lto = "thin"` and `incremental = false`.

```
$ cargo build --release --config configs/thin.toml -v
rustc --crate-name sketch_lto_dep ... --crate-type lib ... -C opt-level=3 -C linker-plugin-lto ...
rustc --crate-name sketch_lto_app ... --crate-type bin ... -C opt-level=3 -C lto=thin ...
```

Cargo turns the one key into two different arguments: the library is built to bitcode alone, and the binary runs the LTO. No single flag appended to every crate says the same.

`configs/incremental.toml` sets `incremental = true`.

```
$ cargo build --release --config configs/incremental.toml -v
rustc --crate-name sketch_lto_dep ... -C embed-bitcode=no ... -C incremental=/home/user/target/release/incremental ...
rustc --crate-name sketch_lto_app ... -C embed-bitcode=no ... -C incremental=/home/user/target/release/incremental ...
```

The directory is cargo's to choose, under its target directory.

## By a flag on one crate

```
$ cargo rustc --release -p sketch-lto-dep -- -Clto=thin
error: options `-C embed-bitcode=no` and `-C lto` are incompatible
```

Cargo has already decided how the library is emitted from the profile, and a flag appended after it contradicts that decision.

## What this gives the pragma round

LTO and incremental are workspace choices, like driver, backend and C compiler in `202611191615_cargo-keys-for-driver-backend-and-cc`: the pragma writes a key of the generated config and nothing into `HILAVITKUTIN_BUILD_RUSTFLAGS`. That is where the design already puts `ExpandedLto` ("generated Cargo config"). The edge form is not shown here.
