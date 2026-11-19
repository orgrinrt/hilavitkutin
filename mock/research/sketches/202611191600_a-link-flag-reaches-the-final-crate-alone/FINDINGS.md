# Sketch findings: a link flag chosen by the final crate's build script reaches that crate's link and rebuilds nothing else

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the pragma round can hand a linker pragma's flags to the crate that links, through the wrapper the crate already has or through cargo's own directive, and say that a library's set carries none

## Hypothesis

A linker is chosen for a link, and a workspace has one link per binary. The crate hands rustc flags over today by printing `cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=...` from a crate's build script, and the workspace wrapper appends that variable to the crate's own rustc. If the final crate's script alone prints a link flag, the binary takes it and the libraries under it are not rebuilt when it changes. A link flag printed by a library's script does nothing to the binary and still costs a rebuild.

A workspace of a library `link-flag-dep` and a binary `link-flag-app` on it, each with a build script, and the wrapper with the text `write_wrapper` writes, named in `.cargo/config.toml`. The scripts read what to print from environment variables, so one tree shows every case. The flag is `-Wl,--build-id=<hex>`, since `readelf -n` shows it in the binary and no second linker has to be on the machine.

Built under `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`) on x86_64 Linux, from this directory, with `cargo build -v`. Paths below are shortened to the crate; the rest is as cargo printed it.

## What was tried, in order

### 1. No flag: both crates build

```
   Compiling link-flag-dep v0.0.0 (.../crates/dep)
   Compiling link-flag-app v0.0.0 (.../crates/app)
```

### 2. The final crate's script chooses a link flag: the binary alone is rebuilt and carries it

`env SKETCH_APP_FLAGS=-Clink-arg=-Wl,--build-id=0x1111aaaa cargo build -v`

```
       Dirty link-flag-app v0.0.0 (.../crates/app): the env variable SKETCH_APP_FLAGS changed
   Compiling link-flag-app v0.0.0 (.../crates/app)
       Fresh link-flag-dep v0.0.0 (.../crates/dep)
```

`readelf -n /home/user/target/debug/link-flag-app`

```
Displaying notes found in: .note.gnu.build-id
  GNU                  0x00000004	NT_GNU_BUILD_ID (unique build ID bitstring)
    Build ID: 1111aaaa
```

### 3. The library's script chooses a link flag too: both are rebuilt and the binary is unchanged

`env SKETCH_APP_FLAGS=-Clink-arg=-Wl,--build-id=0x1111aaaa SKETCH_DEP_FLAGS=-Clink-arg=-Wl,--build-id=0x2222bbbb cargo build -v`

```
       Dirty link-flag-dep v0.0.0 (.../crates/dep): the env variable SKETCH_DEP_FLAGS changed
   Compiling link-flag-dep v0.0.0 (.../crates/dep)
       Dirty link-flag-app v0.0.0 (.../crates/app): info of dependency `link-flag-dep` changed
   Compiling link-flag-app v0.0.0 (.../crates/app)
```

```
    Build ID: 1111aaaa
```

rustc takes `-Clink-arg` on a library without a word and links nothing with it.

### 4. The link argument by cargo's directive, with no wrapper flag: the binary alone is rebuilt and carries it

The final crate's script prints `cargo::rustc-link-arg-bins=<arg>`. After one build that cleared the variables of attempt 3 (both crates dirty, as expected), the value was changed:

`env SKETCH_APP_LINK_ARG=-Wl,--build-id=0x4444dddd cargo build -v`

```
       Dirty link-flag-app v0.0.0 (.../crates/app): the env variable SKETCH_APP_LINK_ARG changed
   Compiling link-flag-app v0.0.0 (.../crates/app)
       Fresh link-flag-dep v0.0.0 (.../crates/dep)
     Running `.../.cargo/hilavitkutin-rustc-wrapper .../bin/rustc --crate-name link_flag_app ... -C link-arg=-Wl,--build-id=0x4444dddd`
```

```
    Build ID: 4444dddd
```

### 5. The case that must fail: the flag of attempt 2 with no wrapper

`env RUSTC_WORKSPACE_WRAPPER= SKETCH_APP_FLAGS=-Clink-arg=-Wl,--build-id=0x1111aaaa cargo build`

```
   Compiling link-flag-dep v0.0.0 (.../crates/dep)
   Compiling link-flag-app v0.0.0 (.../crates/app)
```

```
  GNU                  0x00000014	NT_GNU_BUILD_ID (unique build ID bitstring)
    Build ID: 901f43f2efd404708de6cfb932abcb3240968c59
```

The linker's own twenty bytes: the flag did not arrive. So the id of attempt 2 came through the wrapper and from nowhere else. Both crates were rebuilt, since the wrapper is part of what cargo holds a unit to.

## The result

A link flag printed by the final crate's build script reaches that crate's link through the workspace wrapper, and changing it rebuilds that crate alone; the library under it stays fresh.

The same flag printed by a library's script changes nothing in the binary and rebuilds the library and everything on it. A linker pragma in a library's set is cost with no effect.

A plain link argument needs no wrapper: `cargo::rustc-link-arg-bins` from the final crate's script does the same, and the wrapper is then needed only for what is no link argument (`-Clinker=`, `-Zcodegen-backend=`, `-Clinker-flavor=`).

Taking the wrapper away rebuilds every crate of the workspace.

## What is NOT established

Whether a build script can tell that its crate links. `CARGO_BIN_NAME` is not set for a build script, and nothing here reads the crate's targets; the sketch names the final crate by hand through which variable is set.

A test binary, an example or a `cdylib` of a library crate is a link of its own, and none was built here.

`-Clinker=` and a second linker (`mold`, `wild`, `lld`) were not run; the flag here is one the system linker takes.

A dependency from outside the workspace was not in the tree, though the workspace wrapper does not run for one by cargo's own rule.

No time was taken. What a rebuild costs is a bench.
