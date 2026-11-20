# Gated interner bridge: can a contracts-only consumer bind `StringInterner` to `InternerApi`

**Outcome:** FAILS WITH E0117

Three crates stand for the shape on default features: `providers.rs` holds the
`InternerApi` contract alone (the bridge impl gated off under `portable`),
`strcrate.rs` holds `StringInterner<A>` over a consumer arena, and
`consumer.rs` brings its own arena and writes the bridge itself. The compiler
refuses it under the orphan rule, so on a contracts-only build the str crate's
interner cannot be the `HasInterner` provider without a newtype. The positive
control is the shipped bridge at `crates/hilavitkutin-providers/src/interner/arena.rs:21`,
which compiles because it sits in the crate that owns `InternerApi`.

Toolchain `nightly-2026-05-28` (`rustc 1.98.0-nightly`), run from this directory:

```
rustc --edition 2024 --crate-type lib --crate-name providers providers.rs
rustc --edition 2024 --crate-type lib --crate-name strcrate strcrate.rs
rustc --edition 2024 --crate-type lib --crate-name consumer consumer.rs \
  --extern providers=libproviders.rlib --extern strcrate=libstrcrate.rlib -L .
```

Compiler output is `consumer.out`. One instance, a spike: it shows the shape,
not the real crates, whose build scripts this does not run.
