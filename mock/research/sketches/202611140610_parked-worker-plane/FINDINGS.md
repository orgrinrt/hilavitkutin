# Which parked worker survives Miri

Job 4 of the engine plan asks how wide the aliasing hole of a parked `run_parallel` worker is, and whether each repair the seats and the ledger name closes it. The engine cannot answer the second half without building a repair, which the ratified evidence-then-bless row forbids before op rules, so this sketch builds a miniature of the scheduler four ways and runs the plan's three drivers on each under Miri. The miniature keeps what the engine does on the calling thread: `!Unpin` and pinned, a pool spawned on the first frame with plain field writes during and after the spawn, a `&mut` reborrow of the bindings at every frame start as `reset_accumulators` takes, `replace_value` and `replace_resource` through `&mut self` between frames, and a `Drop` that waits every worker out. `src/lib.rs` names the engine line each of these stands for.

The arms differ in what a worker holds across the park and where what it reads lives. `today` holds a shared reference to the scheduler across every park (`worker.rs:101`). `narrow` holds a raw pointer across the park and forms the reference per frame after the wake. `plane` moves the frame words, the per-worker contexts and everything a worker reads to one provider block, and the worker holds a shared reference to that across the park. `program` gives each core its own provider-allocated program, immutable after the first frame, with its record range, its bindings and a pointer to the shared sync words.

The outcome, every arm and driver alone, in `miri-matrix.txt` (verdicts) and `miri-matrix-full.txt` (Miri's output):

| Arm | Stacked Borrows | Tree Borrows | SB, race detector off | TB, race detector off |
|-----|-----------------|--------------|-----------------------|-----------------------|
| today | FAILS, data race | FAILS, data race | FAILS, aliasing | WORKS |
| narrow | WORKS | WORKS | WORKS | WORKS |
| plane | WORKS | WORKS | WORKS | WORKS |
| program | WORKS | WORKS | WORKS | WORKS |

Each row holds for all three drivers: two frames; a frame, `replace_value`, a frame; a frame, `replace_resource`, a frame.

`today` fails on the first frame, before any swap. The race is a plain write the calling thread makes after the first worker started (`src/on_sched.rs:99`, a context written for the next core) against that worker's retag of the whole scheduler (`:39`); in the engine Miri names the same race at `run_parallel.rs:212`, the `spawned` write. With the race detector off, Stacked Borrows still refuses it: the reference formed at `:39` is invalidated by the function-entry retag of the frame start's `&mut` to the bindings (`:113`) and used again at `:47`. Tree Borrows accepts that second part. So the hole is wider than the swaps: any plain write or `&mut` the calling thread forms over a byte the held reference covers breaks it, and the engine forms one every frame.

`narrow` works under both models, against both seats' argument that the next exclusive borrow of the scheduler invalidates the raw pointer whether it is held as a reference or raw. In this miniature the scheduler is `!Unpin`, as the engine's is (`scheduler/mod.rs:313`), so its `&mut` and `Pin<&mut Self>` are not retagged unique, and the raw pointer the worker holds shares the provenance the calling thread writes through. The argument would hold for an `Unpin` scheduler; it does not hold for this one. Its soundness rests on the `PhantomPinned` field and on the worker forming no reference across the park, neither of which a type checks.

`plane` works once the plane is written whole before the first worker starts; written context by context across the spawns, it raced the first worker's retag of the plane, the same race `today` has. `program` works and needs no such ordering, since each program is complete before its worker exists and nothing writes it after.

Holds for: the miniature in `src/`, two workers, four records, x86_64 linux, the nightly the workspace pins, Miri with `-Zmiri-ignore-leaks`, each model as named. It does not hold for the engine itself: the engine's drivers are `crates/hilavitkutin/tests/parked_worker_aliasing.rs`, and their run is in `mock/research/202611140606_parked-worker-aliasing/`.

Run with, from this directory, for each test name and model:

```
MIRIFLAGS="-Zmiri-ignore-leaks [-Zmiri-tree-borrows] [-Zmiri-disable-data-race-detector]" \
  cargo miri test --config 'build.rustc-workspace-wrapper=""' --test drivers -- --exact <arm>::<driver>
```
