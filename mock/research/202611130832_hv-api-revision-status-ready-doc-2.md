Source: kaski channel (coord/kaski of clause-dev), att/20261113T083250554027356-hv-api-revision-272327/status-ready-doc-2.md

ready-doc 288811a (hilavitkutin, work/api-revision), replacing 8af52de

Changed since 8af52de:

- Trunk merged (a32fa32, over c42d707 and the branded index sketch). The two conflicts, the api design's bridge paragraph and the engine design's capacity sentence, are consolidated by hand; the overflow lane's text is kept whole.
- `append` is a safe method that takes the value alone, with `len` beside it. 8af52de had it take an index, which would have broken every unit that appends once a frame from outside `each` (item 9). Changelist C8 states the correction and its reason.
- A `Buffered` store is put under the same `RunCfg::Overflow` choice in one paragraph; it has no append and is sized from its domain, so it refuses nothing today.
- Item 8 is done in the doc window, since it is sketch and bench work outside the gated source: sketch `mock/research/sketches/202611130728_data-bound-registration/`, bench `registration_binding` in `mock/benches/`, note `mock/research/202611130728_data-bound-registration-findings.md`. Changelist C10 names them. `cargo mock` green, docs regenerated.

Still waiting on your review of the doc changelist before the source window opens.

A question for op, ticket which_registration_hilavitkutin_s_contract_takes_beside_the. Question tickets do not reach him now, so it is here for you to put to him. Nothing I build next depends on it: the contract keeps typed registration either way.

Which registration does hilavitkutin's contract take beside the typed one? He answered "Decide in the api round" on 13 November; this is that round's sketch and bench.

Numbers (two units, five u32 columns, 512-record morsels, 160 samples an arm, opt 3 fat LTO, one x86_64 cloud host; medians in ns at 16384 and 65536 records):
- typed 13641 / 59608 (0.87 ns a record); the same arm built twice 13499 / 60838 (noise floor, up to 3.2%)
- table for columns, once a morsel, typed units: 13834 / 59091, inside the floor
- table for columns and units, once a morsel: 13697 / 58753, inside the floor
- the same with a run-time stride: 25231 / 104826, 75% to 87% over typed
- table looked up once a record: 108212 / 451524, seven to eight times typed

Sketch: a data-declared table builds no_std, no alloc, no dyn, with three limits the typed contract lacks: plain values only; a data-declared unit is a compiled body behind a function pointer that fusion cannot see through; the plan's refusals become run-time values.

Options:
a (recommended): typed only in this revision; content channels are solved above the engine as data in typed columns; data-declared stores are designed in a round of their own when a consumer needs them. A table is a drop looser in typestate, his pillars put that behind an opt-in feature, and the bench says waiting costs nothing.
b: data-declared plain-value stores under typed units, opt-in behind a feature, bound once a morsel or above. Inside the noise; a missing or mis-shaped store is refused when the scheduler is built, not by the compiler.
c: data-declared units and stores both, opt-in behind a feature, beside the typed form. Inside the noise per morsel; those units do not fuse and their plan is checked at run time. No per-record form.

Also still open with you from the first status: ticket which_names_the_access_list_s_cell_and_end_take_in_hilavitku.
