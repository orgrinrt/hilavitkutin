# parked-worker-aliasing-measured

**Date:** 2026-11-14
**Phase:** TOPIC
**Scope:** the first half of job 4 of `mock/research/202611140100_engine-path-plan/PLAN.md`: ticket 93, deviations 1, 5 and 6 of the ledger, measured and put to op on the question ticket `bless_the_scheduler_plane_relocation_that_closes_the_worker`. No repair is built here.

## What may be built

The ratified row `the_gate_2_agent_call_deviations_are_blessed_or_rebuilt_only_on_evidence_put_to_the_maintainer` (`mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:64-71`) says of these deviations that "the canon shape is built or sketched, benched against the shipped shape where the ledger names a bench trigger, and presented to the maintainer with evidence for a ruling to bless it or rebuild it. None is blessed by default and none is rebuilt by default." The rows carrying the relocation, `deviation_five_has_a_confirmed_aliasing_hole_so_the_plane_relocation_is_soundness_required_and_resolves_deviations_one_five_and_six_in_one_round` and `deviations_one_and_six_are_answered_by_the_whole_plane_arena_route_and_the_proposed_ruling_is_the_relocation_round`, are held and bind nothing. So this round builds the drivers and the sketches and changes no engine source; the second half of job 4 builds what op rules.

## What the engine does

A worker forms one shared reference to the scheduler and holds it across every park (`crates/hilavitkutin/src/scheduler/worker.rs:101`). Its comment and the design (`crates/hilavitkutin/DESIGN.md.tmpl:1047-1057`) say this is sound because every between-frame write to a worker-visible field goes through interior mutability, and "a plain write through the scheduler pointer would invalidate the workers' live reference under the aliasing model". The calling thread makes such writes: `(*me).spawned = Bool::TRUE` after the spawn loop (`scheduler/run_parallel.rs:212`), the context of each next core inside it (`:188-192`), and an exclusive reborrow of the bindings at every frame start (`reset_accumulators`, `:223-225`). The swaps take `&mut self` between frames (`scheduler/dirty.rs:49`, `:97`, each with `FIXME(placeholders_tagged_74e2b9918149)`).

## The first measurement

The plan's first seat predicted that two plain frames already violate Stacked Borrows, and nobody had run it. Miri on the existing swap test, `tests/replace_swap_plan_band.rs`, under Stacked Borrows, stops on the first `run_parallel` frame, before any swap:

```
Undefined Behavior: Data race detected between (1) non-atomic write on thread `a_plan_affectin` and (2) retag read of type `hilavitkutin::scheduler::Scheduler<...>` on thread `unnamed-2`
  --> crates/hilavitkutin/src/scheduler/worker.rs:101:21
  --> crates/hilavitkutin/src/scheduler/run_parallel.rs:212:16
```

The worker's retag of the whole scheduler is a read of every byte it covers, and the `spawned` write races it.

## The sketch

`mock/research/sketches/202611140610_parked-worker-plane/` builds a miniature of the scheduler four ways (today's held reference, the narrower repair of a raw pointer across the park with a reference per frame, the relocation to a provider-allocated plane, and the canon shape of a per-core program) and runs the plan's three drivers on each, under Stacked and Tree Borrows, with and without the race detector. `today` fails every driver with the race, and with the race detector off Stacked Borrows still refuses it, at the frame start's `&mut` to the bindings; Tree Borrows accepts that part. The other three arms pass everything. The narrower repair passing is against both seats' argument, and the sketch says why: the scheduler is `!Unpin` (`scheduler/mod.rs:313`), so its `&mut` is not retagged unique, and a raw pointer held across the park keeps the provenance the calling thread writes through. The plane passes only when written whole before the first worker starts. `FINDINGS.md` there has the table and the predicate.

## The drivers

The plan names three, each to run under Stacked and Tree Borrows through a test provider, and to be committed red: two consecutive `run_parallel` frames; a frame, `replace_value`, a frame; a frame, `replace_resource`, a frame. The test the deviation 5 audit cites, `parallel_swap_pattern_is_aliasing_clean`, exists on `origin/recover/unlanded-rounds-and-benches` alone as an ignored body of comments, and is not on `dev`; these drivers stand where it was meant to. They live in `crates/hilavitkutin/tests/parked_worker_aliasing.rs`, assert the frames' outputs natively, and are red only under Miri, so the native suite stays green while Miri says what the engine is.

## What goes to op

The question ticket `bless_the_scheduler_plane_relocation_that_closes_the_worker` gets the evidence: the drivers' run on the engine, the sketch's table, and the three arms that close the hole, with what each costs. The narrower repair is the smallest change and rests on an invariant no type checks (the `PhantomPinned` field, and no reference across the park). The plane moves what workers read into one provider block written before the spawn. The per-core program is the canon shape the registry's open question names beside the relocation (`whether_the_maintainer_blesses_the_plane_relocation_round_for_deviations_one_five_and_six_and_the_bless_of_deviation_four`), and is the larger work the plan places after this track. Undefined behaviour is no design, so whichever op rules, the hole closes in the second half of job 4.
