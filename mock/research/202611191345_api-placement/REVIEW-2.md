# Review 2: `work/hilavitkutin-stranded-and-placement` (`7b0c0928`) against `origin/dev`

Canon gate: misaligned (findings 1 to 3). Checked against `execution-3.toml:231-249` (ratified), op's words at `OP-CALL.md:7-16`, and op's rule `the-record-lands-from-every-branch.md`. I am the second expert on these canon calls. My read is not blind: the brief had me read REVIEW-1 first. Test gate: `cargo mock test --cheap` exit 0 (379 passed and 16 skipped in the members, 142 more in the bench and tool trees). I read every test body the diff adds or moves.

## Verdict

MERGE AFTER FIXES. Findings 1 to 5 are blockers.

## First-pass findings, against the tree now

1. Fixed for the instance only. `CeilingDiv` is in `plan::ceiling_div` and row `:406` says so. The class it named is still open: see findings 1 and 2.
2. Fixed. Both rows are superseded by held rows in `principles-4.toml`.
3. Fixed. No template names a deleted sink type.
4. Fixed in wording. The em-dash on the line above the rewritten words is still there (finding 6).
5. Open, handed to the coordinator. The `3f0145c8` body also says "`CeilingDiv` ... deleted", which is false at the tip. One more reason to reword it.
6. Wording fixed at `DESIGN.md.tmpl:729`. The question ticket misstates the canon (finding 2).
7. Fixtures added. They do not test the seal (finding 4).
8. Fixed. `UnitId` and `FiberId` use hard-coded widths (finding 7).
9. Scripts committed. The record question asks something op's rule already answers (finding 3).
10. Fixed.

## Findings

1. `mock/registry/tenet/identity-3.toml:368,391,408` [spec, blocker]: REVIEW-1's Notes said to fix the class. Round 202611191514 fixed only the one instance REVIEW-1 named, so the defect is still there in three ratified rows. Each one attributes agent text to op's "all four chosen":
   - `:391` holds the count-type residue reading. `OP-CALL.md:22` itself marks that reading "the round's and not his words", and `:393` cites `:22` as the source.
   - `:408` adds "sealed", "its one implementation" and "the engine and the providers take".
   - `:368` lifts the provisional row's AccessSet, store types, context, Clock and ThreadPool into ratified canon. Op's option text said "This supersedes one sentence of a provisional row" (`OP-CALL.md:12`), not the whole row.

   Fix: cut each `what` and `mechanism` down to op's option text. Supersede only the capability, sink and codec sentence of `identity-2.toml:196`. Drop `:22` from every `source`.
2. `mock/crates/hilavitkutin/tests/ui/executor_outside_the_engine_fail.rs` and `DESIGN.md.tmpl:729-732` [spec, blocker]: the ratified row `execution-3.toml:231` says "the consumer can provide a stealing executor through the executor trait". This branch pins the opposite as intended behaviour: the fixture's type is literally named `StealingExecutor`, and the ratified row `:408` writes in "sealed". Ticket `may_a_consumer_write_a_hilavitkutin_executor_or_does_the_eng` tells op "Nothing ratified settles which holds", which is false. Fix: correct the ticket. It should cite `execution-3.toml:231` and ask whether op overturns the ratified row. Until he answers, mark the fixture and the design sentence `FIXME(<ticket>)` as contradicting ratified canon.
3. `mock/research/202611191400_stranded-branches/DECISIONS.md:21,22,27` [rules, blocker]: op's rule says new record files land "without exception", and that two branches adding one path with different content are "consolidated by hand". The note still decides "retire" for three branches whose record blobs are not on `dev`, and the workspace ticket `does_an_earlier_draft_of_a_landed_record_land_whole_from_a_r` asks op about a case the rule already settles. Fix: change those three rows to land their record (consolidated), or at least to hold, citing the rule.
4. `mock/crates/hilavitkutin/tests/ui/*_outside_the_engine_fail.rs` [tests, blocker]: none of the three fixtures names `hilavitkutin::sealed`. Make `mod sealed` `pub` at `src/lib.rs:58` and all three still fail with byte-identical stderr, so the one change that breaks the seal stays green. Fix: add a fixture that names `hilavitkutin::sealed::EngineOnly` and expects E0603.
5. `mock/PRINCIPLES.md.tmpl:172` and `principles-4.toml:129,146` [public-surface, blocker]: they name `KeyValueStore` in `hilavitkutin-api`, which exists nowhere in the tree. This is the same class as REVIEW-1's finding 3, inside the rows this round wrote. Fix: drop `KeyValueStore`, leaving `Map<K, V, N>`.
6. `mock/Cargo.toml:33` [rules]: an em-dash, at the head of the comment REVIEW-1 asked to rewrite. Fix: use a colon.
7. `mock/crates/hilavitkutin/tests/engine_id_index.rs` [tests]: the loops use hand-written `1usize << 16` and `<< 7` while `PhaseId` and `TrunkId` use `ADDRESSABLE`, so the walk does not follow a width change. Out-of-range `from_index` is not catalogued. Fix: add `ADDRESSABLE` to `UnitId` and `FiberId` and loop over it. Add a test for index `ADDRESSABLE`.
8. `mock/crates/hilavitkutin/src/plan/ceiling_div.rs:17,21` [tests]: `D = 0` is not covered (`div_ceil` panics during const evaluation). The doc says `(N + D - 1) / D`, which overflows near `usize::MAX`, while the code does not. Fix: add a trybuild fixture for `D = 0` and document it as `N.div_ceil(D)`.
9. `mock/registry/tenet/plan-3.toml:212-213` [drift]: the source points into closed round `202611191347`, which the re-file note says "is not its round". The active topic at `mock/design_rounds/202611191521_topic...md` lost its `**Phase:**` line. Fix: point the row at the flat topic and restore the line.

## Verified

- `PoolFrame`, `WakeStrategy`, `HybridExecutor` and the `Send`/`Sync` impls with their `// SAFETY:` moved verbatim (`pool_frame.rs:123-131`).
- `EngineOnly` sits in a private module (`lib.rs:58`), and the api re-exports nothing that has moved.
- The sweep: attribution is clean, and the PR text follows the conventions. Its three JUDGE items on `hilavitkutin/Cargo.toml` features are outside this diff.
- The `cargo mock canon` subjects resolve. Superseded rows on `dev` carry no stamp, so dropping the stamps follows the registry's practice.

## Notes

This is the second time round the same failure: op's answer is applied where it is convenient and agent text rides into ratified rows on his quote. Before a third pass, grep every `standing = "ratified"` row this branch adds for words that are not in its `quote`.
