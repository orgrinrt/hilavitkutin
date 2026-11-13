# s2: a compiled host unit over an extension surface

Spike, seat 02, round 202611131951. Standalone: no hilavitkutin crate is used; the host's column
table is a `Vec<Vec<i64>>` standing in for the store. It checks four things about the arm in
which the schedule stays compiled and load-time code runs inside a compiled host unit's declared
access envelope, called through a C-ABI function pointer once per morsel.

| Test (`tests/envelope.rs`) | Result | What it says |
|---|---|---|
| `a_conforming_extension_is_admitted_and_computes_the_function` | green | admission by stable store name works and the per-morsel call computes the function |
| `a_write_outside_the_host_units_envelope_is_refused_at_load` | green | a write the host unit never declared is refused before anything runs |
| `a_read_of_a_store_the_host_unit_never_declared_is_refused_at_load` | green | same for a read |
| `swapping_the_variant_between_morsels_changes_nothing_in_the_result` | green | an interpreted variant swapped for a compiled one after morsel 0, 1, 3 or 9 gives the same column: the morsel is a switch point (Kohn, Leis, Neumann, ICDE 2018, fig. 5) |
| `an_extension_that_writes_what_it_declared_as_read_is_caught` | red, on purpose | 4095 of 4096 records of a store declared read-only were overwritten through the read pointer and nothing at the boundary saw it. Admission checks a declaration, not the code behind it |

`asm/functions-x86-64-v3.s` is the release assembly of the four functions, `-Ctarget-cpu=x86-64-v3`,
nightly-2026-05-28, read rather than timed:

- `s2_ext_compiled` (the extension's own morsel loop): no call, AVX2 `vpaddq` over four `ymm`
  registers, sixteen records an iteration.
- `s2_host_per_record` (host calls the extension once a record through an opaque pointer): one
  `callq *16(%rsp)` inside the record loop, no vector instruction.
- `s2_host_per_morsel` (host calls once a morsel): one `callq *(%rsp)` per morsel.

So the shape holds: across a boundary the compiler cannot see through, the call has to sit at
morsel granularity for the extension's loop to vectorise at all. How much that is worth is not
measured here; that needs `mock/benches`.

Run: `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --release --no-fail-fast -- --test-threads=1`
and `env -C <this dir> CARGO_INCREMENTAL=0 cargo rustc --release --lib -- --emit=asm -Ctarget-cpu=x86-64-v3`.
`OUTPUT-tests.txt` is the test run.
