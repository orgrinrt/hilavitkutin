Source: kaski channel (coord/kaski of clause-dev), att/20261113T150306627148807-hv-canon-registry-169f30/report-hv-canon-registry.md

# hv-canon-registry: report

Branch `work/canon-registry` of `orgrinrt/hilavitkutin`, 41 commits over `dev`, head `791c837819d3309cc9ae8e63fd6c592403e86ba9`. `cargo mock` exits 0 on the head with the `workflow` feature on. `dev` is merged in (`f254a3d`, clean). Nothing was pushed to `dev` and no pull request was opened.

## Part four, the tools (round `202611131444`)

`cargo mock canon find|subject|subjects|related|check`, `already-said` and `unearth` work in hilavitkutin. They come from `mockspace-extra-lints` `dev` (18ae1e9) by `features = ["workflow"]` on the `[lint-crates]` entry. No tool was copied in.

What each answered when run on the head:

- `canon subjects`: 17 subjects with their tenet counts, "read 841 tenets and 17 subjects".
- `canon find morsel`, `canon subject governance`: the rows, provisional ones marked.
- `canon related the_engine_is_one_and_the_core_count_is_configuration`: the rows that relate to it and its subject.
- `canon check <ratified id> <ratified id> no_such_row_here`: exit 1, naming the missing id only.
- `already-said "Land as provisional"`: 0 of 961 rows, 7 of 1949 documents, nearest his capture of 13 November and the two swap topics. It reads `../.data/op-responses` and skips the 1221 captures that do not name hilavitkutin.
- `unearth "double buffering"`: five documents with a verdict, 1234 more matched.

The feature is one switch and brings seven registry lints at error. Four fired, 4563 findings. None was downgraded or exempted; `mockspace.toml` gained no `exempt_crates` and no severity line. How each was met:

| lint | findings | what was done |
|---|---|---|
| `a-retired-word-stays-out` | 4172 | The readers had written prose in `rules_out` (238 entries, 137 rows) and the lint reads it as words banned from the tree. Every entry moved unchanged to a new field `rejects`. `rules_out` is empty on every row. |
| `a-tenet-cites-what-its-words-name` | 371 | `cites` may hold only rows the `what` and `because` name by id. The 450 entries moved to `related` as `tenet::` links. None dropped (450 + 43 before, 493 after). |
| `a-tenet-names-a-subject` | 2 | A false finding: the lint reads files by line and a quote line opening with a bracket hid `subject`. `subject` now sits beside `id` in every row. |
| `every-fixme-names-a-ticket` | 18, then 23 after the merge | `cargo mock owed` filed 11 tasks under `mock/registry/ticket/items/` and rewrote the markers to cite them. Comments only. |

### Calls I made that a reader should weigh

1. `rules_out` empty everywhere. The record does retire words (the dead terms of the principles, some retired names). Keeping them in `rules_out` gives about 1600 findings on ordinary uses of `order`, `row`, `entry` and `chain` across designs and code, the only ways through are an exemption list or clearing the tree, and I did neither. The dead terms stay held by the repository's own `vocabulary-discipline` lint, at warn as before. The work of naming retired words row by row with the tree cleared is ticket 6 below (17f90baf8b23). The other reading is that this lets a gate arrive holding nothing.
2. One ratified row's `what` gained two row ids. `already-said` refuses to answer while no row carries `cites`. `the_evidence_then_bless_standard_is_part_of_canon_...` says it governs the GATE-2 deviations and the spectral role deviation, as its passage does, and now names the two rows that state those in parentheses. It says nothing new. It is an edit to make a tool run, and ticket 1 (ef03a0834389) asks the pack to stop needing it.
3. hilavitkutin now has a ticket kind of its own. The feature cannot be had without it, and the marker lint looks tickets up in the repository's own registry. Ten markers cited eight tickets of the workspace tracker; they now cite hilavitkutin tasks named `placeholders_tagged_<workspace id>`. Two trackers for hilavitkutin work is ticket 8 (f9ce88409aab).
4. The `[tools.*]` tables name `../.data/op-responses`, a path out of the repository into the workspace. In a clone that stands alone the tools say it was not read.
5. The pack's entry is renamed to `mockspace-extra-lints.git` (it named the repository's old name, which redirects).

### The merge of `dev`

`dev` had moved by seven commits (the recovered engine source, a stack audit, the swap landing). I merged it before the round with `merge-dev`. 21 cited passages moved and each citation was moved after its lines, checked by refilling every quote from its source. The trunk rewrote the text under four rows; they keep their lines and name `c5f337a` in `rev` (ticket 7, 478634f8f67b). One of them matters to the ratification pass: the seed now records that the swap spec landed under op's provisional stamp of 13 November, "Land as provisional", with S1 to S7 not ratified. The 25 swap rows in `ROWS-FOR-OP.md` were written before that and are untouched; their lead paragraph still says the spec waits on him.

The doc changelist was unlocked twice to correct a count and a claim before the source changelist was written. The local tags `round/202611130854/end` and `round/202611131444/end` are not pushed.

## Parts one to three (unchanged since `c5f337a`)

- Part one, the read: `mock/research/202611130850_the-record-read-whole/LEDGER.md`. Not read: the research imported from polka-dots and saalis (reader batches 39 to 44 never ran).
- Part two, the registry: 903 rows (841 tenets, 62 questions), each tenet with `source` and a `quote` filled from those lines by script.
- Part three: `ANSWERS.md`.
- His words of 13 November: `OP-ANSWERS.md`, `-13`, `-14`, `-18`, `-20`, `-21`. `RECOVERED.md`, `IKIUNI.md`.

## The ratification pass (coord, 12:04; op's words in OP-ANSWERS-18.md)

Every row was classed by a reader against the record: 18 reader runs on sonnet high, one per slice, one neutral brief, every cited range checked by script.

| class | rows | written as |
|---|---|---|
| a, his | 375 tenets | `standing = "ratified"`, `ratified_by`, `ratified_from` (kind and passage) |
| a, replaced later | 8 tenets | provisional, stamp naming the later passage of his and its row |
| a, questions | 24 already his answers, 3 answered from the founding spec and A2-3 with `answered_from`, 1 open on a stamp | |
| c, neither | 352 tenets, 9 questions | `stamp`; 9 of the tenets are his only in part and the stamp names the passage |
| b, not backed or against | 106 tenets, 25 questions | untouched; listed in `ROWS-FOR-OP.md` (131 rows, four parts) |

By kind, the ratified tenets rest on: the founding spec or the principles (318), an amendment that says he ruled it (22), a passage quoting him or saying he chose it (25), another passage saying he ruled the same content (7), the founding spec stating the same point elsewhere (3).

What I opened myself: every class a row not from the founding spec and not already his own answer was read a second time (68 rows, 34 by a second reader, 34 by me); 12 came back his only in part, 9 moved to a stamp, 3 stayed ratified (the two `op_ruled_...` rows of 18 September and `a_resource_value_is_a_one_record_blob_...`). I opened 21 founding rows and every `later:` passage of slices 01 to 04, read every class b line and moved four.

What a reader should know:

- No reviewer has read the pass; the coordinator runs one. The brief is attached to this ready (`review-brief.md`, placeholders `@@WORKTREE@@`, `@@SHA@@`, `@@REPORT@@`), with part four added as item 4b.
- Three sonnet-high seats came back having run on opus (c7547243600a).
- Slices 01 to 04 were classed before the brief carried the paragraph on the bare-engine benches; I found no row among them resting a call on those benches.
- The convergence bench and perf-gate rows are in part two of the file on the coordinator's reading of OP-ANSWERS-21.
- A2-1 is a ruling he delegated to a trace; rows from it are ratified as ruled, and the two rows on the copies of the standalone spec are in the file so he can say whether the trace stands.
- Two class a rows rest on a sketch relaying his ruling from agent memory (`how_the_dispatch_order_is_assembled_...`, `registration_order_as_the_dispatch_order_...`), one on a topic recorded "by user-led design conversation" (`a_consumer_type_is_itself_the_builder_input_...`). A stricter reading would stamp them.
- The rows written from his answers of 13 November were checked against the captures by me alone.

## Tickets

Part four, in the order the report numbers them: 1 ef03a0834389 (already-said and a registry with no cites), 2 d6bd50767b86 (the subject lint and a bracketed quote line), 3 0a2e5e72dbee (a list entry holding a comma is split), 4 f600f6dd83f8 (the garbled captures directory in the pack), 5 3d98467d6048 (owed rewrites a prose mention), 6 17f90baf8b23 (the retired words and the tree), 7 478634f8f67b (four rows whose passages the trunk rewrote), 8 f9ce88409aab (where hilavitkutin tickets live), 9 d2dfc55ffe48 (the generated-file hook and a command that only runs a skill script).

Earlier: f237426a646a, ff77d8b3eac7, 7f45d5e2d5c3, e2d3389d6ad0, 24200d58559b, e014a0e6fde3, ad4a1d3509f1, ae7c4e4bd31e, 830081446e0f, 8ad7746d61c0, 13a6cd07d776, ac1f77cccc5e, cb05ef01a199, a79bb1232a1e, 2a3c771db7ef, 14a3c13dca08, 1d9d93e0d678, eae80cb241fc, c7547243600a, cd055bd7d4b2; bumped 7dc2e880df64 and, in part four, `ticket_add_and_drop_take_over_two_minutes_on_the_workspace` (nine single adds took about 25 minutes).

## Next step for whoever takes it

1. The reviewer over the branch (the coordinator's).
2. Ticket 7: rows from the trunk's new text for the four pinned rows, and the swap group of `ROWS-FOR-OP.md` read against "Land as provisional".
3. Ticket 6: the retired words, with the tree cleared in a round.
4. Readers over the imported saalis and polka-dots research, then `LEDGER.md` rebuilt.
