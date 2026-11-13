# s1: can the record's extension seam load anything on Linux

Spike, seat 02, round 202611131951 (load-time units). It checks one thing: whether the shipped
`hilavitkutin-linking` loader and the shipped `hilavitkutin-extensions` host can load a library
that is not already mapped into the process, here the smallest extension the shipped
`#[export_extension]` macro emits.

- `ext/` is that extension, a no_std cdylib.
- `host/tests/fresh.rs` loads it fresh through `Library::load` and through `ExtensionHost::load`,
  in its own process. Red against the trunk at `271909e5`.
- `host/tests/preloaded.rs` is the control: the library mapped first by `dlopen` with this libc's
  own flags, then loaded through the shipped loader. Green, which is the condition the loader's
  own suite runs under (`mock/crates/hilavitkutin-linking/tests/basic.rs` loads the libc already
  mapped into every process).

Run, from this directory, one call each:

    env -C ext CARGO_INCREMENTAL=0 cargo build --release
    cp <target>/release/libs1_ext.so <scratch>/libseat02_s1_ext.so
    env -C host CARGO_INCREMENTAL=0 EXT_SO=<scratch>/libseat02_s1_ext.so cargo test --no-fail-fast -- --nocapture --test-threads=1

`OUTPUT.txt` is the last run, on x86_64 Linux glibc, nightly-2026-05-28.

What it shows: `Library::load` returns `LoadFailed { platform_code: 0 }` and the host
`LinkFailed { cause: LoadFailed { .. } }` for a fresh library; the same call succeeds once the
library is already mapped. glibc's values read from the libc crate are `RTLD_LOCAL = 0`,
`RTLD_NOLOAD = 4`; the loader passes 4 (`src/backend/unix.rs:9`). What it does not show: anything
on macOS, where 4 is `RTLD_LOCAL` and the loader is presumably right.
