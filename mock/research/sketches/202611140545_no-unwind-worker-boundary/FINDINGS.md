# A no-unwind boundary on a generic worker entry

The engine plan's job 3 asks whether the generic `worker_main` can be made a
boundary that a panic cannot unwind through, in a build that unwinds, and by
which of the two mechanisms its seats named. This sketch builds both against a
generic entry that takes one raw pointer, as `worker_main` does, compiled with
`panic = "unwind"`, and runs each in a child process whose worker thread
panics.

The outcome for each arm, in `test-output.txt`:

| Arm | Mechanism | Outcome |
|-----|-----------|---------|
| A | the entry is `extern "C" fn entry<T>(ctx: *const Ctx<T>)` | WORKS: the child dies of SIGABRT, no exit code, well under the 10 s bound |
| B | a Rust entry holding a guard whose `Drop` panics, forgotten on the normal path | WORKS: the child dies of SIGABRT the same way |
| control | a plain Rust entry | the thread dies, `join` reports it, the child exits 0 |

The control is the case the engine cannot survive: an executor thread that dies
without the engine's worker reaching its done arrival leaves
`frame_await_done` waiting for good.

Arm A is the one taken. A generic `extern "C"` function is accepted by the
compiler with the scheduler's type parameters on it, it needs no `std`, and
the abort is the language's guarantee at a non-unwinding ABI edge rather than a
second panic the guard has to raise. It also marks the entry `nounwind` to the
optimiser, which is what the ratified abort row
(`execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen`)
asks of failure. Arm B works too and stays the shape for a body that has to run
inside a Rust-ABI function.

Holds for: the generic entry over one type parameter and one raw pointer,
`panic = "unwind"`, the dev profile, x86_64 linux, a panic raised in the body on
a spawned thread. It says nothing about a panic on the thread that calls the
run, which unwinds into the caller in both arms because the boundary is on the
worker entry only.
