# Estimates: Where A Design Built To Intent Would Land

Sibling of `AUDIT.md`. The audit was asked to estimate how the designs, built to their intent, would perform against hand-written optimised assembly as a yardstick, against ordinary std Rust, against no_std Rust, and against comparable frameworks by name.

Everything in this file is an estimate. Not one line of it is a measurement of hilavitkutin, and where a number appears it is either reasoning, labelled as reasoning, or a figure from the ad-hoc spike written for this audit (`mock/research/sketches/202611131400_chain-shapes-adhoc/`), which measured hand-written shapes of one kernel on one cloud guest and linked no hilavitkutin crate. The comparisons with other frameworks are reasoning from their published design, since none of them was run here.

The estimates assume the designs are built as written: fusion across a fiber rather than opt-in on a linear chain, per-fiber morsel windows actuating, co-located arenas for intermediates, the incremental skip on every path, and the const-gated dispatch holding at the unit counts a real program has. Where an assumption is doing the work, the section says so.

## The yardstick: hand-written optimised assembly

Take as the yardstick a person writing the inner loop by hand for one target, with full knowledge of the data layout, intrinsics or assembly, and no abstraction to preserve.

For a fused chain of per-record arithmetic over columns, the estimate is that a design built to intent lands between parity and about 1.3x of the yardstick, and that the gap is almost entirely vectorisation width rather than anything structural. The reasoning: once the intermediates are in registers the loop is a straight-line body over contiguous arrays with no aliasing, which is the shape LLVM vectorises well, and the remaining difference is whether the compiler picks the same width and unroll factor the person would. The spike gives one data point in that direction, with the caveat that it is one kernel: hand-written AVX2 ran 0.70x to 0.80x of the compiler's fused loop on the baseline target, and 0.80x to 1.03x once the compiler was given the same target features. So on this kernel the hand-written advantage was the target features and not the hand-writing.

Three things would push a real engine further from the yardstick than that.

Raw pointers in the loop body. The engine hands columns to a unit as raw pointers rather than slices (`mock/research/seed/contracts.md:59-64`). LLVM will version a loop with runtime overlap checks for a handful of pointers, but the number of checks grows with the number of pointer pairs and there is a threshold past which the vectoriser gives up and the loop stays scalar. The spike's raw-pointer arm matched the slice arm at four pointers; that result holds for four pointers and says nothing about a unit that reads a dozen columns. This is the single most likely place for the engine to lose a factor of two to four without anyone noticing, and `BENCH-CATALOGUE.md` asks for a bench of exactly it.

The per-morsel boundary. Every morsel is a loop entry, a prologue and an epilogue, and a vectorised body with a scalar tail. At a window of 4,096 records that is noise; at the 64-record windows the formula can produce for a wide write set (the clamp is 64 to 8,192, `design_rounds/202604200055/202603171900_topic.execution-model.md:97-109`) the tail and the prologue become a measurable fraction. The design's own answer, sizing from the write set rather than the read set, is the right lever.

Register pressure at the fused width. The founding topic already found this and it is the most useful measured fact in the record: a 15-column fused trunk of six units showed eight wasted instructions per element from rematerialisation and spills, and the fix was to cap the fused column count at the register file rather than at L1 (`:22-44`). A fusion design that ignores that cap is slower than no fusion.

## Ordinary std Rust

Against an ordinary competent std implementation, written as a person would write it without thinking about the schedule, the estimate splits by what the person wrote.

If the person wrote one fused loop per logical operation, as the `engine_vs_std` baseline does, a design built to intent lands at parity to about 1.2x, and the gap is per-frame overhead rather than per-record cost. The design's own target is 0.95x to 1.02x (`mock/benches/engine_vs_std/FINDINGS.md:158-161`), which is the right order. The measured frame floor of 7.1 to 11.1 microseconds for one unit in one fiber (`mock/benches/single-trunk-convergence/FINDINGS.md:60-67`) is the thing that decides whether parity holds at small frames: at a million records it is 0.01 ns per record and invisible, at a thousand records it is 10 ns per record and dominant.

If the person wrote a pass per operation, which is the ordinary shape when operations are written as separate functions over vectors, the engine should win by the fusion factor. The spike puts that factor at 2.2x to 3.0x on one kernel at sizes out of cache, which is the bandwidth ratio of moving 16 bytes per record instead of 40. That is the engine's real argument and it is worth stating plainly: the advantage over ordinary code is not that the dispatch is faster, it is that the intermediate columns never exist.

Where the engine should win by more than that, and where no estimate here is reliable, is the incremental skip. A simulation where most records are unchanged most frames pays nothing for them, and an ordinary implementation pays for all of them. The ratio is then whatever the change rate is, and no number in the record measures it because the skip does not run on the parallel path at all.

Where the engine will lose to ordinary std is anything it has no shape for: an operation over a subset selected at runtime, a join, a sort, anything with a data-dependent output size beyond the accumulator's append.

## no_std Rust

Against the same code written no_std with no allocator, the estimate is parity in the steady state, because a no_std implementation of the same loop compiles to the same instructions. The differences are at the edges and they favour the engine.

A no_std implementation has to size its buffers up front, which is what the engine's plan does for it. It has no allocator to grow an output, which is what the accumulator is for. It needs the memory for the intermediates to come from somewhere, which is the co-located arena.

So the honest statement is that the engine's cost against hand-written no_std code is the plan build (a fixed 64 to 67 microseconds in the one measurement, `mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md:89`) plus the per-frame floor, both amortised over frames, against the benefit of not writing the schedule by hand. For a program that runs one frame, the engine is slower. For a program that runs a million frames, the amortised cost is zero and the comparison is the steady state, which is parity.

## Compared with other frameworks

These are reasoning from published design and from what each system is for. None was run for this audit and no number below is measured.

Bevy ECS. The closest first-party comparison in Rust, and the one kaski's audience would reach for. Bevy's scheduler resolves system access sets at runtime into a dependency graph and runs systems on a thread pool; iteration goes through archetype tables with per-component slices, and queries are typed but dispatch is a dynamic walk. The estimate is that a hilavitkutin built to intent is substantially ahead on a chain of per-record operations over the same columns, because the intermediates fuse and Bevy's systems each make a pass, and that Bevy is ahead on everything structural: spawn and despawn, variable entity counts, sparse components, change detection that the framework maintains rather than the user. The honest summary is that they are not solving the same problem, and that hilavitkutin would be a poor general ECS and a good crunching core underneath one.

flecs and EnTT. Both are mature C and C++ entity systems with years of work on archetype storage and iteration. EnTT's grouped views reach near-optimal iteration over dense pools, and flecs has a real query engine with change detection and observers. The estimate is the same as for Bevy on both sides: parity to an advantage for hilavitkutin on a long fused chain over fixed columns, a clear disadvantage on anything involving a changing set of records. Neither of them fuses operations across systems, which is the one thing hilavitkutin's design does that they do not.

Unity DOTS with Burst. The nearest thing in industry to what hilavitkutin is aiming at: jobs over chunked component arrays, compiled by a backend that vectorises and knows the aliasing facts because the job declares its access. The estimate is parity on the per-job loop, with Burst ahead on the vectorisation of a single job today (it has had years of work on exactly that) and hilavitkutin ahead in principle on fusing adjacent jobs, which DOTS does not do. This is the comparison worth taking most seriously, because it is the one that shows that the declared-access-set idea is not speculative: it shipped and it works.

Vectorised analytical engines (DuckDB, and the MonetDB/X100 line it descends from). This is the closest relative of hilavitkutin's actual execution model, and the most useful one for calibration. Those engines process a vector of records per operator, materialise intermediate vectors, and interpret the plan at runtime; the published position of that line of work is that a vectorised interpreter lands within a small factor of compiled code, with the gap being the operator-call overhead per vector and the materialisation of intermediates. Hilavitkutin's design removes both: the plan is monomorphised rather than interpreted, and fusion removes the intermediates. So the estimate is that a hilavitkutin built to intent should be ahead of a vectorised interpreter on the same query shape, and the interesting part is that the gap it is trying to close is exactly the gap that literature measured. The project should read it: the per-vector overhead numbers in that work are the numbers the per-morsel boundary question needs.

rayon. Not a comparison of execution shape, only of the parallel decomposition. The estimate is that rayon's work-stealing split beats the engine's static assignment on irregular work and loses on regular work, where the engine pays no stealing overhead and gets perfect cache affinity. The engine's deterministic assignment is worth more than the speed difference for a simulation, because a work-stealing split cannot give the same result twice.

Arrow compute kernels and the columnar tool ecosystem. A useful calibration point for one number: those kernels are the hand-tuned per-operation form, one pass per operation, and they are close to the memory-bandwidth limit for each pass. A fused chain should beat a sequence of them by roughly the ratio of bytes moved, which is the 2.2x to 3.0x the spike shows for a four-stage chain, and should not beat any single one of them.

## What these estimates are worth

Every one of them assumes the designs get built. The most load-bearing assumption is that fusion becomes a property of a unit's declaration rather than an opt-in entry point on a linear chain, because without that the engine runs the materialising shape and the estimates above describe something that does not exist. The second is that the raw-pointer aliasing surface does not defeat the vectoriser at realistic column counts, which nobody has checked.

The third is that the plan is right. Every estimate here is about the cost of executing a given plan. None of them says anything about whether the plan chain picks a good plan, because there is no oracle and therefore no way to know. A perfect execution of a bad grouping loses more than any of the factors discussed above.
