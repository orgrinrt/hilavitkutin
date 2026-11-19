# Sketch findings: each linker is taken by arguments one crate's script can hand over, and leaves a mark

**Date:** 2026-11-19
**Outcome:** WORKS for mold, GNU ld and gold by one link argument, and for wild by a link argument and the clang driver; FAILS WITH `cc: error: unrecognized command-line option '-fuse-ld=wild'` for wild under the gcc driver
**Unblocks:** the pragma round can state, per linker pragma, what it writes and what proves the linker linked

## Hypothesis

A linker pragma reaches the crate whose set holds it. For each linker the round names there is a set of arguments that crate's script can hand over, a link argument by `cargo::rustc-link-arg` or a rustc flag through the wrapper, and the linked binary says which linker made it.

One binary crate whose script prints each word of `SKETCH_LINK_ARGS` as a `cargo::rustc-link-arg`. Built under `nightly-2026-05-28` on x86_64 Linux with gcc 13.3.0 and clang on the path. mold 2.30.0 came by `apt-get install -y mold`. wild 0.10.0 was built by `cargo install --locked --version 0.10.0 --root /root/.cache/hilavitkutin-build/tools/x86_64-unknown-linux-gnu/wild-0.10.0 wild-linker`, the path the round's tools home would give it.

## What was tried, in order

### 1. No argument: the toolchain's own lld

`readelf -p .comment /home/user/target/debug/linker-taker`

```
  [    62]  Linker: LLD 22.1.6 (/checkout/src/llvm-project/llvm 08c84e69a84d95936296dfcab0e38b34100725d5)
```

rustc itself passes `-B<sysroot>/.../gcc-ld -fuse-ld=lld` on this target, as the link line of attempt 5 shows. So lld is the toolchain's own choice here and needs no pragma.

### 2. `-fuse-ld=mold`

```
  [     0]  mold 2.30.0 (compatible with GNU ld)
```

The argument comes after rustc's own `-fuse-ld=lld` on the link line, and the driver takes the last.

### 3. `-fuse-ld=gold`

rustc says, on every build:

```
warning: the gold linker is deprecated and has known bugs with Rust
  |
  = help: consider using LLD or ld from GNU binutils instead
```

`readelf -p .note.gnu.gold-version` shows `gold 1.16`. It links, and rustc advises against it.

### 4. `-fuse-ld=bfd`

```
  [     0]  GCC: (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0
  [    2d]  rustc version 1.98.0-nightly (57d06900f 2026-05-27)
```

GNU ld leaves no line of its own. Its proof is that no other linker's mark is there.

### 5. The case that must fail: wild under the gcc driver

`env SKETCH_LINK_ARGS=-fuse-ld=wild cargo build`

```
error: linking with `cc` failed: exit status: 1
  = note:  "cc" "-m64" ... "-B<sysroot>/lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld" "-fuse-ld=lld" ... "-nodefaultlibs" "-fuse-ld=wild"
  = note: cc: error: unrecognized command-line option '-fuse-ld=wild'; did you mean '-fuse-ld=gold'?
```

gcc 13 knows a fixed list of linkers and wild is not on it.

### 6. wild through clang, by its path

`env "RUSTFLAGS=-Clinker=clang -Clink-arg=--ld-path=/root/.cache/hilavitkutin-build/tools/x86_64-unknown-linux-gnu/wild-0.10.0/bin/wild" cargo build`

```
  [    62]  Linker: Wild 0.10.0 non-git-build (compatible with GNU linkers)
```

The binary runs and prints `linked`. `wild --version` prints `Wild 0.10.0 non-git-build (compatible with GNU linkers)`.

## The result

mold and GNU ld are each one link argument, `-fuse-ld=<name>`, which a script prints as `cargo::rustc-link-arg` with no wrapper involved. wild is a link argument that names the binary by its path, `--ld-path=<path>`, and needs clang as the driver of that link, which is the rustc flag `-Clinker=clang` and goes through the wrapper. Each of mold, wild and lld writes its name and version into `.comment`; GNU ld writes nothing.

lld is what the pinned toolchain links with on this target when nothing is asked, so it is the absence of a linker pragma here. gold links and rustc warns against it on every build.

A locally built tool under a tools home is used by its full path and needs nothing on `PATH`.

## What is NOT established

Any target other than x86_64 Linux. What the toolchain links with by default elsewhere, and whether `-fuse-ld=lld` is then a pragma worth having, is each target's arm to show.

wild under a gcc new enough to know it was not tried; this machine has gcc 13.

The `-Clinker=clang` here came by `RUSTFLAGS`, which reaches every crate of the build. Through the wrapper it reaches the one crate; that route is the crate's own and is shown for another flag in `202611191600_a-link-flag-reaches-the-final-crate-alone`.

No time was taken. Which linker is faster is a bench.
