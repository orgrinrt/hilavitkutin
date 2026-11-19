# Sketch findings: a consumer's own build script, kept anywhere, wires the wrapper for its workspace by itself

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the consumer-setup round can leave every build script with its consumer and still have the wrapper and its config line appear with nothing done by hand

## Hypothesis

A consumer keeps its build scripts where it wants them. One call from such a script can do all the wiring that the wrapper needs: ask cargo for the workspace root, write `.cargo/hilavitkutin-rustc-wrapper` with its executable bit and the `build.rustc-workspace-wrapper` line where they are missing, and print the directives. Cargo reads its configuration when it starts, so the build that writes them runs without the wrapper and the next one takes the flags.

One crate whose manifest names `build = "scripts/anywhere.rs"`. The script's body after `main` stands in for what the call into `hilavitkutin-build` would do in the script's std context. The flag it hands over is `--cfg sketch_flag_arrived`, and the binary prints whether it was built with it.

The sketch sits under `mock/`, whose `.cargo/config.toml` already names a wrapper for everything below it. The first build was therefore run with `RUSTC_WORKSPACE_WRAPPER` set empty, which is how cargo is told there is none.

Built under `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`) on x86_64 Linux, from this directory.

## What was tried, in order

### 1. First build, no wrapper: the script writes both files and the flag does not arrive

`env RUSTC_WORKSPACE_WRAPPER= cargo run`

```
   Compiling self-wiring-script v0.0.0 (...)
warning: self-wiring-script@0.0.0: hilavitkutin-build wrote the wrapper and its config line under .../202611191650_a-build-script-wires-its-own-workspace/.cargo; this build ran without the wrapper, the next one takes the flags
    Finished `dev` profile [unoptimized] target(s) in 0.49s
     Running `/home/user/target/debug/self-wiring-script`
flag arrived: false
```

This is also the case that must fail: with no wrapper the flag is dropped, so the `true` below comes from what the script wrote.

### 2. Second build, nothing changed by hand: cargo rebuilds and the flag arrives

`cargo run`

```
   Compiling self-wiring-script v0.0.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 0.39s
     Running `/home/user/target/debug/self-wiring-script`
flag arrived: true
```

The rebuild came from `cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER` and from the wrapper being part of what cargo holds a unit to. Nobody asked for it.

`ls -la .cargo`

```
-rw-r--r-- 1 root root   70 Nov 19 15:44 config.toml
-rwxr-xr-x 1 root root  145 Nov 19 15:44 hilavitkutin-rustc-wrapper
```

The two files are committed with the sketch as the script left them.

## The result

A build script that the consumer keeps at a path of its choosing can put the wrapper and its config line in place for its workspace, executable bit included, and the build after that takes the crate's flags with no step by hand. `$CARGO locate-project --workspace`, run from `CARGO_MANIFEST_DIR`, gives the script the workspace root.

The first build of a fresh workspace runs without the flags and says so in one warning.

## What is NOT established

A package built as a dependency from cargo's own checkout. The script here declines to write when the workspace root is under `CARGO_HOME`, and that branch was not run.

A `.cargo/config.toml` that exists and names no wrapper. The script reports it and edits nothing; merging a line into a file the consumer wrote was not tried.

`cargo package` with its verify build, where the package is unpacked under the target directory.

Windows, where the wrapper cannot be a shell script.

Two build scripts of one workspace writing the files in the same moment.

Whether the standing-in code can live in a `no_std` crate. It uses `std::fs` and `std::process`, so in the crate it would be text the script expands or includes, and how that is offered is the changelist's to say.
