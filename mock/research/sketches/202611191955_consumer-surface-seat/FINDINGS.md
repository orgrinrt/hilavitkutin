# Consumer surface probe, 2026-11-19

**Outcome:** FAILS WITH the default interner breaking the interning law, the overflow sentinel turning into an out-of-bounds read (SIGSEGV), a unit unable to register the interner resource (E0599: `Copy` and `ResourceFootprint` unmet), the shipped `HasInterner` shape being unimplementable (E0207, E0210), and the sixteen-byte limit not enforced.

The probe was written by the expert seat of the job `hila-consumer-surface`, whose report is `mock/research/202611191955_consumer-surface-seat/NOTES.md`. It reads four requests an outside consumer made of the engine against the shipped crates at hilavitkutin `origin/dev` of 19 November. It is one instance of each result and carries no more weight than that.

## Toolchain

`cargo +nightly-2026-05-28`, with `Cargo.lock` copied from `mock/Cargo.lock`. Without the lock, notko `dev` resolves to 15a9531 and `hilavitkutin-api` fails with E0107. `cargo +nightly` (1.102.0-nightly of 2026-11-15) fails in `arvo-strategy`; the output is `output/plain-nightly-build.txt`.

## Files and results

`probe/tests/interner.rs` holds results 1 to 6: two interns of one runtime string through the default interner give unequal handles (1), and the same through `Interner<StrDomain>` (2); five interns of one string at the arena give ids `[0, 1, 2, 3, u32::MAX]` (3); the overflow sentinel masked into the 28-bit handle carries `0x0FFF_FFFF` (4); resolving that handle reads entry 268435455 of the table and the process dies with SIGSEGV (5); two distinct strings that both overflow come back as one handle (6). `probe/tests/column_value_width.rs` holds result 11, `Column::<[u8; 4096]>` registering although the ratified limit is sixteen bytes. `reach/src/lib.rs`, features `direct` and `newtype`, holds results 7 and 8, a unit reading `Resource<StringInterner<MemoryArena<256, 16>>>` failing to build on the unmet `Copy` and `ResourceFootprint` bounds, and a local newtype still failing on `Copy`. `accessor/src/lib.rs`, features `fixed` and `method`, holds results 9 and 10: a blanket `HasInterner` impl fails with E0207 and E0210, and a generic-method extension trait over `ResourceProviderApi` builds.

The raw outputs are under `output/`: `probe-tests.txt`, `overflow-resolve.txt`, `reach-direct.txt`, `reach-newtype.txt`, `accessor-fixed.txt`, `accessor-method.txt`, `column-value-width.txt`, `plain-nightly-build.txt`.

## Commands

```
cargo +nightly-2026-05-28 test -p consumer-surface-probe -- --nocapture
cargo +nightly-2026-05-28 test -p consumer-surface-probe --test interner resolve_of_the_overflow_handle -- --ignored
cargo +nightly-2026-05-28 build -p consumer-surface-reach --features direct
cargo +nightly-2026-05-28 build -p consumer-surface-accessor --features fixed
```

## Limits

Miri is not installed for the pinned toolchain, so the out-of-bounds read is shown by the signal and the index arithmetic. No timing was taken and nothing here prices any shape. Result 10 shows only that the trait shape is legal.
