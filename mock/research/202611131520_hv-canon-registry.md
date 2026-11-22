Source: kaski channel (coord/kaski of clause-dev), att/20261113T152052614915669-hv-canon-registry-8cf91e/report-hv-canon-registry-handoff.md

# hv-canon-registry: handoff at the week's wrap-up

`weekly-usage` reads 99%, band wrap-up, and `usage-gate` exits 1. The review of 791c837 came in at 15:16 with fourteen findings. None of them is started: the work is hours long (finding 2 alone clears the tree of retired words in a round) and the band says a helper in flight wraps up. This seat stops here with everything pushed.

## State

- Branch `work/canon-registry` of `orgrinrt/hilavitkutin`, head `c865900df0cef012636ddfa7df6b922d9bb60a1e`, pushed, tree clean. 42 commits over `dev`. `cargo mock` exits 0 on it.
- Landed: nothing. The branch is not on `dev` and must not land before findings 1 to 6.
- `c865900` is one commit over the reviewed `791c837`: it adds `mock/research/202611130850_the-record-read-whole/scripts/` (four scripts, no row changed).
- Not pushed: local tags `round/202611130854/end` and `round/202611131444/end`.
- No design round is open. The two parked flat topics (`202607200100_topic.b1-cap-lifting-completion.md`, `202607281547_topic.adopt-the-shared-hlist.md`) sit flat in `mock/design_rounds/`; `cargo mock close` sweeps them into the round directory and they have to be moved back before the close commit.
- The review is the attachment of coord's message of 15:16 (`hv-ratify-review.md`). The report of part four is the attachment of `ready 791c837` (message 169f30), the review brief the reply to it.
- Tickets of part four: ef03a0834389, d6bd50767b86, 0a2e5e72dbee, f600f6dd83f8, 3d98467d6048, 17f90baf8b23, 478634f8f67b, f9ce88409aab, d2dfc55ffe48; bumped `ticket_add_and_drop_take_over_two_minutes_on_the_workspace`.

## The scripts, run from the clone's root

- `scripts/fill-rows.sh <registry toml>...` refills `quote` and `provenance` from `source`. Run on a copy and diff to find a cited range that no longer holds its text.
- `scripts/reanchor-map.sh <old commit> <ranges file> <changed files>` and `scripts/reanchor-apply.sh <map> <file>...` move cited `path:a-b` ranges after a merge. The apply rewrites a matching token inside a quote line too: diff afterwards and put quote lines back.
- `scripts/workflow-shape.sh` puts rows in the shape the registry lints hold (`cites` only what the words name, the rest to `related`; `subject` beside `id`). It also renames `rules_out` to `rejects`: take that arm out before finding 2 fills `rules_out`, or the script moves the new entries away.

## Next steps, in the order I would take them

1. Finding 12 and 13 first, since every later commit sits on them. Merge `origin/dev` (81c5c58, adds files only, the reviewer found it clean) with a subject of the form `chore: merge dev into the registry branch`. `merge-dev` writes git's default subject, so either pass a message or file a ticket on the tool. Rewording the old merge `f254a3d` means rewriting pushed history on the branch: coord's call whether that is wanted or whether the rule is met from here on.
2. Coord's call 1 with finding 3: rows that are provisional, stamped or listed for op must not read as binding. Read how the pack's `canon` tool decides "binding" (`mockspace-extra-lints`, `crates/tools/canon`) and which `standing` values the pack's `tenet` kind allows (`registry/namespaces.toml`) before choosing between narrowing `canon_paths` and a standing value. Part-one rows of `ROWS-FOR-OP.md` (lines 13 to 195) get `superseded_by` the row holding his word. Show `canon find`, `canon check` and `canon subjects` on one provisional and one for-op row.
3. Finding 1, 4, 5, 6: the rows named there move class. Then the class: every ratified row against each later passage of his on its subject, with the count stated. Finding 6 needs coord's or op's word on whether the A2-1 trace is his ruling; the review hands it back.
4. Coord's call 2 with finding 2: fill `rules_out` on the ratified rows that retire words, as a phrase or an identifier and never a bare common word, and fix every site the lint finds, in a design round (designs and source both change). Known from the first run with the readers' prose in the field: `order` 529, `row` 429, `chain` 320, `entry` 241, `partition` 83, `entity` 17, `usize` 840, `Columns` 258, `Resources` 151, `dyn` 58. The lint excuses a use within 3 lines of the ruling tenet's id. An entry holding a comma is split into several terms (ticket 0a2e5e72dbee), so no entry may hold one until that is fixed.
5. Finding 7 and ticket 478634f8f67b: rewrite the four `rev` rows from the tip's text or supersede them. Finding 8: the swap group of `ROWS-FOR-OP.md` against "Land as provisional".
6. Finding 9: the changelist says 238 `rules_out` entries, the reviewer parsed 207. My count came from the lint's own split of entries on a comma and a space, so it counted split parts. The round `202611131444` is closed; correcting a closed round's changelist is a deprecation in a new round, not an unlock.
7. Finding 10: say on the row that the two ids are there for the tool and go when ef03a0834389 is fixed, or take them out and accept that `already-said` answers INCONCLUSIVE until then.
8. Finding 11: copy the four answers from `.data/op-responses/` verbatim into the round's OP-ANSWERS files and cite the copies. Finding 14: a `[tools.quote-check]` key naming `source` if the tool reads one (`mock help quote-check`), else a ticket.
9. Rounds 202611130153-yegv, 202611130355-yrri and 202611130454-z2b4 from the merge, and the polka-dots and saalis imports: read them into the registry or name them unread in `LEDGER.md`.
10. `cargo mock`, push, `ready` with the full sha.
