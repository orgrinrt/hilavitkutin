Source: kaski channel (coord/kaski of clause-dev), att/20261114T063922449291946-hv-suite-surface-d02eed/report-hv-suite-surface.md, first posted 2026-11-14 06:39 UTC, read 2026-11-15.

# hv-suite-surface report

Branch `work/hv-suite-surface` on `orgrinrt/hilavitkutin`, tip `4081a818`, pushed. It merges `origin/dev` at `ade63db9`, so it is not behind `dev`.

## Rounds

Three rounds, all closed:

- `mock/design_rounds/202611140459/`: the suite as an instrument.
- `mock/design_rounds/202611140533/`: the api surface says what ships.
- `mock/design_rounds/202611140628/`: suite claims hold on any machine. It carries the final review's fixes.

`202607200100_topic.b1-cap-lifting-completion.md` stays flat, since it is the engine lane's. Each `cargo mock close` swept it into the closing round's directory, and I moved it back by `git mv`. That defect is ticket `b9507733282e`.

`cargo mock finish HEAD` read 21 rows and reported nothing. `cargo mock owed` closed three tickets that no marker cites any more:

- `e3cb3f33eb38`: the dispatch codegen markers went.
- `559df4cb0767`: the examples are now running programs.
- `35e2aa2b5329`: no marker on `dev` either.

## Review

The local review ran two passes. Pass 2 was the last one and returned merge after fixes. Every finding is fixed or ticketed:

- 1, the comment lint: fixed. `round` now counts only as the noun, block comments are tracked across lines, and char literals are skipped. 15 lint tests pass. The live tree now has 45 findings, all in the engine lane's `dispatch/`, `plan/` and `scheduler/`. These are part of the round words already handed to the engine lane.
- 2, the GATE-2 comments in `thread/frame.rs` and `resource/bindings/per_core.rs`: fixed. The same forms remain in the engine lane's files, and they go with that hand-off.
- 3, `r5_core_idle_accum`: fixed. The walking-core counts are now given by hand as 4 and 32. A control run with 40 in place of 32 fails as it should.
- 4, `r5_core_idle_parallel`: fixed. It runs at `FixedWorkers::<4>` and `FixedWorkers::<1>`, so both arms run on every machine.
- 5, the second error in `undeclared_read_fail.stderr`: this is the engine's `ColSelector` resolution, so it is ticket `85819d75aea5` for the engine lane.
- 6, the EMA stub notice: fixed. It is plain prose now, and the stub's task `hilavitkutin_providers_2_unmarked_placeholders` names the NEON sketch.
- 7, the examples read results back through hidden positional bindings: ticket `c92ba860c672` (no public read-back).
- 8 and 9 are history and stand:
  - A closed topic in `202611140459` cites an uncommitted scratch log, so its "24 warnings" figure carries no weight.
  - Three merge commits have git's default subject. `acaddd0a` is the 84-character one.

## Test runs

I ran `env CARGO_INCREMENTAL=0 cargo test --workspace --no-fail-fast` in `mock/` on `4081a818`. All 168 test binaries passed: 710 passed, 0 failed, 33 ignored, and each ignore names a ticket. Because nothing was red, there were no reds to check on `dev`.

Other checks:

- `env CARGO_INCREMENTAL=0 cargo check --all-targets` in `mock/` prints no warning.
- Every trybuild fail fixture refuses with matching stderr. The reviewer counted 6/6 in the engine's `tests/ui` and 10/10 in the api's `tests/trybuild`.
- The three examples run:
  - "word_count: 12000 words, 2 frames, alpha 6000 beta 3000 gamma 2000 other 1000"
  - "audio_mix: 20000 records, 3 frames, mix checked, peak 294912"
  - "particle_collision: 10000 particles, 12 frames, 7500 reflected"
- The frozen-authors tests: 18 pass, 9 ignored, with each ignore naming its ticket.
- No `allow` attribute was added and 48 were removed. No ratchet, exemption or suppression was added.

## Canon contradicts

`cargo mock canon contradicts` reports 0 lines both before (by `git grep` of the ruled-out words on `origin/dev`) and after, over these paths:

- `hilavitkutin-api`
- `hilavitkutin-api-macros`
- `hilavitkutin-kit`
- `hilavitkutin-providers`
- `hilavitkutin-frozen-authors`
- `hilavitkutin/tests`

## Still wrong

- Jobs 7/66 (generic_const_exprs in `plan/core_program.rs`) and 30 (dead ids) wait on the engine lane. DMs `34abcf`, `13feb3` and `da8fed` are unanswered. The engine lane also has the following:
  - the carriers (98) and the `standard.rs:4` doc line
  - HybridExecutor (25)
  - the process words in its directories
- The comment lint is advisory and gates nothing until the engine lane is clean. Raising it to error is ticket `dea89d9a7088`.
- `.worktrees/hv-suite-fixes` is left for the reaper. Its staged content equals `8fb145ec`, and the forced removal was refused.
- One commit was not re-authored for the Stop hook's noreply demand. The branch's commits use the workspace identity.
- A second independent canon read of the AdaptArena removal, owed from review pass 1, was not run.

## Tickets

Filed:

- Questions:
  - `6e38ea341af3`: spec copy
  - `cc28b4b881a6`: interner reads and dedup
- Tasks:
  - `dea89d9a7088`
  - `3ede4f8bbf63`
  - `50daae225271`
  - `76d94ddad008`
  - `c92ba860c672`
  - `85819d75aea5`
  - `b9507733282e`
- Registry: `hilavitkutin_providers_2_unmarked_placeholders`

Earlier: `2f880830ddc3`, `f00fb146a68c`.
