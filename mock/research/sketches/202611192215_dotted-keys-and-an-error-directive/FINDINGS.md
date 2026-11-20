# Sketch findings: the generated cargo file in dotted keys, and a script that refuses by `cargo::error`

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the generated cargo file can be written one pragma at a time with no table header, and a refusal can be a directive of the script

## Hypothesis

Several pragmas write keys of one cargo table: an LTO pragma and an incremental pragma both write into the table of one profile. TOML refuses a table header written twice, so a file put together pragma by pragma cannot use headers. A dotted key at the top of the file, `profile.release.lto = "fat"`, names the same key with no header, and two pragmas can each write theirs.

The other question is the refusal. A pragma the target does not admit should fail the build with a sentence, and the design has the script print `cargo::error=`.

One binary crate with a build script, a workspace of its own, `.cargo/config.toml` holding only the include line. Built under `nightly-2026-05-28` (`cargo 1.98.0-nightly`), from this directory with `cargo -Z unstable-options -C <this directory> build --offline --release -v`. Cargo finds its configuration from the directory it runs in and not from the manifest, so `--manifest-path` from elsewhere reads none of this.

## What was tried, in order

### 1. No generated file: the control

```
     Running `.../rustc --crate-name dotted_taker --edition=2024 src/main.rs ... -C opt-level=3 -C embed-bitcode=no ... -C strip=debuginfo ...`
```

No `-C lto`, no `-C codegen-units`, no `-C linker`.

### 2. `generated/dotted.toml` placed as `.cargo/hilavitkutin-build.toml`: every key is taken

```
     Running `.../hilavitkutin-rustc-wrapper .../rustc --crate-name dotted_taker --edition=2024 src/main.rs ... -C opt-level=3 -C lto=fat -C codegen-units=1 ... -C linker=clang -C strip=debuginfo ...`
```

`profile.release.lto`, `profile.release.codegen-units` and `target.x86_64-unknown-linux-gnu.linker` are on the line. The binary prints `CC as cargo set it for this build: clang`, so `env.CC` reached the crate's rustc. `unstable.codegen-backend = true` and the two `incremental` keys are accepted with no complaint. The wrapper in the line is the one of the `config.toml` in `mock/.cargo/` above this directory, which cargo also reads.

### 3. `generated/duplicate.toml.txt` placed as the generated file: one key written twice stops cargo

The file is kept under a name that is not `.toml`, since it is not valid TOML and the repository checks every file that is.

```
error: could not load Cargo configuration

Caused by:
  failed to load config include `hilavitkutin-build.toml` from `.../.cargo/config.toml`

Caused by:
  could not parse TOML configuration in `.../.cargo/hilavitkutin-build.toml`

Caused by:
  TOML parse error at line 4, column 10
    |
  4 | unstable.codegen-backend = true
    |          ^^^^^^^^^^^^^^^
  duplicate key
```

Cargo stops before it builds anything, so no build script runs and none can repair the file. A generated file with a key twice leaves a workspace that does not build until somebody removes the file by hand.

### 4. `SKETCH_REFUSE=1`: two error directives, and the script goes on

```
   Compiling dotted-taker v0.0.0 (...)
error: dotted-taker@0.0.0: sketch: Mold links ELF only, and this target is not ELF
error: dotted-taker@0.0.0: sketch: a second refusal of the same script
warning: dotted-taker@0.0.0: sketch: the script ran to its end
error: build script logged errors
```

The build fails with exit code 101. Both errors are shown, the script ran to its end, and the warning it printed after them is shown too.

## The result

A cargo config file may be all dotted keys with no table header, and cargo reads each as the key of its table. The generated file can therefore be written one pragma after another.

A key that belongs to no profile (`unstable.codegen-backend`, `target.<triple>.linker`, `env.CC`) must be written once in the whole file, however many profiles' sets hold its pragma. Two pragmas of one group that both write such a key with different values in two profiles cannot both be in the file at all.

`cargo::error=` fails the build after the script has run to its end, and a script may print several.

## What is NOT established

Whether a dotted key and a table header for the same table may sit in one file was not tried; the generated file has no header.

Which wins where the consumer's manifest and the generated file set the same profile key was not tried here. Cargo's documentation has configuration ahead of the manifest.
