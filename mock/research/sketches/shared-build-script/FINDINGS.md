# A build script shared by path

Outcome: WORKS for building, FAILS for packaging, WORKS for packaging through a symlink.

`crates/a` names `build = "../../build/prototype.rs"`. `cargo build` compiles and runs the shared script; a second build with nothing changed does not run it again, since it prints `cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER`. With `cargo::rerun-if-changed=build.rs` in its place the script ran on each of three builds in a row: the file does not exist in the crate, and cargo reruns a script whose watched file is missing.

`cargo package -p shared-build-script-a` refuses: "the source file of build script doesn't appear to be a path inside of the package".

`crates/b` has `build.rs` as a symlink to `../../build/prototype.rs` and no `build` key. It builds, and `cargo package` packs the symlink's content as `build.rs`.

Run with the toolchain of `mock/rust-toolchain.toml`, nightly-2026-05-28, on Linux, 15 November 2026: `cargo build -p shared-build-script-a` three times, then `cargo package -p <crate> --allow-dirty --no-verify`.
