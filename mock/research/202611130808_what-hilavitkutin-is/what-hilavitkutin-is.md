Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/what-hilavitkutin-is.md, first posted 2026-11-13 08:08 UTC, read 2026-11-15.

# What hilavitkutin is and is meant to be, from its own record

Read 13 November 2026 at `origin/dev` `c42d707`, the revision at `origin/work/api-revision` `83f6f25`. Nothing built or run. One reader, no prior work seen.

Path keys, all under `/home/user/hilavitkutin/mock/`: P = `PRINCIPLES.md.tmpl`, D = `DESIGN.md.tmpl`, A = `crates/hilavitkutin-api/DESIGN.md.tmpl`, E = `crates/hilavitkutin/DESIGN.md.tmpl`, S/x = `research/seed/x.md`, C = `design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, T3 = `design_rounds/202604200055/202603121800_topic.dispatch-and-optimisation.md`, CL = `design_rounds/202611130617-h6mb_changelist.doc.md` on the revision branch. "Text" means his record says it, "code" means source does it, "judgement" is mine.

## 0. Standing of each source, by its own words

- `mockspace.toml` has no `canon_paths` (grep returns nothing). No canon is declared.
- C calls itself "Single source of truth for hilavitkutin's current design state" (C:5) and its header still reads "Status: IN PROGRESS, being built interactively" (C:4). E defers to it: "The consolidation topic (202603181200) is the authoritative design source. This document maps domains to engine modules" (E:9-10).
- The seed calls itself "the flat, self-contained expression of hilavitkutin's effective engine canon as of 2026-07-19" (S/governance:3-4) and is not frozen: "frozen = false" and "The freeze awaits the op rulings" (S/MANIFEST:4, 10-11). Its precedence order puts C first, op-ruled amendments second, and "roadmaps, audits, expert memos, sketches, generated design docs, agent memory" as "intermediate material, never canon" (S/governance:16-32). It names the design templates "tier-4 under A2-1" (S/MANIFEST:48-49).
- So the brief's ladder (templates stand in for canon) and the seed's ladder (C first, templates fourth) disagree. Where they collide I say so. The collision that matters is write-write ordering (section 5).
- Roadmaps: latest is r8 (`research/pre_seed/202607193000_engine-roadmap-r8.md`), by the seed's words never canon; I use it only as a ledger of built against unbuilt.
- The workspace skill is a claim. Checked in section 4.

## 1. What it is, in sentences a newcomer can act on

1. It is an execution engine for pipelines, and the author writes units, not a loop: "Morsel-driven pipeline execution engine" (D:3).
2. A unit states its data access in types and gives a body: "declare what you read, what you write, how you want to be scheduled, and a body. Everything else (DAG, ordering, phases, trunks, fibers, morsel sizes, strategy, codegen, sync) is derived" (S/contracts:3-6).
3. The api crate is the contract and the engine is replaceable under it: "the api is the contract, the engine is one implementation of it" (S/identity:42-43).
4. All state lives in stores the scheduler owns: "Data in a hilavitkutin app lives in columns and resources the scheduler owns" (S/data-model:3).
5. There are three store types under one scheduling mechanism: "Three store types, one scheduling mechanism" (S/data-model:62; A:140; C:491).
6. The order of work is derived from the access declarations, never written by an author: "Ordering is the data flow. DAG edges come from write-to-read column overlap; there are no triggers and no emits" (S/data-model:112-114).
7. An author gives hints, not an order: "WUs declare scheduling hints; the scheduler derives ordering" (S/identity:131-132).
8. Composition is fixed at compile time: "All WUs registered at compile time. No runtime registration. Plan parameters adapt; composition does not" (P:256-257).
9. The types carry the dispatch: "No `dyn`. No `TypeId`. No `std::any`. WUs are generic over their AccessSet; the monomorphised function IS the type identity" (P:210-211).
10. Reaching a store a unit did not declare does not compile: "Accessing a store not declared in `Read` or `Write` fails to compile" (A:524-525).
11. There is no heap anywhere: "Every crate is `#![no_std]` with no allocation" (P:198), and "The heap is not temporarily available; there is no 'just for this moment'" (P:14-15).
12. The plan is made once and reused: "Plan recomputes only on pipeline structure change (new WUs, record count change, DAG mod)" (P:243-244).
13. The scheduler runs its own lifecycle as units on itself: "The scheduler IS a pipeline. Meta WUs ... run on the same engine that runs user code" (P:249-250).
14. It is one engine at any core count: "There is no separate single-core or multi-threaded engine, run, or dispatch path anywhere" (S/identity:11-12).
15. It is not an ECS and has no tables: "'Entity' implies ECS; hilavitkutin is not an ECS" (C:99), "Columns are independent. No tabular joins" (P:271).

## 2. The pillars

Settled in types. The unit set, the dependency graph, the order and the grouping are type-level facts: "Locked at compile time (the types). The WU set, the dependency DAG, the topological and RCM row order, the fiber/trunk/phase topology, and the monomorphised dispatch functions. Never runtime-mutable" (S/identity:81-84). Store markers are zero-sized and erased before the run: "Store marker types are ZSTs that constrain the DAG, enforce type safety, and determine morsel sizes at compile time; they are erased to plain indices before execution begins" (S/data-model:84-86). Capacities are types: "Every container's capacity lives at type level" (P:104). Only parameters move at run time: "Morsel sizes, per-phase configs, core and lane affinity, record ranges, dirty masks" (S/identity:86-87).

What a contract is, and who implements what. A position is a trait, never a concrete type: "The question isn't 'what type goes here'; it's 'what is the contract this position represents'" (P:47-48), and "contracts before implementations (traits first, always)" (S/constraints:26-27). The consumer implements `WorkUnit` (S/contracts:10-18), the platform traits ("the consumer provides every platform implementation", S/foundations:58-59), the backing memory ("The consumer provides the backing memory; the library never calls an allocator", S/data-model:94-95), kits, and optionally an executor. The engine implements the context a unit runs against: "only the engine names `EngineCtx<'frame, W::Read, W::Write>`" (E:159-160). The api already splits declaration from running: "`Plannable` is that half, so a consumer whose runner is not the fiber engine can declare and plan work" (A:375-377).

What the engine does with the contracts. It reads only the declarations: "hilavitkutin does not care what is in the columns, only the read/write dependency structure" (S/foundations:31-32). From them it builds "the DAG from Read/Write overlap, detect phases via waist analysis, identify trunks per phase, assign branches and bridges, form fibers within each trunk, size morsels per fiber, select an execution strategy per phase" (S/plan:15-18), then emits code with "zero indirect calls" (S/dispatch:3-4).

Where state lives and where code lives. State: columns and resources the scheduler owns, on one store: "All engine runtime state lives in one store" and "There is no bespoke per-subsystem storage. No hand-rolled arena" (P:121, 128). Resource against column "is intent (persistent / cold versus hot / per-morsel), not a separate mechanism" (P:124-126). Code: unit bodies. "The op author controls the inner loop, the framework controls morsel boundaries" (S/contracts:66-67). What the engine does not own: "column storage backing memory, the data model or schema, query planning, UI, or IO" (S/identity:26-27), and "Column<T> ... carries no schema parameter; schema-as-traits wrappers are consumer-side" (S/data-model:72-74, resolution R1 at C:2397-2404).

What the scheduler knows and derives.
- Read after write: "`write_masks[i] & read_masks[j] != 0` means i precedes j" (S/plan:32-33). "Write implies Read" (S/contracts:38).
- Write and write on one store: "Write-write ambiguity on the same store resolves by scheduling-hint priority (Urgency, then Divisibility, then Significance), tie-broken by most downstream dependents first, then a deterministic fallback" (S/data-model:114-117; C:591-593; A:366-368). The hints: three sealed axes, "Higher discriminant = higher priority" (A:940-942).
- Phases and waists: "A phase is a wide DAG section between waists" (S/plan:20). Trunks: "a column-disjoint maximal path within a phase: trunks share no write columns, so sibling trunks need zero synchronisation" (S/plan:21-23). Fibers: "the unit of column co-location and morsel windowing" (S/plan:126). Fusion is codegen's choice inside a fiber (S/plan:126-128), and the common case is "the WU bodies as a pure function pipeline through locals" (S/dispatch:83-88).
- Morsels: "`morsel_size = (L1_usable / sum(write_sizes)).clamp(MIN_MORSEL, MAX_MORSEL) & !3`" (S/plan:188). Scalar or chunked is inferred: "any Column in Read/Write means morsel-chunked, Resource/Virtual only means scalar" (S/plan:195-197).
- Conditions: "`Always` (unconditional, the default) and `On<V>` (runs when virtual V fires); an `After<U>` condition was dropped" (S/contracts:32-34). Virtuals "are pure flags, never data carriers" (S/contracts:95).
- Skipping: "A WU with an entirely clean predecessor set skips execution" (S/plan:117), "a per-morsel generation counter bumps on write, unchanged morsels skip" (S/plan:208-210), and stamps "increment only on completion" (S/scheduler:69).
- Commutativity is declared per unit and gates the one record split: "2-way head+tail convergence inside a single-trunk commutative phase ... Never an N-way record or morsel partition" (S/execution:17-22).

The constraints and what each is for.
- No alloc, no `dyn`, the 16-byte value cap, columnar layout, `Copy`, 64-byte alignment: each removes a vectorisation killer, "The design is the SIMD contract" (S/constraints:81-89). No alloc also makes sizing a plan-time fact: "allocation size is const generic, known at plan time, which is what lets morsel sizing and arena reservation be computed once" (S/storage:16-17).
- No `dyn`, static composition: "This is what makes LLVM devirtualisation possible: the schedule is statically analysable" (S/identity:83-84).
- `&self` on every context method and raw pointers, never slices: "`&mut self` anywhere in the Context API breaks every fused fiber through noalias write reordering" (S/constraints:20-22).
- No runtime spawn, no linker-magic registration, no `std::any` (S/constraints:10-15). No bare primitives (P:19-24). Extension across an ABI is "registered traits + `repr(C)` descriptors (inventory pattern), not `dyn`" (P:78-79).
- "One mechanism, not two ... If two subsystems solve the same problem differently, one must go" (P:113-117).

The optimisation stance. Performance is decided by measurement and gated: "Algorithm and performance forks are benched, never argued" (S/constraints:107-108), "The gate is blocking and standing" against "optimally threaded standard-library code at equal core count" (S/execution:195-196), every dispatch function passes an assembly gate with "zero indirect branches in the body" (S/dispatch:156-158). Shortcuts are refused in the design, not the code: "never a pragmatic shortcut, change the design not the implementation" (S/constraints:33-34). Adaptation "must never degrade" (E:921-922).

## 3. What it is meant to become and is not yet

Each line: the statement, its source, its standing.
- RCM row order applied as the execution order, ending the producer-before-consumer registration rule: "It must never be presented to consumers as permanent" (S/contracts:90-91). Seed, op decision r2 (b). Not built (r8:170-172).
- Per-morsel generation counters and version stamps. Seed and C (S/plan:208; C:2139-2145). r8 lists both ABSENT, with only "coarse per-store dirty" in code (r8:174-175).
- Dirty skip on the parallel path, head+tail convergence, phase overlap by progress counters, heterogeneous cores: canon in seed, built but unreached or absent in code (r8:135-147, 175).
- Dulmage-Mendelsohn dead-unit and dead-column elimination, matrix-chain grouping, spectral forming trunks (S/plan:79-112). Seed. Absent or deviated (r8:167-169, 194-199).
- `PipelineResult` with poisoning, schedule introspection, evict and inject, the facade unit for loaded extensions (S/scheduler:80-126). "designed; the per-fiber result type is not yet shipped" (S/scheduler:117-118).
- A mutable resource surface: "when a mutable resource surface ships, mutable resources write back to canonical storage after the loop" (E:528-530). Template text, tier 4. Today "A consumer `Resource<T>` is `Copy` and read-only after init" (E:876-877).
- Write-after-read ordering: "WAR anti-dependencies are a tracked plan-chain follow-up" (E:44-45). Template text.
- The plan itself as columns on the store: "Unifying these flat columns onto the scheduler's columnar store is the next step" (D:48).
- Fully compile-time per-core programs by build-script codegen, if a bench demands it (S/dispatch:72-78). Op-blessed deviation.
- A registry of canon rows once the seed freezes (S/governance:64-68). Not created.

## 4. What it is not

From his text:
- Not an ECS, no entities, rows, archetypes or tables (C:92-101; P:286-287).
- No author-written order: "`order` as a WU field" is a dead term (S/identity:131), `After<U>` was dropped (S/contracts:33-34), "no triggers and no emits" (S/data-model:113).
- No access outside the declaration: fails to compile (A:524-525), and the context "holds only the projected stores it declares (a WU cannot reach an undeclared store, since its Context does not carry it)" (E:149-151). A snapshot view "cannot escape across a phase barrier" (A:1101-1102).
- No storage beside the store: "When a subsystem needs to hold N things, the answer is a column, not a new arena" (P:130-131).
- No second mechanism for one concern (P:113-117), no second engine path (S/identity:11-13), self-hosting "is not a second engine" (S/scheduler:41-42).
- No runtime registration, no dynamic schedules (S/scheduler:31-33), no runtime column type creation (C:2411-2412), no linker magic (S/contracts:77-78).
- No slices from storage, no `&mut self` on the context (S/contracts:51-63). No data in virtuals. No result extraction from the run (S/scheduler:83-85). No work stealing by default, no backpressure, no lineariser (S/data-model:125).
- No schema or data model in the engine (S/identity:26-27; R1, C:2397-2404).

The workspace skill against his text (judgement):
- Its decision test (no way to refer to scheduler data from outside a declared access set) agrees with A:524-525 and E:149-151.
- It calls the model "ECS-shaped" (skill:8). His text says it is not an ECS (C:99). The skill's word is wrong.
- Its example has two units both with `type Write = (Column<Diagnostic>,)` (skill:75-76) and does not say how they are ordered. This is the exact blind spot of today's question.
- It says a morsel iterator "hands out `&T` into the column buffer" (skill:107). His text: "there are no slice-returning methods" and `read` returns `T` by value (S/contracts:59-62).
- It says "exactly three things and there is nothing else". True of S/data-model:62, but E describes a fourth, the accumulator (E:1194-1199), see section 5.
- "No runtime checks" is not a statement of the record. The record has runtime refusals (`BuildError` on registration order, S/scheduler:33-36; accumulator refusal, A:494-507).

## 5. Contradictions and silences

Write-write order, the record against itself.
- Text, tier 1 and seed and api template: hint priority, then most downstream dependents, then a deterministic fallback (C:591-593; S/data-model:114-117; A:366-368; S/plan:34-35).
- Text, engine template: "WAW conflicts serialise deterministically by input index (the lower index first)" (E:43-44).
- Code: `if s < t && writes[t].overlaps(&writes[s])` adds the edge, lower registration index first (`crates/hilavitkutin/src/plan/steps/step1_dag.rs:45-51`). Hints are not read there.
- Judgement: the code implements only the last fallback of his rule. I found no ledger line recording this as a deviation. Either way the record never says two writers "have no order". His answer of today agrees with the tier-1 text.

Who declares what "changed" means. The passages:
- Founding: "a monotonic counter that increments whenever any row in the morsel's range is modified. The ColumnStorage (or a separate change-tracking layer) bumps the generation when a write occurs" (T3:909-912). Changed means written, and the engine or the storage declares it.
- C: version stamps "Capture before-versions before WU runs, compare after. Changed stores = bitmask of actually-modified columns" (C:2141-2143).
- Research he likely recalls, marked "Research (reference material for hilavitkutin design)": a table of "Phantom-write detection strategies: Always increment / Bitwise comparison / Hash comparison / Developer hint `.mark_unchanged()`", then "loimu uses bitwise comparison for resources under 4KB ... For larger resources, the developer calls `.mark_changed()` explicitly; default is 'assume changed.'" (`design_rounds/202604200055/202603141800_research.loimu-dag.md:570-580`; the same in `...research.loimu-incremental.md:91-141`, with "Adopt this directly" at :749-753, written for polka and saalis).
- Agent round of June: "an explicit per-store change signal the consumer calls" (`design_rounds/202606111200/202606111200_topic.e7-dirty-skip-incremental.md`, section "Why the change signal is non-optional"). E:107-110 repeats it: "an input the consumer marked changed". Code: `Scheduler::mark_dirty::<T>()` (`crates/hilavitkutin/src/scheduler/dirty.rs:87`).
- So the record holds three answers: the write itself, a value comparison by the write accessor with a developer hint above a size, and the consumer's call. None is a parameter of the store type. None is in the seed beyond "bumps on write" (S/plan:208).

Record counts per column. Text: storage is per column, `reserve<T>(&mut self, id: StoreId, len: USize)` and `count(&self, id)` (A:268-279). Text: one count at build, "`build(storage, record_count)`" (E:1151). Text: "hilavitkutin does not load data, allocate storage, or know record counts ahead of time" (S/plan:214-215). Within a fiber one cursor serves all columns: "record index advances one cursor across all columns" (S/plan:129). The record does not say what happens when two columns in one scheduler differ in count. Silent.

Reads at an offset. Silent as a declared access. What bears on it: "Records within a fiber are independent; execution order of records does not affect correctness" (S/data-model:127-128); "No pipeline breaker: an op needing all input records before any output is a materialisation boundary" (S/plan:144-145); `BatchApi` is the "full morsel slice, non-fusible" form (A:462); the accessors "are morsel-relative and the only cross-record surface is the accumulator append path" (E:327-329).

A store read as the last frame and written as the next. Silent. Nearest: "There are no partial writes: WUs buffer internally and write on return, downstream does not run until completion" (S/data-model:132-134); `ResourceSnapshot<'phase, R>` "captured at a phase boundary" (A:1099-1100); read-only columns shared between trunks by "snapshot-to-local (copy the shared column per trunk at the phase transition" (S/plan:159-162); the morsel's third axis is "temporal (frames)" (S/plan:199); a cyclic dependency "cannot be linearised; the runner rejects it as `PlanError::Cycle`" (E:47-49).

Content adding stores without Rust. The record says no to new store types at run time: "Column type registration stays static" (S/scheduler:93). It allows new data and loaded code: "New data (records added to existing columns) flows through the same static WUs ... New code (a runtime-loaded extension) never enters the engine's monomorphised dispatch; it integrates through a statically registered facade WorkUnit" (S/identity:98-103), and "The facade declares bridge stores normally, so the plan sees honest access sets" (S/scheduler:110-111). Whether content needs a new store is not in the record.

Other collisions found on the way.
- Three stores (S/data-model:62; A:140) against an accumulator kind described in E:1194-1199 and S/constraints:66-68. The api template never names it.
- Resource layout: P:123-126 says every aggregate "struct-of-arrays-decomposes to scalar columns". S/storage:23-25 says the value is "one contiguous one-record blob ... never decomposed into per-member columns", bench-decided. The principles file is the stale one by the seed's order.
- `PlanAffecting`: sealed in A:1119-1126, "open marker traits consumers implement" in S/storage:83-84 (op ruling A2-3).
- `ColumnValue`: any `Copy + 'static` in A:225-228, "at most 16 bytes ... enforced at compile time" in S/data-model:44-45.
- Type taxonomy: P:92 "There is no fourth 'bare primitive' category", S/constraints:13 "trait, newtype, repr(C) enum, or primitive".
- Builder: `.add`/`.resource`/`.column` in S/scheduler:11-19, one verb `.with` in A:34.
- Default cap 16 in D:92, 256 in A:1131.

## 6. The revision against the page

For each form of the changelist: in the model's terms or outside, the passage, the model's own answer. All judgement unless quoted.

C7, two writers of a frame-buffered store refused. Outside the model. CL:153 argues "the rule would have nothing to resolve and the earlier unit would be dead work". The record orders any two writers of a store by hint priority (S/data-model:114-117), and the map row it rests on carried the premise "Two writers of one staged column have no order", which the record contradicts. What double buffering changes: readers see the committed side, so the read-after-write edge between this frame's writer and this frame's readers goes away, and a second writer cannot read the first writer's staged values. It does not change that two writers of the staged side have an order and that the later one wins where both write a record. So the schedule still decides, by the same rule. Dead work is also already the plan's to find, not a build refusal: "horizontal components are dead-end WUs whose outputs nothing reads (eliminable ...)" (S/plan:81-83). Two writers gated on different virtuals, or by hints, are a legitimate pipeline the refusal would forbid.

C6, the frame-buffered kind. Partly outside. In the model: it is a store marker, declared in access sets. Outside: (1) it is the first store whose access builds no edge, against "one scheduling mechanism ... a write-to-read overlap on a Resource builds the same DAG edge as one on a Column" (S/data-model:70-72); (2) the swap is a new engine walk, "`SwapBuffered` walk at the close of a pass in all three entries" (CL:183-184), beside the meta pipeline, against "Meta WUs ... run on the same engine that runs user code" (P:249-250) and "One mechanism, not two"; (3) "written in the value form only" ties a store kind to an author form, where the record ties access to the declaration alone; (4) the change rule as a third type parameter of the store is a new answer to a question the record answers three other ways (section 5). The model's own answer would start from what it has: the frame boundary is `meta::ScheduleEnd` and `meta::PassStart` (S/scheduler:44-46), completion is already buffered ("write on return", S/data-model:132), and a committed read-only view exists as `ResourceSnapshot`. A swap is then a unit on the meta band, and the writers are ordered like any writers.

C3, record domains (`Column<T, D>`, `Domain` with axes and reach, `Extent<D>`, `build(storage)`). The need is real and the record is silent on it. The form is outside on two counts. A second parameter on `Column` that says which set the column belongs to is what R1 removed: "Column<T> only, IR is consumer-side ... The `Column<In<IR>, As<T>>` schema-as-traits pattern is consumer-side" (C:2397-2399). Axes, keys and grids are a data model, and "It does not own ... the data model or schema" (S/identity:26-27); "Columns are independent" (P:271). `Extent<D>` as a new singleton kind stands beside a concept that already covers it: a resource whose value changes the plan is `PlanAffecting`, "Consumers opt in for resources whose value shape changes the plan" (D:92), and record count is already a plan parameter (S/identity:86-90). The model's own answer: the count is a fact of the column in storage (`count(id)`, A:279), a fiber's columns share one cursor so a fiber has one count by construction, and the plan carries a count per fiber, with a build refusal where a unit names columns of unequal count. Grouping names and geometry stay consumer-side.

C5, neighbour reads. Declaring the read in the access set is in the model's manner. The geometry (`Shift<Axis, N>`, `Edge`) is data model, as above. Restricting it to a store with a committed side is consistent with record independence (S/data-model:127). It also adds members to the read list that are not stores, where "`AccessSet` is a trait on tuples of store markers" (S/data-model:81). The model's own answer is not in the record; nearest is the non-fusible `batch` form with a reach in records.

C4, the value form as the primary form, two routers and `Declares`. Mixed. A pure per-record function is what the flattener already wants ("a pure function pipeline through locals", S/dispatch:86) and what `each` and the fusion opt-in already are (A:239-243, 458-460). A runner-agnostic declaration already exists as `Plannable` (A:370-377). Outside: `Declares` is a second declaration trait beside `Plannable`, against P:113-117; making the value form first demotes the contract his text states, "a body" under the author's control (S/contracts:3-6, 66-67); schedule and hint move to a router parameter where "`Always`, `On<V>`, and the meta conditions live on the trait impl" (S/scheduler:22-23) and `type Hint` is on the unit (S/contracts:14); dropping `COMMUTATIVE` loses a declared fact the plan reads (S/data-model:119-123), where a stateless map is simply commutative. CL:241 admits "one gather, call and scatter per record", which is slower than the body it replaces until fusion over lists exists; no bench is cited, against S/constraints:107-108.

C8, safe accessors with a branded index. In the model. It moves "ownership discipline that the engine proves at plan time" (A:552-553) into a type, which is the typestate pillar. One loss against his text: the body can no longer use the position as a number, and "the op author controls the inner loop" (S/contracts:66).

C1, five store kinds. Naming the accumulator corrects a gap between A and E. Counting `Buffered` (and in effect `Extent`, a sixth) departs from "Three store types, one scheduling mechanism".

C2, the shared list. In the model: one mechanism, fixed upstream (S/foundations:13-15).

C9 formats, C10 registration stays typed. Hold his text. The owed data-bound table sketch is against R6 if it ever enters the contract (S/scheduler:31-33); as a sketch it decides nothing.

The changelist as a whole states the pillars per form (CL "How each form holds the pillars") but tests each form against his words of 13 November only, not against the passages above. That is where the two departures of today came from.

## 7. Questions that are his

Q1. The founding spec and the seed say two writers of one store are ordered by hint priority, then by most dependents, then a fixed fallback. The engine's design template and the code order them by registration position alone. Which is the design?
(a) hint priority first, as founded; the code is a recorded deviation to fix. (b) registration position, and the founding rule is retired. (c) hint priority, with registration position named as the fixed fallback.

Q2. A unit needs the value a store held at the start of the frame while the same or another unit produces the next value. How does hilavitkutin say that?
(a) two ordinary stores and a unit on the frame-end meta band that carries next to current. (b) one store kind with a committed and a staged side, swapped by a meta unit, its writers ordered like any writers. (c) an ordinary column plus a read marker for "as of frame start", the engine deriving the buffering. (d) a kind with a committed and staged side, one writer only, swapped by the engine outside the meta band (what the revision wrote).

Q3. Columns in one scheduler may need different record counts. Where does the count live?
(a) on the column in storage; a fiber takes the count of its columns and a unit over unequal columns is refused. (b) on the column's type, as a group parameter. (c) consumer-side wrappers name groups, the engine sees per-column counts only.

Q4. Does hilavitkutin own geometry (axes, neighbour offsets, the value past an edge)?
(a) no; the engine knows only a declared cross-record read with a reach in records. (b) yes, in the api. (c) yes, in an opt-in crate over the api.

Q5. You answered "Declared per column" on what "changed" means. Where is it declared and for which stores?
(a) on every per-record store. (b) only on stores with a committed side. (c) by the unit that writes. And the default: differs by value, or written.

Q6. Is a per-record value function with no context the first form an author writes, or an opt-in beside `WorkUnit`?
(a) `WorkUnit` with its body stays the one contract; the value function is its fusible opt-in. (b) two equal forms under one declaration trait. (c) the value function first, `WorkUnit` as the block form.

Q7. May content add a store without Rust being compiled?
(a) no; content adds records, extensions add code behind a facade unit with statically declared stores. (b) yes, through a table bound at run time, which changes static composition.

Q8. Is the accumulator a fourth store type of the model, or an access mode of a column?

## 8. What I could not establish

- Whether any ledger or amendment records the registration-index write-write rule as a blessed deviation. I searched the seed, E and the code, not every round.
- Whether the research he recalls on "changed" is the loimu passage or another. A phrase search over `mock/` found no sentence that assigns the declaration to a column.
- C beyond its header, domains 06 to 12, 16, 23, the constraints and the resolutions; the ten other crate designs; the roadmaps before r8; the two topics of the revision beyond their openings; the sketches. Read in part or not at all.
- What "Schedule" names in his answer of today: the `Schedule` parameter of `WorkUnit` (`Always`, `On<V>`) or the derived plan. Both readings put the order with the scheduler.
- Whether the tier order of S/governance or the brief's (templates stand in for canon) is his current word; the seed is unfrozen and A2 is an "op-delegated trace".
- Any performance claim. Nothing was run.
