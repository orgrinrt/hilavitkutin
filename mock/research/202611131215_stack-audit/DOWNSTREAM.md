# Fit To What Is Built On Top

Sibling of `AUDIT.md`. The audit was asked to judge hilavitkutin as part of the stack, with the downstream being kaski, a general game engine in the sense that Unreal, Unity and Godot are, plus generic applications and server daemons built on hilavitkutin, arvo and notko.

That is a much wider brief than the one the engine was designed against. The founding consumers were polka with about 200 records and a deep chain, saalis at 10,000 to 100,000, and loimu at 100,000 to 10 million (`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1964-1970`). Op has since said the target is cases with millions of records crunching constantly, and that 1 to 4 morsels is almost never a realistic use case (channel `op-answers-20`). The gap between those two statements is the subject of this file.

## What a game engine needs that is not there

Mark: going wrong.

Several record sets. A game has units, tiles, particles, projectiles and lights, and they do not have the same count. The engine has one record count for every column in a schedule (`crates/hilavitkutin/src/scheduler/builder.rs:170`; `mock/crates/hilavitkutin/DESIGN.md.tmpl:1188-1196`). Everything downstream of that is distorted: an accumulator's capacity is the record count, the morsel formula divides one count, the N-way split slices one range. Op has answered the shape: contiguous per prototype, with indices to process the needed windows (op-words digest 2.29). The api revision adds a record domain as a column parameter with a count per domain, which is that answer. It is not in the engine.

Growth. Spawning is the ordinary case in a game and the engine cannot do it. Columns are sized at plan time and never resized (`mock/research/202607200200_replaceable-swap-semantics-spec.md:62-63`), the storage holds one base pointer and re-reserving is undefined (`crates/hilavitkutin-api/src/storage.rs:46-47,:59`), and the memory provider has no grow operation (`crates/hilavitkutin-api/src/platform.rs:28-53`). The workaround a consumer would reach for, reserving the maximum and keeping a live count, works and is what a fixed-capacity simulation does; it does not work for a world that streams.

Reading a neighbour. A cellular simulation, which is what kaski's lattice is, reads the cells around the one it writes. The engine's access model has a read set and a write set and no notion of reading a different record of the same column, and the unsafe index accessor (`crates/hilavitkutin-api/src/context.rs:117-137`) lets a unit do it with no declaration and no bound, which is also what makes the N-way slice unsound for such a unit. The api revision's `Reach<C, const N: Cap>` with `Window` here, before and after is the right answer and has no example and no sketch yet.

Reading last frame's value. Every simulation that updates in place needs the previous value of something, and the usual answer is a buffered column. The engine has no such store kind. The api revision adds a frame-buffered kind, reviewers disagreed about its form, and no sketch exists. Op asked for a schedule that exists specifically for two writers of such a column; the round refuses two writers instead.

Emission at scale. Spawning projectiles, particles, events. The accumulator is the mechanism and its admissible input depends on the worker count (`mock/crates/hilavitkutin/DESIGN.md.tmpl:414-420`), which a simulation cannot accept.

Determinism. kaski runs at 30 ticks a second with speed multipliers, and a colony simulation has to replay. Determinism across worker counts is not held today: accumulator results depend on the core count, and the ungated N-way record split (`crates/hilavitkutin/src/scheduler/worker.rs:323-348`) gives a unit that reads across slices a different answer per run. Determinism across targets is untouched: there is no wasm arm anywhere in the bench harness and no cross-target digest.

Fixed point. kaski's own lane is `Fixed<FRAC>(i64)` with an `i128` multiply, scalar (`kaski` `fixed.rs:54,:173-176`), which is the correct choice for a deterministic simulation and the worst case for the vectorisation the engine's advantage rests on. A 128-bit multiply per record does not vectorise on the targets in question. This is not an engine defect; it is a fact about the consumer that the engine's performance story has not accounted for, and the catalogue asks for a bench of a fixed-point lane for that reason.

Content as data. The workspace rule is that a kind of content is a trait with rows a pack declares, never a closed set. The engine's composition is static by design and that is right for the engine, but a game's content arrives at load time. The plugin-host layer exists for exactly this seam and has no engine-side story: an extension's work units cannot join a schedule, because the schedule is a compile-time fact. Whether that is a limit to accept or a problem to solve is the largest open design question for kaski specifically, and nothing in the record addresses it.

Persistence. A colony simulation saves. The cold store opens without reading a manifest, flushes without writing and reports success (`crates/hilavitkutin-persistence/src/cold_store.rs:32,:60`).

## What a daemon needs that is not there

Mark: going wrong.

A server is a different animal from a frame loop and three things block it outright.

Failure. Abort is the policy (`consolidation:2448-2451`), a worker panic can hang the frame wait (`mock/crates/hilavitkutin/DESIGN.md.tmpl:607-610`), and `PipelineResult` is designed and not shipped (`mock/research/seed/scheduler.md:117-120`). A daemon that aborts the process because one record was malformed is not a daemon, and one that hangs is worse. This is the item `AUDIT.md` marks back to the drawing board: the engine needs a failure policy with a per-unit and per-record arm, not a repair of abort.

Ingestion. Work arrives over a socket, in batches nobody sized at build time. That is the growth problem again with a harder deadline, plus a boundary the engine does not have: there is no backpressure anywhere in the design (`mock/research/seed/plan.md:219-221`), so a producer faster than the pipeline has nowhere to push back to.

Observability. A long-running process needs to say what it is doing. The adaptation subsystem is the closest thing and it does not actuate; there is no counter surface, no per-phase timing a caller can read, and the two clock reads per phase per frame that the engine already does (`crates/hilavitkutin/src/scheduler/run.rs:205,:296`) feed an EMA whose update is a no-op body.

Against that, two things fit a daemon well. The pre-allocated, no-allocator discipline is exactly what a long-running service wants, because there is no fragmentation and no allocator latency in the steady state. And a schedule compiled once and reused across millions of frames is the right shape for a request loop with a fixed pipeline.

## What fits well right now

Mark: good as it is, for this shape.

A batch job over fixed columns with a known record count, a deep chain of per-record operations, and a result at the end. That is what the engine was designed for, it is what the founding consumers were, and for that shape the design is sound and most of it is built. A consumer with that shape today gets a compile-time schedule, no allocation, no indirect dispatch, and a parallel decomposition derived from declarations rather than written by hand. That is a real thing and it is worth saying in an audit that is otherwise mostly about what is missing.

The region op named, millions of records crunching constantly, is this shape at a larger size. The engine is closer to being right for it than it is for a game, and the missing pieces for it are performance pieces (fusion, windows, the parallel path's incremental skip) rather than model pieces.

## What that means for the order of work

The model questions and the performance questions are not equally urgent, and the record has them the wrong way round. Every performance item in the roadmap is downstream of a model decision that is still open: the morsel formula divides a record count that is about to become per-domain; the parallel split slices a range that a reach window will constrain; the accumulator's capacity is the record count. Doing the performance work first means doing it twice.

So: settle the record domains, the buffered kind and the reach window in the api, with op's answers already on record for all three; fix the aliasing hole and the failure policy, because they are correctness; then the fusion and window work, with the bench the catalogue asks for. The founding consumers' scale is no longer the thing to design against, and the engine should stop being measured at 4,096 records.
