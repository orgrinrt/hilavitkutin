# src changelist

Round topic `202611200124-arjq_topic.the-buffered-mark-and-the-stale-partial-pass.md`, under the locked `202611200124-arjq_changelist.doc.lock.md`.

## Changes proposed

### 1. The import's mark on a buffered store

`mock/crates/hilavitkutin/tests/host_import.rs`, the `buffered!` modules, both modes: a unit that reads the buffered store and writes `Column<B>`. It rests on a clean frame; after an import it runs, `B` holds twice the import, and a frame later the buffered store still holds the import. It fails if `import` copies without marking the store.

### 2. The stale partial pass, catalogued

The same modules: `FirstWindow` becomes a unit that maps every window while one flag holds and the first window only while another does. A pass over every window, then a pass over the first window only, must leave a record of an unwritten window at the first pass's value. Under `Snapshot` it holds; under `Exchange` it is red, ignored with `a_pass_that_skips_map_in_a_window_publishes_stale_records` named, and the ticket says the red test now stands.

### 3. Refusals pinned

An empty import past the count is refused, as the empty dump is. A refused import into a buffered store moves nothing on either side, and a buffered dump at an offset gives the records from that offset.

### 4. The fused refusal's twin

`mock/crates/hilavitkutin/tests/ui/buffered_unit_fused_pass.rs`, the `_fail` fixture over `Column<A>` in place of `Buffered<A, Snapshot>`, and `tests/ui.rs` runs it. The `_fail` fixture's comment names the ticket of the gap.

### 5. `import`'s `SAFETY` comment

`mock/crates/hilavitkutin/src/scheduler/host.rs`: the comment no longer claims `&mut self` shuts out a frame in flight; it says soundness is not established under `run_parallel` and keeps the `FIXME(placeholders_tagged_74e2b9918149)`. No code changes.
