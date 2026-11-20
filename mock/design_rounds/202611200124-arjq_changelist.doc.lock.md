# doc changelist

Round topic `202611200124-arjq_topic.the-buffered-mark-and-the-stale-partial-pass.md`.

## Changes proposed

### 1. The fused entry's buffered refusal is a gap

`mock/crates/hilavitkutin/DESIGN.md.tmpl`, after the paragraph on the accumulator gap of the fused entry: a paragraph saying `FuseCarrier` takes units over plain columns only, so a unit over a buffered column is refused at compile time; the chain threads values over one reservation a column, and a buffered column is two the boundary trades. The three entries keep one meaning, so this is a gap, tracked as ticket `the_fused_entry_refuses_a_buffered_column`, and until it closes such a unit goes through `run` or `run_parallel`.

## Not changed

The canon. The host import and dump section, whose stale-partial-pass premise is already tracked by `a_pass_that_skips_map_in_a_window_publishes_stale_records`.
