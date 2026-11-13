# s5: what the trunk's ColumnValue admits, and what a consumer's lane format gets

Spike, seat 02, round 202611131951. Uses the shipped `hilavitkutin-api` at `271909e5`.

| Test (`tests/admits.rs`) | Result | What it says |
|---|---|---|
| `a_consumers_own_types_are_column_values_at_their_native_width` | green | a consumer's `Q16(i32)`, `Q32(i64)`, `i128`, `u8` and a packed `Lanes4x16([u16; 4])` are all column values at their own width. The tickets' premise, "Word is the storage unit and Cell64 the widest cell" and "Value is sealed and Int<BITS> is its only impl", describes the voided `hilavitkutin-contracts`, not the trunk |
| `a_value_wider_than_sixteen_bytes_is_admitted_against_r3` | green, and should not compile per the spec | the founding spec caps a value at sixteen bytes (consolidation `:2416-2419`, R3; R4 `:2421-2426`). The trunk's blanket (`column_value.rs:25-34`) has no cap, and never had one (`git show 6597bbfc:mock/crates/hilavitkutin-api/src/column_value.rs`) |
| `nothing_below_a_byte_is_packed_by_the_store` | green | `bool` reports 8 bits; `BIT_WIDTH` is the container width and nothing packs (`column_value.rs:8-12`) |
| `the_two_lane_formats_compute_the_same_product_where_both_fit` | green | control for the assembly below |

`asm/functions-x86-64-v3.s`, release, `-Ctarget-cpu=x86-64-v3`, read and not timed:

- `s5_mul_q16` (a consumer's 32-bit lane, 64-bit product): `vpmuldq` and `vpsrlq` on `ymm`,
  eight records an iteration.
- `s5_mul_q32` (kaski's 64-bit lane, `i128` product): one-operand `imulq` and `shldq` per record,
  four unrolled scalar products, vector registers only to store.

So the lane width is the consumer's choice of `T` and the code shape follows from it; the store
does not need to know. How much the narrow lane is worth is a bench's, under `mock/benches`.

Run: `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --release --no-fail-fast -- --test-threads=1`
and `env -C <this dir> CARGO_INCREMENTAL=0 cargo rustc --release --lib -- --emit=asm -Ctarget-cpu=x86-64-v3`.
