### 4. The aliasing hole, measured and put to op

Ticket 93, deviations 1, 5 and 6 of the ledger. Crates: `hilavitkutin` tests and `mock/research/sketches/`; after op's ruling, `hilavitkutin` (`scheduler/`, `thread/`) and `hilavitkutin-api` if `PoolFrame`'s placement moves (it is in `platform.rs:267`; if so the job touches that file too).

The job has two halves, because the ratified evidence-then-bless row above governs these deviations. The first half is the evidence and builds no repair: the Miri drivers, then a sketch of each arm, the relocation to a provider-allocated plane and the canon shape, the spec's per-core program holding its own ranges and sync points, each run under the drivers. That evidence goes to op on `bless_the_scheduler_plane_relocation_that_closes_the_worker`. The second half builds what he rules, and does not start before.

The Miri drivers come first and are committed red: two consecutive `run_parallel` frames; a frame, `replace_value`, a frame; a frame, `replace_resource`, a frame (the driver of `tests/replace_swap_plan_band.rs`, by the seats' reading at `:252-330`). Each runs under Stacked Borrows and under Tree Borrows, through a test provider and not raw `mmap`.

The cases that must fail: the two swap drivers under Stacked Borrows on today's tree. The first driver's result is recorded either way, since it says whether a plain second frame is already a violation. A sketch of the narrower repair runs under the same drivers: if it passes, both seats' argument against it is wrong, and that goes to op with the rest.

The test the deviation 5 audit cites, `parallel_swap_pattern_is_aliasing_clean`, is on neither `dev` nor the api revision. It exists on `origin/recover/unlanded-rounds-and-benches` alone (`tests/replace_swap.rs:278-297` there), ignored, with a body of comments and no assertion, so it cannot fail. These drivers replace it, and if that stub reaches the trunk before this job the job deletes it.

Frees: op's ruling on the deviations, and after it a sound base for the one body. The swap's ratification does not wait on this job; op tied it to the two gaps alone, jobs 6 and 7.

### 5. The one frame body
