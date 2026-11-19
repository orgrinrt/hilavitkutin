# Sketch findings: a driver, a backend and the C compiler variables are each one cargo configuration table, and a build takes them

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the pragma round can name, per pragma of the driver, backend and C compiler groups, the configuration keys its tool writes, and say which of them reach more than the workspace's own crates

## Hypothesis

Three of the round's groups are not rustc flags of one crate. A driver is `target.<triple>.linker`, a backend is `profile.<name>.codegen-backend` behind cargo's unstable flag of the same name, and a C compiler is the `CC`, `CXX` and `AR` variables under `[env]`. Each is a table the tool could write into `.cargo/config.toml` beside the wrapper's line, and cargo takes it with no flag on the command line.

One binary crate with a build script that prints the three variables as it sees them. Each table is a file under `configs/`, handed to cargo with `--config <file>`, which reads it as it reads `.cargo/config.toml`.

Built under `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`) on x86_64 Linux, from this directory. The machine has `gcc`, `clang`, `ld.lld` and `ld.gold`; it has no `mold`, `wild` or `zig`.

## What was tried, in order

### 1. No table: the variables are unset and the wrapper of the repository is in the line

`cargo build -v`

```
     Running `/home/user/hilavitkutin/mock/.cargo/hilavitkutin-rustc-wrapper .../bin/rustc --crate-name cargo_keys_taker ...`
warning: cargo-keys-taker@0.0.0: CC is unset
warning: cargo-keys-taker@0.0.0: CXX is unset
warning: cargo-keys-taker@0.0.0: AR is unset
```

The sketch is a workspace of its own and has no `.cargo/` directory. Cargo reads configuration from every directory above the one it runs in, so `mock/.cargo/config.toml` applies here too.

### 2. The driver: `configs/driver.toml`

`cargo --config configs/driver.toml build -v`

```
       Dirty cargo-keys-taker v0.0.0 (...): the config settings changed
     Running `... rustc --crate-name build_script_build ... -C linker=clang ...`
     Running `... rustc --crate-name cargo_keys_taker ... -C linker=clang ...`
    Finished `dev` profile [unoptimized] target(s) in 1.19s
```

The key reaches the build script's link as well as the crate's, since the build names no `--target` and host and target are then one.

### 3. The case that must fail: a driver the machine lacks

`cargo --config configs/driver-missing.toml build`

```
error: linker `no-such-driver` not found
  |
  = note: No such file or directory (os error 2)

error: could not compile `cargo-keys-taker` (build script) due to 1 previous error
```

### 4. The C compiler: `configs/cc.toml`

`cargo --config configs/cc.toml build`

```
warning: cargo-keys-taker@0.0.0: CC=clang
warning: cargo-keys-taker@0.0.0: CXX=clang++
warning: cargo-keys-taker@0.0.0: AR=llvm-ar
```

With the caller's environment already naming one, `env CC=gcc cargo --config configs/cc.toml build`:

```
warning: cargo-keys-taker@0.0.0: CC=gcc
warning: cargo-keys-taker@0.0.0: CXX=clang++
warning: cargo-keys-taker@0.0.0: AR=llvm-ar
```

A variable of `[env]` yields to the one the caller set, unless the table says `force = true`.

### 5. The backend, before the component is on the machine: the key is taken and rustc refuses

`cargo --config configs/backend.toml build -v`

```
     Running `... rustc --crate-name build_script_build ... -Z codegen-backend=cranelift ...`
error: failed to find a `codegen-backends` folder in the sysroot candidates:
       * /root/.rustup/toolchains/nightly-2026-05-28-x86_64-unknown-linux-gnu
```

### 6. The backend, with the component

`rustup component add rustc-codegen-cranelift-preview`, then `cargo --config configs/backend.toml build`:

```
   Compiling cargo-keys-taker v0.0.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 0.20s
```

`readelf -p .comment /home/user/target/debug/cargo-keys-taker`

```
  [     0]  rustc version 1.98.0-nightly (57d06900f 2026-05-27) with cranelift 0.131.0
  [    4b]  rustc version 1.98.0-nightly (57d06900f 2026-05-27)
  [    80]  GCC: (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0
  [    ad]  Linker: LLD 22.1.6 (/checkout/src/llvm-project/llvm 08c84e69a84d95936296dfcab0e38b34100725d5)
```

The first line is the crate's own object, built by cranelift. The second is the standard library's, which the toolchain ships built by LLVM. The last is the linker the toolchain takes when nothing chooses one, its own `rust-lld`.

## The result

Each of the three is one table of cargo configuration and a build takes it: `[target.<triple>] linker` for the driver, `[unstable] codegen-backend = true` with `[profile.<name>] codegen-backend` for the backend, and `[env]` for the C compiler variables.

None of the three stops at the workspace's own crates the way the wrapper does. The driver and the backend reach the build script, and the profile and target tables apply to every crate cargo builds, the dependencies among them. `[env]` reaches every build script, and yields to the caller's environment.

A backend whose component is missing and a driver that is not on the path are both refused by rustc with a line that names what is missing, after cargo has taken the key.

A nested workspace under `mock/` reads `mock/.cargo/config.toml`, so the wrapper's line applies to every sketch and bench workspace below it.

## What is NOT established

The backend was set on the `dev` profile only. Whether cranelift builds the engine's crates, with their feature gates, is not shown by a crate of one line.

Nothing here compiles C. The variables were read by a build script and no `cc` crate was in the tree.

`zig cc` as a driver was not run; it is not on the machine.

Whether a driver and a linker pragma agree (`clang` with `-fuse-ld=mold`, `gcc` with `-fuse-ld=wild`) is not tested here; the sketch `202611191600_a-link-flag-reaches-the-final-crate-alone` carries the link argument and this one the driver, apart.

A build with `--target` set, where host and target differ, was not run, so what the driver key does to a build script there is not shown.

The component was added to this container's toolchain by hand. How the tool table brings it, and what the pin moving does to it, is the round's to say.
