# s3: content is data, units fixed, forms closed

Spike, seat 02, round 202611131951. Standalone, no hilavitkutin crate. A compiled unit holds a
closed set of forms (`Scale`, `Offset`, `Clamp`, `Sum`); content rows read at load name a form,
the stores it binds and its constants. The set of forms and each form's loop are the build's;
which rows exist, what they bind and their constants are the load's.

| Test (`tests/forms.rs`) | Result | What it says |
|---|---|---|
| `rows_compose_to_what_a_hand_written_unit_computes` | green | three rows from a pack (scale, clamp, sum) give what a hand-written unit gives over 10 000 records |
| `a_row_writing_a_store_the_unit_only_reads_is_refused_at_load` | green | refusal at load against the unit's envelope |
| `a_row_naming_a_store_the_unit_never_declared_is_refused_at_load` | green | same, for an undeclared store |
| `a_row_asking_for_a_form_the_build_does_not_hold_is_refused` | green, and it is the arm's wall | a mod that wants a neighbour's value, a form the build does not hold, has no row to write |

`asm/functions-x86-64-v3.s`, release, `-Ctarget-cpu=x86-64-v3`, read and not timed. The result
went against what the spike set out to show: `s3_per_record`, which matches on the form at every
record, came out as four vectorised loops (`vpaddq`, `vpcmpgtq` with `vblendvpd` for the clamp,
the multiply lowered to `vpmuludq` pairs) because the form does not change inside the loop and
LLVM unswitches it; its two calls are the panic paths of `clamp`. So with a form that holds for
the whole morsel the once-a-record and once-a-morsel shapes compile to the same loops, and the
interpreter's cost appears only where the form varies record by record or where a row is
bytecode rather than a closed form (s2's interpreted variant).

Run: `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --release --no-fail-fast -- --test-threads=1`
and `env -C <this dir> CARGO_INCREMENTAL=0 cargo rustc --release --lib -- --emit=asm -Ctarget-cpu=x86-64-v3`.
