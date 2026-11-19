# Review 1: `work/hilavitkutin-stranded-and-placement` against `origin/dev`

Canon gate: misaligned on one point (finding 1), otherwise checked against `mock/registry/tenet/identity-3.toml` and op's words in `mock/research/202611191345_api-placement/OP-CALL.md:7-16`, which match clause-dev `57-op-calls-on-the-night.md` section 3 word for word. I am the first expert on these canon calls, so a second read is owed. Test gate: `CARGO_INCREMENTAL=0 cargo mock test --cheap` is green on the whole tree (690 run, 690 passed, 35 skipped; 142 more in the bench and tool trees; `ui compile_fail_fixtures` passes).

## Verdict

MERGE AFTER FIXES

## Findings

1. `mock/registry/tenet/identity-3.toml:409` [spec]: this row is `standing = "ratified"`, and its quote is op saying "`ceiling_div` move into the engine". Its mechanism says "`CeilingDiv`, read by nothing, is deleted rather than moved". OP-CALL.md:22 admits the deletion is "the round's and not his words", and then the deletion went into a ratified row anyway. That turns an agent call into ratified canon. It also falsifies the stamped row `plan-3.toml:336` ("`CeilingDiv` provided as a recommended heuristic"), which nobody superseded. Fix: move `CeilingDiv` into the engine as op said and drop the clause. If deleting it is still wanted, ask op with a question ticket.
2. `mock/registry/tenet/principles-2.toml:342-374` and `:457-477` [spec]: two stamped rows still name `Collector<T>`, `DiagnosticSink` and `ByteEmitter` as "hilavitkutin-api contracts". This branch deletes all three, and notko only has `Push`/`BulkPush`. Fix: supersede both rows with a row that names notko's sink.
3. `mock/PRINCIPLES.md.tmpl:64-65,171` (renders `docs/PRINCIPLES.md:73-74,180`), `mock/agent/rules/cookbook.md.tmpl:27,34,39`, `no-heap.md.tmpl:20-21`, `type-surface.md.tmpl:77,105`, `mock/agent/skills/type-surface-review/SKILL.md.tmpl:32,53` [public-surface, blocker]: the generated docs and agent rules still tell readers to use the deleted sink types. Fix: repoint every line to `notko::sink`, or drop the line where notko has nothing.
4. `mock/Cargo.toml:33-36` [public-surface, blocker]: the comment says the api uses notko for "`BoundedPush`, `Decoder`", and both are deleted. Fix: say what the api uses notko for now.
5. Commit `3f0145c8` body [rules, blocker]: "Op chose ... the placement two blind seats agreed on" describes how the change was made, in a public repo. Fix: reword the body before landing to say what changed and why.
6. `mock/crates/hilavitkutin/DESIGN.md.tmpl:729` [naming]: "Open `trait Executor: Sealed`" contradicts the next sentence (`EngineOnly`, "cannot be written" outside the engine). Fix: "Sealed `trait Executor: EngineOnly`". Two other standing claims also still say consumers can implement the executor: `execution-4.toml:428` ("an open trait") and `mock/agent/MAIN.md.tmpl:68` ("consumers may implement the `Executor` trait for stealing"). Whether consumers may write an executor is a question for op. Nothing ratified settles it, and the branch decided it silently in the design text.
7. `mock/crates/hilavitkutin/src/lib.rs:129-135` [tests]: `EngineOnly` is a new seal, and no compile-fail test shows that a crate outside the engine cannot implement `Executor`, `DispatchCodegen` or `FiberShape`. Fix: add a trybuild fixture per trait under `tests/ui/`.
8. `mock/crates/hilavitkutin/tests/engine_id_index.rs:87-113` [tests]: the moved test checks a hand-picked sample of each id's range (0..32 to 0..65536) when the whole range is cheap to check. Fix: loop over the full range.
9. `mock/research/202611191400_stranded-branches/DECISIONS.md:5` [drift]: the scripts that did the measuring "ran from a scratch directory" and were not committed. Fix: commit them beside the note. Separately, `the-record-lands-from-every-branch.md` says "new files are taken whole", and this note retires branches that still hold record blobs found nowhere on `dev`: `work/hv-suite-words`/`-examples` (1 changelist draft) and `recover/unlanded-rounds-and-benches` (37 bench variant files). Whether earlier drafts of a landed record are exempt is a question for op.
10. `mock/crates/hilavitkutin/src/plan/mod.rs:14` and `mock/benches/codegen_dispatch_axis_a_assertions.md:11` [drift]: they name the deleted `CeilingDiv` and the old `hilavitkutin-api/src/dispatch_codegen.rs` path. Fix: repoint both.

## Verified

- Every claim in DECISIONS.md checks out against git: 10 landing commits are on `dev` with 0 paths differing over each branch's paths, the ancestries and the identical `canon-registry` tree hold, the record-blob counts (16/16, 1, 37) hold, and the hold on `commit-subject-config` is right (`9b9c444e` is not on `dev`).
- `dispatch_codegen.rs` moved by rename (93% similar). The only changes are the seal name, the `RunCfg` path and the deleted `StoreMarker`, which nothing read.
- `PoolFrame`/`WakeStrategy` and the `unsafe impl Send/Sync` moved verbatim, and the `// SAFETY:` comment came with them.
- The api has no re-export left behind, and `lint-forbidden-hilavitkutin-api` still holds.
- The superseded `identity-2` row and the `persistence.toml` row follow the registry's convention for superseded rows. The `plan-3` citation repoint resolves.
- The deleted `sink_smoke.rs` and `codec_smoke.rs` only tested the deleted modules, so nothing with a reason to exist was lost.
- The two ui stderr files changed line numbers only, and the fixture passes.
- No bylines and no em-dashes in the commits. `refactor!:` is allowed by the repo's `writing-style.md`.

## Notes

Findings 1 and 2 are the same mistake: op's ruling was applied where it fills a gap and left out where it falsifies a standing row (`a-governing-claim-is-applied-where-it-hurts.md`). Run a grep for every deleted item name across `mock/registry` and `mock/**/*.tmpl`, then fix every hit.
