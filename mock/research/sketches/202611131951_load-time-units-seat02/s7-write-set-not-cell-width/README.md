# s7: the morsel's L1 promise breaks on a fiber's write width, not on a cell's

Spike, seat 02 phase two, round 202611131951. Uses the shipped trunk planner at `271909e5` and its
default run config (`L1_USABLE` 24576 B, `MIN_MORSEL` 64). Written to test seat 01's inference from
its c1, that "The record's 16-byte bound is what keeps the formula meaningful", and the rationale of
seat 01's derived bound `size_of <= L1_USABLE / MIN_MORSEL` (384 B).

Step 9 sums the fiber's write set (`hilavitkutin/src/plan/steps/step9_morsel.rs:40-56`) and clamps
the window below at `MIN_MORSEL` (`:73-75`), as the record specifies
(`(L1_usable / Σ write_sizes).clamp(MIN_MORSEL, MAX_MORSEL) & !3`, consolidation spec `:834`).

| Test (`tests/window.rs`) | Result | Line from `OUTPUT-tests.txt` |
|---|---|---|
| `control_one_16_byte_write_column_fits` | green | 16 B a record, window 1536, morsel write set 24576 B of 24576 |
| `forty_16_byte_write_columns_each_inside_r3_fit` | red | every cell 16 B (inside R3), 640 B a record, window 64, write set 40960 B of 24576 |
| `two_384_byte_write_columns_each_inside_the_derived_bound_fit` | red | every cell 384 B (inside seat 01's bound), 768 B a record, window 64, write set 49152 B of 24576 |

So no per-cell bound keeps the promise; the quantity is a fiber's summed write width against
`L1_USABLE / MIN_MORSEL`. Since the record itself writes the clamp, a fiber past that width
overshooting L1 is the record's designed trade, not drift; the drift is only the missing cell cap.

`OUTPUT-seat01-c1-b2-rerun.txt` is seat 01's own `c1-oversized-cell` and `b2-resize-loses-records`
rerun from seat 01's branch tip `80e85ad4` in a detached read worktree: both reds reproduce
(window 64, write set 262144 B; eight records read back as zeros) and `growth_moves_the_base_pointer`
passes.

Run: `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --no-fail-fast -- --nocapture --test-threads=1`.
No timing; nothing here is a bench.
