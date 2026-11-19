# Job report: the stranded branches, the api placement, the red on dev

The job `hila-stranded-and-placement` ran on branch `work/hilavitkutin-stranded-and-placement`, cut from `origin/dev` at `07a9e9a3`. `dev` did not move while it ran, so no merge of `dev` was needed. Four design rounds closed on the branch and three review passes ran, the last ending at merge after one fix, which is made.

## The stranded branches

`mock/research/202611191400_stranded-branches/DECISIONS.md` decides each of the seventeen SOURCE and MIXED branches the workspace note classed, with the scripts that measured them in `scripts/`. Thirteen retire, on `dev` byte for byte at the landing commit the note names for each. Four hold: `work/hv-suite-words`, `work/hv-suite-examples` and `recover/unlanded-rounds-and-benches` add record files whose content is nowhere on `dev`, owed to `dev` consolidated by hand under the rule that a branch's record lands whole; `work/commit-subject-config` waits on mockspace's unlanded `work/commit-type-gates`. No source lands from any of them, and no branch was deleted: retiring the thirteen is `forge-route delete-branch hilavitkutin <branch>` for each.

## The api placement

Op's call of 16 November is recorded in `OP-CALL.md` here. Round `202611191347` moved the items, round `202611191514` and round `202611191546` answered the first two reviews.

| Item | Where it is now |
|---|---|
| `capability`, `sink`, `codec` | deleted; a consumer takes notko's `Push` and `BulkPush` |
| `dispatch_codegen` and its ids | `hilavitkutin::dispatch::codegen`, `DispatchCodegen` and `FiberShape` under the engine seal `EngineOnly` |
| `PoolFrame`, `WakeStrategy` | `hilavitkutin::thread::pool_frame` |
| `Executor`, `ExecutorError`, `HybridExecutor` | `hilavitkutin::thread::hybrid`, `Executor` open as the ratified row `work_stealing_is_not_the_default_and_the_consumer_may_provide_a_stealing_executor` says |
| `CeilingDiv`, `Ceil` | `hilavitkutin::plan::ceiling_div` |
| `StoreMarker` | deleted, read by nothing |
| `MemoryProviderApi`, `WorkUnit` and the unit vocabulary | stay in the api |

Canon: four ratified rows in `tenet/identity-3.toml`, one per option op chose, each in his option text; a held row restating the api's surface, which supersedes the provisional `identity-2.toml` row; the held persistence row superseded by op's own row for it; two `principles-2.toml` rows superseded by held rows that name notko's sink. Every template that taught the deleted sink types, or called persistence standalone, says what the rulings say.

## The red on dev

`hilavitkutin::ui compile_fail_fixtures` is green: the two stderr files are regenerated, line numbers only, the same refused bound.

## Tests

`cargo mock test -p hilavitkutin --full --no-cache` 340 passed, 21 skipped, both heavy ui tests included (the third review's run); `-p hilavitkutin-api --full` 88 passed; `cargo check --workspace --tests` in `mock/` clean; the bench support crates check clean; `cargo mock` lints pass. No red met was dev's. `canon contradicts` over `hilavitkutin-api` and `hilavitkutin` reports nothing. Every compile-fail fixture the branch adds fails for the reason it names.

## Left for the coordinator

Two commit messages want rewording before the landing: the body of `3f0145c8` (it names the maintainer's choice and the blind seats, and says `CeilingDiv` is deleted) and the subject of `923fe3cd`. Rewriting the pushed history was refused here.

## Follow-ups, each a ticket

- `two_ratified_placement_row_ids_carry_glosses_their_quote_doe`
- `does_the_api_surface_sentence_op_left_standing_keep_its_prov` (question, op)
- `may_a_consumer_write_a_hilavitkutin_executor_or_does_the_eng` (question, op; answered by the ratified row, open only if he overturns it)
- `does_an_earlier_draft_of_a_landed_record_land_whole_from_a_r` (question, op)
- `hilavitkutin_s_root_design_and_one_provisional_row_still_wri`
- `the_hilavitkutin_scheduler_takes_no_executor_though_the_desi`
- `catalogue_the_edge_cases_of_ceilingdiv_and_the_engine_id_rou`
- `the_moved_pool_frame_and_executor_text_still_says_substrate`
- `cargo_mock_close_sweeps_an_unrelated_inherited_flat_topic_in` (mockspace)
- `lazy_tool_s_one_lock_makes_a_hilavitkutin_pre_push_wait_on_a`, `the_bootstrap_s_cargo_mock_step_prints_a_cargo_binstall_bran`, `the_homma_hook_regen_passes_skip_surfaces_which_the_installe`, `a_helper_cannot_reword_a_commit_on_its_own_unlanded_branch`, `stand_up_cannot_be_re_run_once_its_job_branch_exists_locally` (workspace)
