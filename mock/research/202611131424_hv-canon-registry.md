Source: kaski channel (coord/kaski of clause-dev), att/20261113T142444803885357-hv-canon-registry-1dde0c/report-hv-canon-registry.md

# hv-canon-registry: report

Branch `work/canon-registry` of `orgrinrt/hilavitkutin`, 29 commits over `dev`, head `c5f337a` when this was written. `cargo mock` passes on the head. Nothing was pushed to `dev` and no pull request was opened.

## What is on the branch

- Part one, the read: `mock/research/202611130850_the-record-read-whole/LEDGER.md`, one line per document with its date, standing and reader. Not read: the research imported from polka-dots and saalis (reader batches 39 to 44 never ran; the week reached the wind-down band first). The ledger and `ANSWERS.md` say so.
- Part two, the registry: `mock/registry/`, 903 rows (841 tenets, 62 questions) with their kinds and fields declared in `mockspace.toml`. Every tenet has a `source` as path and lines and a `quote` filled from those lines by script.
- Part three: `ANSWERS.md`, the eight questions answered from the record with passages quoted.
- His words of 13 November as they came in, verbatim: `OP-ANSWERS.md`, `OP-ANSWERS-13.md`, `-14`, `-18`, `-20`, `-21`. `RECOVERED.md` copies the passages only the recovery branch carries (the branch was read at its ref and not merged). `IKIUNI.md` holds what ikiuni says on buffering, as a reference.
- Part four waits. `orgrinrt/mockspace-extra-lints` `dev` was 91079e5 and `work/tools-upstream` (fc0e294) was not an ancestor of it when last checked (12:20 UTC), so `canon`, `already-said` and `unearth` are not on the pack's `dev`. No tool was copied in.

## The ratification pass (coord, 12:04; op's words in OP-ANSWERS-18.md)

Every row was classed by a reader against the record: 18 reader runs on sonnet high, one per slice, each in its own worktree, one neutral brief, every cited range checked by script.

| class | rows | written as |
|---|---|---|
| a, his | 375 tenets | `standing = "ratified"`, `ratified_by` (by default from the written history, his words quoted), `ratified_from` (kind and passage) |
| a, replaced later | 8 tenets | provisional, stamp naming the later passage of his and its row |
| a, questions | 24 already his answers, 3 answered from the founding spec and A2-3 with `answered_from`, 1 open on a stamp | |
| c, neither | 352 tenets, 9 questions | `stamp`; 9 of the tenets are his only in part and the stamp names the passage |
| b, not backed or against | 106 tenets, 25 questions | untouched; listed in `ROWS-FOR-OP.md` |

By kind, the ratified tenets rest on: the founding spec or the principles (318), an amendment that says he ruled it, tier 2 of the record's own order (22), a passage quoting him or saying he chose it (25), another passage saying he ruled the same content (7), and the founding spec stating the same point elsewhere (3).

`ROWS-FOR-OP.md` has 131 rows in four parts: 22 against a word of his that stands; 42 on 12 points the record states two ways; 54 whose own source says they wait on him (the swap spec, the deviations awaiting a bless, the runtime round's autonomous calls, the erased boundary, the abandoned round 2b, forks handed back); 13 agent rows a later agent round replaced. Each row has its says, the finding, the passages and options. Where a group has no question row the options were written for the file and it says so.

### What I opened myself

- Every class a row that is not from the founding spec and not already his own answer was read a second time against its cited lines: 68 rows, 34 by a second reader and 34 by me. 12 came back as his only in part. 9 moved to a stamp. 3 stayed ratified: the two `op_ruled_...` rows of 18 September (his chosen option is the row's point, the round's application sits beside it) and `a_resource_value_is_a_one_record_blob_...` (bench-decided in the storage addendum, tier 2 by A2).
- Of the founding rows a script checked that each source lies in the founding spec or the principles, and I opened 21 of them. I read every `later:` passage of slices 01 to 04.
- I read every class b line and moved four: two founding rows to b so both sides of a founding-against-founding point are listed (dispatch below ten thousand records; entry against record), one question to b (its std half is answered, its arrayvec half is not), and one row from b to a stamp (`dispatch_runs_in_the_plan_s_rcm_reordered_...`, which A1-1's bench bears out).

### What a reader of this should know

- No reviewer has read the ratification pass. The week reached 90% while the last readers ran, the usage gate refuses a new sub-agent, and I did not start one. The reviewer brief is ready at `/tmp/claude-0/-home-user/0c022aa8-1852-5c29-9672-2df0555bc5b1/scratchpad/review-brief.md` (placeholders `@@WORKTREE@@`, `@@SHA@@`, `@@REPORT@@`). The second half of the second reading was done by me for the same reason.
- Three sonnet-high seats came back having run on opus (ticket c7547243600a); after the third I ran one reader at a time.
- Slices 01 to 04 (the founding rows) were classed before the brief carried the paragraph on the bare-engine benches. They hold founding text only, and I found no row among them that rests a call on those benches.
- OP-ANSWERS-21 names the benches against hand-written and plain Rust paths. The coordinator's message also named the convergence bench and the perf gate, so `which_convergence_a_single_trunk_phase_uses`, the perf-gate row and the deviation-two row are in part two of the file on that reading.
- A2-1, the order of authority, is a ruling he delegated to a trace of the record. Rows written from it are ratified as ruled, and the two rows on the copies of the standalone spec are in the file so he can say whether the delegated trace stands.
- Two class a rows rest on a sketch that relays his standing ruling from agent memory with a phrase of his quoted (`how_the_dispatch_order_is_assembled_...`, `registration_order_as_the_dispatch_order_...`). One rests on a topic recorded "by user-led design conversation" (`a_consumer_type_is_itself_the_builder_input_...`). They are ratified on the reader's reading; a stricter reading would stamp them.
- The rows written from his answers of 13 November (the question rows, the two bench rows, the dispatch question) were checked against the captures by me alone.
- The `mockspace.toml` changes are the registry kinds and fields and `canon_paths`. No lint severity was touched.
- The local tag `round/202611130854/end` was made by `cargo mock close` and is not pushed.

## Tickets

f237426a646a, ff77d8b3eac7, 7f45d5e2d5c3, e2d3389d6ad0, 24200d58559b, e014a0e6fde3, ad4a1d3509f1, ae7c4e4bd31e, 830081446e0f, 8ad7746d61c0, 13a6cd07d776, ac1f77cccc5e, cb05ef01a199, a79bb1232a1e, 2a3c771db7ef, 14a3c13dca08, 1d9d93e0d678, eae80cb241fc, c7547243600a (sonnet seats falling back to opus), cd055bd7d4b2 (`wait-for-slot` counting finished agents); bumped 7dc2e880df64.

## Next step for whoever takes it

1. A fresh `pr-reviewer-senior` over the branch with the brief above, once the usage band allows.
2. Readers over the imported saalis and polka-dots research, then `LEDGER.md` rebuilt.
3. Part four, when `work/tools-upstream` is on the lint pack's `dev`.
