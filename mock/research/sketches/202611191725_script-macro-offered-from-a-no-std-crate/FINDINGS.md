# `script!` offered from a `no_std` crate, and the three cases the wiring owed

**Outcome:** WORKS, with one change to the mechanism: a tree cargo unpacked is told by `Cargo.toml.orig` at its root, since `CARGO_HOME` alone lets `cargo package` fail.

19 November 2026. Holds for: Linux host, `nightly-2026-05-28` under rustup, cargo's own resolver, a shared target directory outside the workspace. Windows and a cargo run without rustup are not covered.

The doc changelist of round `202611151157` left four things to show before the source changelist locks. `script-lib` stands in for `hilavitkutin-build`: it is `#![no_std]`, allocates nothing, and exports `script!` with `#[macro_export]`. The decisions (`wiring`, `config_names_wrapper`) and the text (`write_wrapper`, `write_config`, `configure`) are functions over `core::fmt::Write`. Everything that touches the host is in the macro's expansion and names `::std` by absolute path, so it is the script's code and the crate stays as it is. It builds with no feature gate.

## The macro from a `no_std` crate

`consumer/scripts/anywhere.rs` is `fn main() { sketch_script_lib::script!(Deterministic); }`, named by `package.build`.

```
$ env RUSTC_WORKSPACE_WRAPPER= cargo run -p sketch-script-consumer
warning: sketch-script-consumer@0.0.0: sketch: wrote the wrapper and its config line, under <sketch>/.cargo
flag arrived: false, deterministic: false
$ cargo run -p sketch-script-consumer
flag arrived: true, deterministic: true
```

The second build prints no warning: cargo hands `RUSTC_WORKSPACE_WRAPPER` to the script once the config names the wrapper, and `wiring` answers `Wired`.

## A `config.toml` that exists without the line

`own-config/.cargo/config.toml` is committed as `[build]` with `incremental = true`.

```
$ env RUSTC_WORKSPACE_WRAPPER= cargo run --manifest-path own-config/Cargo.toml
warning: sketch-own-config@0.0.0: sketch: wrote the wrapper; config.toml exists and names no wrapper, add `build.rustc-workspace-wrapper` to it, under <sketch>/own-config/.cargo
flag arrived: false
```

The file is left as it was and the wrapper is written beside it. Cargo replays the warning on every build of the member until the line is there.

## `cargo package` with its verify build

With the wrapper named, `cargo package --workspace --allow-dirty` packages and verifies both crates. With none named, the first form of the macro failed:

```
$ env RUSTC_WORKSPACE_WRAPPER= cargo package --workspace --allow-dirty
warning: sketch-script-consumer@0.0.0: sketch: wrote the wrapper and its config line, under /home/user/target/package/sketch-script-consumer-0.0.0/.cargo
error: failed to verify package tarball
Caused by:
  Source directory was modified by build.rs during cargo publish. Build scripts should not modify anything outside of OUT_DIR.
  Added: /home/user/target/package/sketch-script-consumer-0.0.0/.cargo
```

The verify build unpacks the crate under the target directory, and `cargo locate-project --workspace` answers that directory, which is not under `CARGO_HOME`. Every tree cargo unpacks carries `Cargo.toml.orig` at its root, so the macro now treats a root with that file as fetched. At version `0.0.1`, so the script ran again:

```
$ env RUSTC_WORKSPACE_WRAPPER= cargo package --workspace --allow-dirty
   Compiling sketch-script-consumer v0.0.1 (/home/user/target/package/sketch-script-consumer-0.0.1)
warning: sketch-script-consumer@0.0.1: sketch: fetched checkout, nothing written: /home/user/target/package/sketch-script-consumer-0.0.1
```

The run between the two, at `0.0.0` after the change, did not rerun the script and proves nothing.

## A checkout under `CARGO_HOME`

`takes-by-git` depends on `dep-lib` by git at `4a78664b`, with the local clone standing in for the remote. Cargo does not show a dependency's warnings, so the evidence is the script's recorded output and the checkout itself:

```
$ env RUSTC_WORKSPACE_WRAPPER= cargo run --manifest-path takes-by-git/Cargo.toml
the dependency answers 7
target/debug/build/sketch-dep-lib-837698476220d94e/output:
cargo::warning=sketch: fetched checkout, nothing written: /root/.cargo/git/checkouts/hilavitkutin-9ace09fd58e0f6b2/4a78664/.../dep-lib
$ ls -a <checkout>/dep-lib
Cargo.toml  build.rs  src
```

`own-config/build.rs` prints whether `CARGO_HOME` reaches a script when the caller sets none (`env -u CARGO_HOME`): `true`, under rustup's cargo. A cargo run without rustup is not shown here, so the source falls back to the home directory's `.cargo` where the variable is absent.

`dep-lib/.cargo/` in this directory comes from a build of `dep-lib` in place that was not one of the runs above. It is the same first-build wiring, in a tree that is the consumer's own, and it is committed as it was written.

## What this gives the source changelist

The wiring declines in two cases, a root under `CARGO_HOME` and a root that holds `Cargo.toml.orig`. `Prototype` is named by identifier in the macro. A dependency's script stays silent to the consumer whatever it decides.
