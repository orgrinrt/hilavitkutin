Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/map-loop-core.md, first posted 2026-11-13 05:29 UTC, read 2026-11-15.

# Map of `work/loop-core` onto hilavitkutin as it is

Read at `origin/work/loop-core` `5603f85` against `origin/dev` `26e926b`, 13 November 2026, by one seat that saw none of the night's planning. Nothing was built, run or checked out. Every tree was read only.

## What this file is

The lead designer asked for a hold and a map before the branch goes anywhere: every item the branch brings, laid against hilavitkutin's own design, so he can see whether it is his engine extended or a second vocabulary beside it. This file is that map, the map in the other direction, what the repository's own research already says on user-chosen column formats, and three shapes he can choose among.

The short answer is that the branch is a second vocabulary. Of the 52 items mapped, 20 are an existing hilavitkutin concept under a second name, 7 extend an existing concept, 3 correct one, 17 are new and belong in hilavitkutin (12 of them test instruments), and 5 do not belong in it. The new things are few and real: a record domain, a column whose reads see the last committed frame and whose writes are staged until a swap, a declared neighbour read, laws with mutants, an oracle runner, and frozen consumers under a ledger. Everything else the branch needed to carry those was rebuilt beside the engine instead of taken from it, and the branch's design text then calls the existing contract "legacy" and schedules its removal, which no word of his supports.

## How to read it

Three kinds of statement are kept apart. "Design" is a quotation from a design he wrote, with path and line. "Code" is what a file does, with path and line, and claims nothing about whether it is right. "Judgement" is mine. Paths under `mock/crates/` are written from the crate name. Branch paths are read with `git show 5603f85:<path>`.

## What governs, and the two gates

hilavitkutin declares no canon. Code: `mockspace.toml:9` sets `mock_dir = "mock"` and the file has no `canon_paths` key (a grep for it returns nothing). So the designs stand in: `mock/DESIGN.md.tmpl`, each crate's `DESIGN.md.tmpl`, and the research and rounds under `mock/`.

Gate one, refusals. Three things the branch and the night's roadmap say are refused here as stated, because they contradict a design of his or his words and name no supersession he made.

- Branch `mock/DESIGN.md.tmpl` (added lines): "`hilavitkutin-api` is the legacy contract until the engine is ported onto the contracts, and is then removed." His words of 12 November: "Ideally hilavitkutin is the perfect engine and paradigm for this engine". His words of 13 November: "I am wary and uncomfortable if that means you are adding something entirely out of place or too specialized or too parallel to what hilavitkutin is now". The panel's own stamped call 5 reads the other way: "b: in a small crate depending on notko alone, moved into `hilavitkutin-api` later" (`stamps-1.md:25`). The branch inverted the direction of the move.
- Branch `hilavitkutin-contracts/DESIGN.md.tmpl:29` and `:93`: "`Value` is sealed and `Int` is its one impl", "downstream content has no value type of its own". Design, `hilavitkutin-api/DESIGN.md.tmpl:228`: "Any `Copy + 'static` type is column-storable, reporting a `size_of * 8` width. Zero friction for consumers." His words of 13 November: "Upstream on hila it should be entirely generic and configurable by user."
- Branch `hilavitkutin-contracts/DESIGN.md.tmpl:15`, a kernel declares `ID: KernelId`. Design, `hilavitkutin-api/DESIGN.md.tmpl:359`: "No NAME const. WU identity is the type itself. Naming is a consumer concern". The roadmap page lists this as superseded "for the new crates only" (`pl1-engine-roadmap-r7.md:341`), which is a supersession made by an agent's note. It is his to make.

Gate two, suites. I lean on no test suite. I read the bodies of `hilavitkutin-laws/src/lib.rs` (all of it), `hilavitkutin-reference/src/lib.rs` (all of it) and the second half of `hilavitkutin-reference/tests/threads.rs`. I did not read `tests/bind.rs`, `tests/laws.rs`, `tests/mutants.rs`, `tests/common/`, the `ui` pairs, or any test of the existing engine, and no statement below rests on any of them passing. Where a sketch's FINDINGS file is cited, the claim is the file's, unverified by me beyond reading it.

One thing I verified by reading that bears on the laws: `model`, `morsel_split_and_order` and `frame_isolation` are comparisons of word slices (`hilavitkutin-laws/src/lib.rs:118-161`). A law holds when slices are equal. Whether the slices handed in mean what the law's name says is decided wholly by the caller in the test files I did not read.

## 1. The map

Columns: the item; what it is; the existing concept, design then code; verdict. Verdict codes: SAME (same thing under a second name), EXT (extension of an existing concept), COR (correction of an existing concept), NEW (new and belongs in hilavitkutin), OUT (does not belong in hilavitkutin).

### 1.1 What an author writes

| # | Branch item | What it is | Existing concept | Verdict |
|---|---|---|---|---|
| 1 | `ValueKernel`, `at` (`contracts/src/kernel.rs:39-52`) | A receiver-less function from the values of its declared reads at one record to the values of its writes | Design `hilavitkutin-api/DESIGN.md.tmpl:241`: "A work unit that is a pure per-record map (reads one column, writes one column, value to value, no cross-record state) may additionally declare that map as an opt-in fusion contract". Code `hilavitkutin-api/src/record_op.rs:34-42`, `RecordOp::apply(&self, x: In) -> Out` | EXT. `RecordOp` lacks more than one read and one write, a resource read, and a neighbour read, and it is an opt-in beside `execute` where the value form should be the primary way to write a unit. The value kernel is `RecordOp` widened to lists. |
| 2 | `KernelId` (`kernel.rs:19`) | A number an author declares so data can name the kernel | Design `hilavitkutin-api/DESIGN.md.tmpl:359`: "WU identity is the type itself". Code `work_unit.rs:5` | OUT as an author's duty. Where an identity must survive a rebuild it is a stored-data concern: `hilavitkutin-persistence/src/manifest.rs:27`, `:54` (`ColumnMeta`, `TableMeta`) or the consumer's content table. |
| 3 | `Kernels<D, P>` and the sealed `KernelList` (`kernel.rs:59-143`) | The type-level list of kernels a table is bound against, with an id fold | Design `hilavitkutin-api/DESIGN.md.tmpl:93` (`UnitDispatch<W>` prepends to the `Wus` accumulator). Code `work_unit.rs:230-256` (`WorkUnitBundle`) | SAME. |
| 4 | `Domain` with `Key`, `Step`, `Shape`, `first`, `next`, `reach` (`domain/mod.rs:18-32`) | A set of records with its own extent and key order | None. Code: one record count for the whole scheduler, `hilavitkutin/src/scheduler/builder.rs:170`. The research already places it: roadmap r6 puts domains behind phases B to D (`panel-consolidation-1.md` L65, citing r6 `:13-14`, `:315-320`) | NEW. It lives in `hilavitkutin-api` beside the store markers (`store.rs`), as a parameter of a column, and in the plan as a record count per domain. The `first`/`next`/`reach` walk itself is one of the branch's own unlocked shapes and I would not carry it as is. |
| 5 | `Offset` with `STEP` (`domain/mod.rs:35-40`) | A named displacement in a domain | None | NEW, with row 9. |
| 6 | `Column` trait (`domain/mod.rs:44-51`) | A value per record of a domain | Design `hilavitkutin-api/DESIGN.md.tmpl:146`: "`pub struct Column<T>(PhantomData<T>); // collection, morsel-chunked`". Code `store.rs:76` | SAME, and the same name. What it adds is row 4's domain and row 12's boundary value. |
| 7 | `Resource` trait (`domain/mod.rs:55-58`) | One value, constant over a step | Design `:145`. Code `store.rs:18` | SAME, and the same name. |
| 8 | `Own<C>` (`domain/member.rs:16`) | This record's value of a column, as a member of a list | A `Column<T>` in a `Read` or `Write` access set. Design `:159-165`. Code `access.rs:18`, `:39` | SAME. |
| 9 | `Near<C, O>` (`member.rs:19`) | A declared read of a column at an offset | None. Code: a unit may call `read(i)` at any index, unchecked, `context.rs:112` | EXT of the read access set, which lacks a declared reach. kaski's need is stated at `kaski/mock/registry/tenet/engine.toml:559` (`the_lattice_and_the_cascade_are_one_abstraction_and_the_lattice_adds_only_the_neighbour_read`). |
| 10 | `Param<R>` and `Coefficient` (`member.rs:22`, `bind/mod.rs:56`) | A resource value supplied per unit | `Resource<T>` in the read set, `context.rs:179` | EXT. The existing resource is one value per type; this needs one value per unit of the same kernel type. In hilavitkutin's terms that is a kernel generic over its resource marker, or a resource keyed by unit. |
| 11 | `Member`, `ReadMember`, `WriteMember`, `Reads`, `Writes` (`member.rs:25-68`) | The read list and the write list of a kernel | `AccessSet`, `Contains`, `WorkUnit::Read`, `WorkUnit::Write`. Code `access.rs:18-39`, `work_unit.rs:56-58` | SAME. |
| 12 | `Column::OUTSIDE` (`domain/mod.rs:50`) | What a neighbour read past the edge returns | None | NEW, with row 9. It sits better on the neighbour read or the domain than on the column, since two readers of one column may want different edges. |
| 13 | `Roles` (`member.rs:73-76`) | The list of every column a kernel names, written out by the author | The union of read and write sets, which the existing code computes: `Concat`, `AccumRead`, `AccumWrite`, `access.rs:81-93`, `work_unit.rs:230-256` | SAME, and it should not be written by an author. The branch's own ledger shows it cost the frozen author an edit: `hilavitkutin-frozen-authors/BREAKS.toml`, "names its role list: one role for each column it reads or writes". |
| 14 | `Value`, sealed, with `Int<BITS>` its one type (`domain/value.rs:114-123`, `num/int.rs:167`) | What a column may hold | Design `hilavitkutin-api/DESIGN.md.tmpl:219-231`. Code `column_value.rs:25-29`, `impl<T: Copy + 'static> ColumnValue for T` | SAME, narrowed from every `Copy` type to one. Refused above. |
| 15 | `Cell8` to `Cell64`, `Word` (`value.rs:13-107`) | Every cell stored as eight little-endian bytes | Design `hilavitkutin-api/src/storage.rs:31-33`: "type-native stride (`size_of::<T>()` per column, no universal stride)" | OUT of the contract. A universal eight-byte cell is one store's private layout. As a public trait it is the universal stride the design rules out. |
| 16 | `FormatId` (`value.rs:35`) | A run-time tag compared at bind | Design `:163-165`: "the monomorphised function IS the type identity, no TypeId". The nearest ratified thing is the erased static-shape addressing he chose for resources on 2 July (`mock/design_rounds/202607161000/202606210600_topic.hybrid-decision.md`), and `hilavitkutin-persistence/src/primitives.rs:71` (`BitWidth`) | EXT of the erased-shape descriptor, to columns. It has a place only if columns may be bound from data (row 17). |

### 1.2 The bind

| # | Branch item | What it is | Existing concept | Verdict |
|---|---|---|---|---|
| 17 | `Binding`, `Role` as plain data (`bind/mod.rs:46-74`) | A table row that says which stored column fills which column of which kernel | Registration by type: `.with(value)`, design `hilavitkutin-api/DESIGN.md.tmpl:34-37` | NEW, as a second way to register beside the typed one, and only if he wants content to add columns without Rust. That is the panel's call 9, on which no seat had evidence (`panel-consolidation-1.md:305`). The branch built its whole contract on the unanswered default. Its place would be `hilavitkutin-kit` or the builder. |
| 18 | `UnitId` (`bind/mod.rs:34`) | A unit's number, declared in the table | Code `hilavitkutin-api/src/dispatch_codegen.rs:191`, `pub struct UnitId(pub Uint<16>)`, a dense index the plan assigns | SAME name, second meaning. The plan's own index serves. |
| 19 | `bind` (`bind/mod.rs:460`) | Check the table against kernels and store, once | `SchedulerBuilder::build`, design `hilavitkutin-api/DESIGN.md.tmpl:815`, code `hilavitkutin/src/scheduler/builder.rs:167` | SAME, moved from compile time to run time. |
| 20 | `BindRefusal` (`bind/mod.rs:81`) | Why the table did not bind | Code `hilavitkutin/src/scheduler/build_error.rs:21`, `BuildError` | SAME. |
| 21 | `TwoWritersOfOneColumn`, refused | Two writers of one staged column have no order | Design `hilavitkutin-api/DESIGN.md.tmpl:366-368`: "Write-write ambiguity resolved by scheduling hint priority across three dimensions. Tie-break: most deps first, then deterministic fallback." | COR, scoped. The fault: priority orders two writes in a pipeline where the second overwrites the first, but for a column whose writes are staged and swapped whole there is no first and second, so a priority has nothing to resolve. Evidence: S3 FINDINGS, fixtures 05, 06 and the data-bound twin. It corrects the sentence for the staged kind only; for a pipeline column the sentence stands. The supersession is his. |
| 22 | `Bound` (`bind/mod.rs:380-437`) | The checked table that holds the store | The built `Scheduler` with its `ExecutionPlan`, code `hilavitkutin/src/plan/mod.rs:66` | SAME. |
| 23 | `Col<D, V>` (`bind/mod.rs:129`) | A typed handle to a stored column, minted by the bind | `StoreId` with the inferred index witness, design `hilavitkutin-api/DESIGN.md.tmpl:463-471` (`ColSelector<T, I>`), code `id.rs:16` | SAME, minted at run time. |
| 24 | `Unit`, `gather`, `absorb`, `words` (`bind/mod.rs:159-200`) | One unit's read handles paired with the committed view | Design `hilavitkutin/DESIGN.md.tmpl:147-160`, the per-unit `EngineCtx<'frame, R, W>` that "holds only the projected stores it declares". Code `hilavitkutin/src/dispatch/engine_ctx/mod.rs:119` | SAME. |
| 25 | `Staging`, `split`, `scatter`, `low`, `high`, `Outside` (`bind/mod.rs:203-275`) | One unit's exclusive write part, cut by position, writes refused outside it | `ColumnWriterApi::write`, code `context.rs:123-132`: "unsafe fn write ... Caller must hold the exclusive writer slot" | COR. The fault: exclusivity is a comment the plan is trusted to keep and the author's body is `unsafe`. Evidence: `context.rs:107-132`; S3 FINDINGS, "the staged columns of two kernels run on two scoped threads, with no unsafe anywhere in this form". |
| 26 | `Units`, `UnitFn`, `StagedFn`, `StepFn` (`bind/mod.rs:277-366`) | The interface a runner in another crate walks the units through | Design `hilavitkutin/DESIGN.md.tmpl:177-183`, `RunFiber` over `WuCons` cells; and design `hilavitkutin-api/DESIGN.md.tmpl:372-378`, `Plannable`, "so a consumer whose runner is not the fiber engine can declare and plan work". Code `hilavitkutin/src/dispatch/fiber_run.rs:52`, `work_unit.rs:117` | EXT of `Plannable`: the design already admits a runner that is not the fiber engine but gives it a declaration only, no typed access to what it declared. |

### 1.3 The store and the frame

| # | Branch item | What it is | Existing concept | Verdict |
|---|---|---|---|---|
| 27 | `Read` (`store.rs:26-34`) | The committed view every read of a step reads, shared between workers | Design `mock/DESIGN.md.tmpl:90`: "`ResourceSnapshot<'phase, R>` read-only typed view captured at a phase boundary; lifetime bounded to the current phase (no lift-out)" | EXT of the snapshot from resources at a phase to columns at a frame. |
| 28 | `Stage`, `Taking`, `Opening` (`store.rs:45-85`) | The staged side, cut by column and by position | None beyond row 25 | EXT, same correction as row 25 seen from the store. |
| 29 | `Store` (`store.rs:89-111`) | The trait a storage implementor writes | Design `hilavitkutin-api/DESIGN.md.tmpl:262-282`, `ColumnStorage`. Code `storage.rs:38-80`, over `MemoryProviderApi` (`platform.rs:28`) | SAME: a second storage contract. It takes no memory provider, where his words of 12 November are "Hilavitkutin also comes with its own memoryprovider contract for the columnar data". |
| 30 | The frame as step, then swap (`store.rs:106-110`, `reference/src/lib.rs:192-219`) | Reads see the last committed frame, writes are staged, a swap commits them | None. In the existing model a reader ordered after a writer sees that writer's output in the same pass (design `hilavitkutin-api/DESIGN.md.tmpl:151`, "`Write<Resource<X>> -> Read<Resource<X>>` builds the same edge") | NEW. It is the one real difference of model. In hilavitkutin's own terms it is a store kind: a sibling of `Column<T>`, as `Accum<T>` was added (`store.rs:110-119`, "Sibling of `Column<T>`"), whose reads resolve to the committed side, whose writes are exclusive, and which builds no read-after-write edge. S3's FINDINGS says the same in its last line: "In hilavitkutin's vocabulary a frame lane is a `Column<T>` stepped per frame". kaski's need: `engine.toml:396`, `:678`. |
| 31 | "frame" | One step, swap and digest | Design `mock/DESIGN.md.tmpl:141`: "Schedule once, reuse across frames"; a pass | SAME word, same thing. |
| 32 | `Morsel` (`store.rs:115`) | A non-zero record count | Design `mock/DESIGN.md.tmpl:82`. Code `hilavitkutin-api/src/run_cfg.rs:85` (`MORSEL_SIZE`), `hilavitkutin/src/plan/inputs.rs:20`, `dispatch/morsel.rs:31` | SAME. |
| 33 | `Visit`, `Visitor` (`store.rs:133-146`) | Which records, in which morsels, in which order, written by an implementor | Design `mock/DESIGN.md.tmpl:80-86`, the morsel loop, owned by the engine | SAME, and on the wrong side. Under his property the order of visiting is plumbing. As an implementor's contract it gives a consumer something to rewrite when the plumbing grows. A permuted order is a fine instrument inside a test kit (row 46). |

### 1.4 Numbers and lists

| # | Branch item | What it is | Existing concept | Verdict |
|---|---|---|---|---|
| 34 | `Bool`, `USize`, `Identity`, `Additive` (`num/`) | Proxies for arvo's types | arvo's own, already imported: code `hilavitkutin-api/src/access.rs:57`, `<USize as Identity<Additive>>::IDENTITY` | SAME, to the name. |
| 35 | `Index`, `Count`, `Ordinal` (`num/index.rs`) | Position, quantity and place as three types that do not convert | None in hilavitkutin; bare `USize` at `context.rs:112` | OUT. They belong in arvo; the branch's design says so itself ("each going at the swap to arvo", `contracts/DESIGN.md.tmpl:5`). |
| 36 | `ColumnNo` | A stored column's number | Code `hilavitkutin-api/src/id.rs:16`, `StoreId` | SAME. |
| 37 | `Lane`, `Int<BITS>`, `Wide<BITS>` (`num/lane.rs`, `num/int.rs`) | Arithmetic a kernel generic over a number is bounded by | arvo, by the panel's own placement (`panel-consolidation-1.md:32`) | OUT. arvo's, with kaski's `Fixed` the stated proxy. |
| 38 | `Cons`, `Empty`, `Here`, `There` from `notko-hlist` (`list.rs:6`) | The type-level list | Code `hilavitkutin-api/src/access.rs:50-53`, a local `Cons` and `Empty` | COR. The fault: the api keeps a second copy of `notko-hlist`'s membership traits (`access.rs:34-39`, `:104-116` against `notko-hlist/src/membership.rs`, panel L19, fault H15). The branch is right to use the one list. The roadmap cut the step that would have fixed it in place (`roadmap.md:446`, "HV4 (adopt `notko-hlist` in the legacy api) is cut"). |
| 39 | `Values` (`list.rs:15`) | A list's nested pair of values | None | OUT. `notko-hlist`'s, and the file says so. |
| 40 | The 128-bit id word (`kernel.rs:32`) | A set of kernel ids | Design `hilavitkutin-api/DESIGN.md.tmpl:155`: "`AccessMask` is the arvo-bitmask `Mask64` (<=64 stores) or `Mask256`" | SAME. |

### 1.5 Digest, runner, laws, frozen crates, lints

| # | Branch item | What it is | Existing concept | Verdict |
|---|---|---|---|---|
| 41 | `Digest`, `Absorb` (`digest.rs`) | A fold of state as fixed-width words, the same on every target | None found in `hilavitkutin-api/src` or `hilavitkutin/src` (grep for `digest`, `fnv`). `arvo-hash` is a pinned dependency (`mock/Cargo.toml:46`) that I could not open | NEW as an instrument. The fold function may be `arvo-hash`'s under a second name; not established. |
| 42 | `step`, `run`, `Run`, `Refusal` (`reference/src/lib.rs`) | A one-thread runner with no plan, kept as the oracle | Code `hilavitkutin/src/scheduler/run.rs:77`, `run_fused.rs:66`, `run_parallel.rs:100`, three entries with nothing they are all held to | NEW. An oracle is what the existing engine lacks most (panel L21). It belongs as the thing `run`, `run_fused` and `run_parallel` are compared with, written against `hilavitkutin-api`. |
| 43 | `digest` of a frame (`reference/src/lib.rs:130`) | Frame number, then every written column in unit and key order | None | NEW, with 41. |
| 44 | `Law`, `Verdict`, `Witness` (`laws/src/lib.rs:18-67`) | A law's outcome as a value naming where it failed | None | NEW. |
| 45 | B1 `model` | A run equals a plain model | None | NEW. |
| 46 | B2 `morsel_split_and_order` | Output does not depend on morsel size, visiting order or layout | Assumed and never stated: the adapt subsystem resizes morsels at run time (design `mock/DESIGN.md.tmpl:58-66`) | NEW. This is a law the existing engine already owes. |
| 47 | B3 `frame_isolation` | A step leaves the committed side alone and does not read stale staged cells | None | NEW, with row 30. |
| 48 | `hilavitkutin-frozen-authors` (`Diffuse`, `Plane`, `Heat`, `Rate`, its model) | A consumer kernel frozen under a hash ledger | The megaround's "test+example bar" (design `mock/DESIGN.md.tmpl:146`), three example apps named and, per the panel, one of them a comment around an empty `main` | NEW. It is the direct instrument of his property. It has to be written against the contract he keeps, or it freezes the wrong surface. |
| 49 | `hilavitkutin-frozen-implementors` (`Columns`, `Records`, `Lanes`, keys; `Forward`, `Permuted`) | Two store layouts and two visiting orders, frozen | Code `hilavitkutin-providers/src/storage.rs:35`, `ArenaColumnStorage`, the one implementation of `ColumnStorage` | NEW as a second store under one contract; its place is `hilavitkutin-providers` and the test kit. The visiting orders are row 33. |
| 50 | `LEDGER.toml`, `BREAKS.toml`, lint `frozen-ledger` | Every edit of a frozen file is a recorded break | None | NEW. |
| 51 | Lints `frozen-no-feature-gate`, `frozen-no-recursion-limit`, `frozen-no-foreign-turbofish` | A frozen crate is written as a consumer could write it | None. Code: `hilavitkutin-api/src/lib.rs:11` itself carries `recursion_limit = "512"` | NEW. Repo lints in `mock/lints/`, as placed. |
| 52 | `mock cross-target` tool | Digest streams equal on the host and under wasm in node | None | NEW, a tool in `mock/tools/`, as placed. |

Two things the branch changes outside its new crates. It rewrites the root `README.md` (200 lines changed) around the new kernel form, and it marks `hilavitkutin-api`'s README and crate doc as legacy. Both are public surface and both state the refused sentence.

Counts, one verdict per row, 52 rows. SAME 20: rows 3, 6, 7, 8, 11, 13, 14, 18, 19, 20, 22, 23, 24, 29, 31, 32, 33, 34, 36, 40. EXT 7: rows 1, 9, 10, 16, 26, 27, 28. COR 3: rows 21, 25, 38. NEW 17: rows 4, 5, 12, 17, 30 and 41 to 52. OUT 5: rows 2, 15, 35, 37, 39. Of the 17 new, 12 are instruments (digest, oracle, laws, frozen crates, ledger, lints, tool) and 5 are contract (domain, offset, boundary value, the frame-buffered store kind, the data-bound table, the last conditional on his call).

## 2. The reverse map

What the existing design has that the branch's vocabulary has no word for, and what becomes of each if the branch's vocabulary were the contract.

| Existing concept | Where | In the branch | If the branch's vocabulary became the contract |
|---|---|---|---|
| Read-after-write between units in one pass, the dependency graph | Design `hilavitkutin-api/DESIGN.md.tmpl:150-152`; `mock/DESIGN.md.tmpl:48` (12 steps) | Absent. Every read is of the committed frame, so no unit depends on another within a step | The plan loses its input. `build_dag`, topo sort, waists, RCM, block diagonal, spectral partition and fiber grouping order nothing, because there are no edges. A pipeline consumer (the design's "signal / DAG frameworks, language runtimes", `mock/DESIGN.md.tmpl:160`) cannot be written at all: stage two cannot read what stage one wrote this pass. |
| Within-fiber fusion of a chain | Design `hilavitkutin/DESIGN.md.tmpl:139-145`, `:361-379`; code `dispatch/fusion.rs:56`, `:126`, `:162` | Absent, and with no chains nothing to fuse | The mechanism he built to reach a hand-fused loop has no case. What "fusion" could mean under the branch is running several units in one pass over the records, a different thing (S1's "fused, one pass per row"). |
| Phases, waists, trunks, fibers, branches, bridges | Design `mock/DESIGN.md.tmpl:108-112`, the vocabulary line | Absent | The whole hierarchy between "pipeline" and "morsel" has no referent. |
| Morsel sizing from SIMD width and the L1 write budget; micro-morsels; the 3D morsel cube | Design `mock/DESIGN.md.tmpl:82`; `plan/steps/step9_morsel.rs` | `Morsel` is a number the caller passes | Survives only if the runner computes it. Nothing in the contract carries a column's width to the plan, since every cell is a `Word`. |
| Per-core dispatch programs, `DispatchCodegen`, `FiberShape`, `CoreProgram` | Design `mock/DESIGN.md.tmpl:52-56`; code `dispatch_codegen.rs:234`, `:368`, `:428` | `StepFn`, `StagedFn` | Could sit behind `StepFn`. The cut by unit then by position is narrower than a per-core program that interleaves units. Not established whether a compiled per-core program can be expressed through `Units::next`. |
| `Executor`, thread pool, parking, `PoolFrame`, P and E cores | Design `mock/DESIGN.md.tmpl:68-78`; code `platform.rs:264`, `:414` | Absent; a test uses `std::thread::scope` | Untouched and unreachable: the contract has no platform traits at all. |
| `MemoryProviderApi`, `ColumnStorage`, 64-byte alignment, type-native stride, raw base pointers | Code `platform.rs:28`, `storage.rs:38` | `Store`, per-cell `get` and `put` through a handle | The provider contract he named is bypassed. Per-cell access through an eight-byte word replaces a base pointer and a typed stride; its cost is unmeasured. |
| `Resource<T>` as a real singleton with a lifetime, `Field`, `Seq`, `Map`, `ResourceFootprint`, the snapshot before the morsel loop, the hybrid erased addressing | Design `hilavitkutin-api/DESIGN.md.tmpl:300-313`; `mock/research/202606210600_storage-bench-findings.md` | `Param` coefficient, one word | The resource model he benched and decided on 2 July has no counterpart. A `Seq` or `Map` resource cannot be expressed. |
| `Virtual<T>`, `On<V>`, `OnMeta<V>`, the fired flag, the self-hosting meta pipeline | Design `:147`, `:362-364`; `hilavitkutin/DESIGN.md.tmpl:734-925` | Absent | No conditional scheduling, no events. Principle 5, "The scheduler schedules itself", has nothing to stand on. |
| `Accum<T>`, append, drain at pass end | Code `store.rs:119`, `context.rs:146-170` | Absent at this commit (emission is the roadmap's next slice) | Would be rebuilt as a new emission form rather than corrected in place, though the panel's faults H1 to H6 are all stated against the existing lines. |
| Scheduling hints, `COMMUTATIVE` | Code `hint.rs`, `work_unit.rs:84` | "It carries no hint and no commutativity constant" | Dropped. The panel finds `COMMUTATIVE` read by nothing (L11), so that half is a fair deletion; the hints price units in the plan and go with the plan. |
| `Plannable`, `HintExt` | Design `:370-419` | Absent | The design's own seam for a second runner is left unused and a parallel one is cut. |
| `Kit`, `Provider`/`BuilderInput`, `.with`, `Replaceable`, `PlanAffecting`, plan caching and the dirty bit | Design `:32-136`, `:530-900`; `mock/DESIGN.md.tmpl:92-100` | A slice of `Binding` | No composition of bundles, no typestate proof of coverage at build; every refusal the build made at compile time becomes a run-time value. S3 says this in so many words: "The price is that the three refusals the typed form makes at compile time become values." |
| Adapt, the nine axes, metrics as resources, anomaly virtuals | Design `mock/DESIGN.md.tmpl:58-66` | Absent | Unreachable, since it lives on resources, virtuals and the meta pipeline. |
| `hilavitkutin-persistence`, `PersistenceProvider`, evict and inject | Design `mock/DESIGN.md.tmpl:94`; `hilavitkutin-persistence/` | Absent | Unreachable. The roadmap's journal and keyframe would be a second persistence beside it. |
| `hilavitkutin-ctx`, the provider tuple, `HasX` accessors | Design `:424-496` | Absent by intent | Dropped. This one the evidence supports in part (section 5). |
| `hilavitkutin-extensions`, `-linking`, `LinkedBin` | Design `:193-214` | Absent | Unreachable, and the sealed `Value` and crate-made `FormatId` close the door a plugin would need. |
| `hilavitkutin-str`, `-sym`, the interner kit | `mock/DESIGN.md.tmpl:24-26` | Absent | Unaffected, standalone. |
| `hilavitkutin-build` | `mock/DESIGN.md.tmpl:17` | Absent | Unaffected. |

Judgement. The branch does not cover hilavitkutin; it covers one new store kind and one author form, and re-states about twenty existing things in order to stand without the rest. If its vocabulary became the contract, the engine he designed would not be ported onto it "as another runner". It would have nothing to do there: its planner needs edges the contract cannot express, and its storage, resources, virtuals, kits and adapt have no way in.

## 3. The research he remembers

Searches run. `cargo mock unearth` does not exist in this repository (`cargo mock` lists no such tool; project tools are `coverage`, `near-copies`, `rulings-with-no-verbatim`). So: a grep over `mock/research`, `mock/design_rounds`, `mock/benches`, every `DESIGN.md.tmpl` and `PRINCIPLES.md.tmpl` for `bitpack`, `BIT_WIDTH`, `type-native stride`, `universal stride`, `sub-byte`, `packed width`, which named 60 files; a listing of every bench by name; and a read of the `202606210600` storage-model set, the bench findings of `float_width`, `fxpmul_strategy`, `sat_arithmetic`, `load_alignment`, `cache_layout`, and `benches/engine_vs_std/FINDINGS.md`. I did not read the 500-line loimu heritage file, `seed/data-model.md`, or the polka-dots imports.

What was found, committed files only.

The design already says columns are generic and user-chosen. `hilavitkutin-api/DESIGN.md.tmpl:228-237`: "Any `Copy + 'static` type is column-storable ... BIT_WIDTH is a future hook for sub-byte bitpacking ... No pack/unpack today: the type IS the storage (R3, R4)." And `storage.rs:31-33`: "64-byte aligned with type-native stride (`size_of::<T>()` per column, no universal stride)".

The panel-and-bench he most likely remembers is the six-variant storage-model round, `mock/research/202606210600_storage-bench-findings.md`, on the harness, Apple M1, two full runs. It asked whether a type-erased, shape-described store (V4, "loimu-style type-erasure-via-shaping") costs anything against native monomorphised access (V1). Quoted: "The v4/v1 median ratio across all nine scalar arms, both runs: run-1 spanned 0.946x to 1.049x; run-2 tightened to 0.991x to 1.017x. Every arm is within +-1.7%" (`:334-335`). "V4 carries no measurable scalar-latency penalty against V1 on this host" (`:337`). His decision is recorded at `:348`: "Resolved (op, 2026-07-02): go with the hybrid, global-capable." The same file prices the layouts that lose: decomposed per-member columns 1.90x to 3.06x on a 64-member gather (`:300-303`), a run-time handle table "+8% to +16% on the N=1024 best-20%" (`:295-296`), shared shape-bound columns 3.29x and 3.44x false sharing (`:195`, `:323-324`), and a live-streamed collection against a snapshot copy 0.384x to 0.394x at 64 MiB (`:316`). Its predicate, as the file states it: one host, modelled kernels and not the wired engine (`:258-266`). It is about resources. I found no bench of the same question for columns.

On widths and formats of a value, the microkernel benches, all on the harness:
- `mock/benches/load_alignment_n4096_findings.md:8-9`: aligned cast, `read_unaligned` and a `from_le_bytes` pack, "Spread: 1.01x (fastest 603.0 ns, slowest 609.0 ns)"; at N=256 1.04x, at N=16384 1.01x. So a little-endian byte cell, the branch's `Word`, is not by itself slow to load. That is one load pattern, not the branch's per-record `get` through a handle and a format.
- `float_width_n4096_findings.md:8-11`: f32, f64 and i64 fixed point, "Spread: 1.29x (fastest 1290.2 ns, slowest 1663.3 ns)"; 1.62x at N=256.
- `fxpmul_strategy_n4096_findings.md:10`: i128 intermediate against i64 truncate, "Spread: 1.30x"; 1.45x at N=256, 1.08x at N=16384.
- `sat_arithmetic_n4096_findings.md:8-11`: "Fastest: sat_wrap at 1397.9 ns median (-47.1% vs baseline)", "Spread: 1.91x"; 2.02x at N=16384. The branch's one lane, `Int<BITS>`, is "signed and saturating".
- `cache_layout_n4096_findings.md:9`: array of structs against columns, "Spread: 1.03x".

On "match or surpass as ceiling". `mock/benches/engine_vs_std/FINDINGS.md` measures the single-core engine against a hand-fused std loop and finds the engine 2.1x to 4.2x slower on a four-stage chain, with "the designed 0.95x to 1.02x parity" as the bar a standing red gate holds it to. The file calls itself a timing test; the panel calls it a spike (L22). The design round that added `RecordOp` fusion (`mock/design_rounds/202606092000/202606092000_topic.d4-within-fiber-linear-fusion.md`) cites the sketches as proving register residency. I found no committed number showing the gate turned green after fusion landed. The 1.28x to 1.40x noalias figure in `storage.rs:30` is, by the storage bench's own word, "NOT a current-repo measurement" and was not reproduced as wall clock on M1 (`:63-66`, `:111-113`).

Judgement. What the record supports is his memory in substance: erased, shape-described storage was benched at parity with monomorphised access, and he chose it, for resources. What the record does not hold is the same measurement for columns, or any measurement of a column whose format is chosen at run time. The branch did not use any of this. It picked a sealed value type and a universal eight-byte cell with no bench and against the design's stated stride.

## 4. Shapes he can choose among

### Shape A. The needed forms as a revision of `hilavitkutin-api`, in place

What moves where.
- The frame-buffered column becomes a store kind in `hilavitkutin-api/src/store.rs`, a sibling of `Column<T>` and `Accum<T>`, any `ColumnValue`. The plan builds no read-after-write edge for it (`plan/steps/step1_dag.rs`), `build` refuses a second writer of it as a `BuildError`, and the scheduler swaps at pass end. Supersedes, for that kind only, `hilavitkutin-api/DESIGN.md.tmpl:366-368`.
- A record domain becomes a parameter of a column and a per-domain record count in the builder, replacing the single count at `scheduler/builder.rs:170`.
- The value kernel becomes `RecordOp` widened: read list in, write list out, over the existing access sets, with `Near<C, O>` added as a member of a read set. A unit that is a value kernel writes no `Ctx` and no `execute`; the engine derives both, as `hilavitkutin/DESIGN.md.tmpl:164-165` already says the context is "pure type functions of the access sets". Supersedes the `type Ctx` line of `hilavitkutin-api/DESIGN.md.tmpl:328-334` for value kernels; `execute` stays as the block form.
- `access.rs`'s local list is replaced by `notko-hlist` (row 38).
- The laws, the digest, the oracle runner, the frozen crates, the ledger lint and the three frozen lints and the cross-target tool are kept, as a conformance kit written against `hilavitkutin-api`: `hilavitkutin-laws` and `hilavitkutin-reference` stay as crates, the frozen crates stay, all rewritten onto the api's names.
- A data-bound table, if he wants content to add columns without Rust, is a second registration mode in `hilavitkutin-kit`, built on the erased shape descriptor of his 2 July decision, after a bench of it for columns.

What the existing scheduler changes and when. First landing: the store kind, the missing edge, the swap, on the single-thread `run` only. Second: the widened value form and the derived context. `run_fused` and `run_parallel` follow under the oracle, one entry at a time. Nothing of the planner is removed.

What is deleted from the branch. `hilavitkutin-contracts` as a crate; `num` (to arvo, or dropped where arvo's types already serve); `Value`, `Cell`, `Word`, `FormatId` as public contract; `KernelId`; `Roles`; `Visit` as an implementor's trait; the legacy sentences and the README rewrite.

What stops building meanwhile, and the bridge. The frozen crates and the reference runner stop building until the api carries the store kind and the value form. The bridge is the branch itself, unmerged, kept as the working model the rewrite is diffed against; its digests are the first expected values. The harder block is arvo: the api pins ten arvo crates at rev `1b2796e9` over ssh (`mock/Cargo.toml:39-48`), and whether `dev` builds in a cloud container today I did not establish. If it does not, this shape waits on that or on arvo's integer slice, which is the reason the branch gave for standing apart.

What a kernel author sees: `impl` of one trait on a unit struct, `type Read = read![...]`, `type Write = write![...]`, and a function from values to values. Column markers are his own types, any `Copy` value. No id, no role list, no context.

What a store implementor sees: `ColumnStorage`, corrected (the panel's H21: growth, a stated re-reserve), over `MemoryProviderApi`, plus two sides for a frame-buffered column. No visiting order.

### Shape B. The reference runner and the laws as the oracle of the existing scheduler, and nothing else yet

What moves where. Only rows 41 to 52 land, rewritten against today's `hilavitkutin-api` with no new store kind: an oracle that runs registered work units one record at a time in registration order, laws B1 and B2, frozen authors that are plain work units, the ledger and lints. The existing engine's three entries are compared with it. The staged column, the domain and the value form wait for a design round he sits in.

What the existing scheduler changes: nothing at first; then whatever the laws find. The panel's step 0 list (H2, H9, H17 to H19, H22) is the expected first catch.

What is deleted from the branch: all of `hilavitkutin-contracts`, the frozen implementors' stores, B3.

What stops building: nothing on `dev`. kaski's lattice step has no upstream form until the round; kaski keeps its own double buffer meanwhile, which his words of 12 November rank second ("Consider the upstream work preferred over bespoke non-reusable work only in kaski").

What an author sees: today's `WorkUnit`, unchanged. What an implementor sees: today's `ColumnStorage`, now with a second implementation under one suite.

### Shape C. The branch as it is

Five crates beside the engine, the api marked legacy, the engine "ported later as another runner".

What the existing scheduler changes: per the roadmap, everything at HV18, behind six arvo crates. Per section 2, the port has no target: the contract cannot express what the planner plans.

What stops building: nothing now. Two contracts stand in one repository with no end date either seat could give (`stamps-2.md:167`, "Two contract crates in one repository with an end date behind six arvo crates").

What an author sees: `ValueKernel` with `Roles`, `Reads`, `Writes`, an `ID`, values as nested pairs, one sealed integer type. What an implementor sees: `Store`, `Read`, `Stage`, `Visit`.

It is listed because it is on the table. It is refused by gate one as written, and would need him to supersede `hilavitkutin-api/DESIGN.md.tmpl:219-231`, `:359-360`, `:366-368`, `storage.rs:31-33` and the crate table of `mock/DESIGN.md.tmpl`, and to say the engine is no longer the paradigm.

### Shape D. A, taken in the order of B

B first, as a short landing with no design question in it. Then A's design round with him in it, with the oracle and the frozen crates already there to hold each step. The branch stays unmerged as the model.

### Which I would take

D. The reasons, in order of weight.

1. It is the only order in which nothing he has not seen becomes contract. B's content is instruments; A's content is three additions to his own vocabulary, each small enough to say in a sentence and each needing his word on one superseded line.
2. The branch's good parts survive whole: the laws, the mutants, the ledger, the oracle, the staged model and its typestate (S3), the value form (S1). What goes is the re-statement.
3. His property is tested from the first landing against the contract that will be kept, so a break recorded in `BREAKS.toml` is a break of the real surface.
4. It does not close the data-bound table. It makes it a second mode with a bench in front of it, which is what "if both ways would have users, then configurable via consts and generics" asks.

What would change my mind.
- If `hilavitkutin-api` cannot be built or changed until arvo's slice lands, and that is weeks away, then the notko-only crate is a fair bridge, but as the panel stamped it: named and shaped to be moved into `hilavitkutin-api`, holding only rows that are NEW or EXT, with no word "legacy" anywhere.
- If he rules that columns are bound from data as the primary form (call 9, option a), the typed access sets stop being the main surface and the branch's `Binding` and `Col` move from second mode to first. The store kind, the domain and the value form still belong in the api.
- If a sketch shows the widened `RecordOp` cannot be given a derived context without the author naming the engine (the L7 fault surviving), the value form needs the per-handed-thing traits of S1's arm E2, and the `HasX` context goes for value kernels.

## 5. Which contract serves his property better

His property: growth in the plumbing demands no change on implementors.

What the existing contract does, code. An author's unit names a context type, `work_unit.rs:72-78`, and the design tells the author to assign it the engine's alias `CtxFor`, `hilavitkutin/DESIGN.md.tmpl:162-163`, defined in the engine crate at `hilavitkutin/src/dispatch/engine_ctx/bundles.rs:132`. So an author's crate names a runner. The body calls `unsafe` `read(i)` and `write(i, v)` at a bare index (`context.rs:112`, `:129`), is run "once per morsel" (`work_unit.rs:86-95`), and through `BatchApi::run` is handed the morsel's index range (`context.rs:226-230`). An author can therefore see the morsel, the index and the engine's type. Any growth that changes one of the three can change what an author's code does or whether it compiles.

What the existing contract already has that does not leak, code. `RecordOp::apply(&self, x: In) -> Out`, `record_op.rs:41`: no index, no morsel, no context, no runner. And `Plannable`, `work_unit.rs:117-124`, a declaration that "names no runner".

What S1 found (`mock/research/sketches/202611122252_runner-generic-workunit/FINDINGS.md`, holds for: nightly-2026-05-28, x86_64 linux, a 16 by 16 world, 6 ticks, `i32` members). Two arms: E1, a context the runner supplies; E2, one small trait per handed thing. "In both arms the unit's source is unchanged under all five runners" (ordered, permuted, fused, threaded, a 64-store registry), by file hash. On growth by one capability: "E1 edited 5 of 5 existing runner crates ... E2 edited 0 of 5. In both arms the unit hash did not move." So for authors, both a runner-supplied context and handed values kept the property; the context form cost every runner an edit per new capability.

What S3 found (`202611122242_frame-view-typestate/FINDINGS.md`, same toolchain). The committed view, write-only staged handles and the close unit hold as compile errors; the view is `Sync` with no `unsafe impl`; "The contract can rest on the data-bound form; the type-level form holds only for lists fixed in the engine's types". In hilavitkutin the lists are fixed in the engine's types: that is what `.with` and the typestate builder do. So S3's typed form, fixtures 05 and 06 as build-time errors, is the one that fits the existing design, and S3 shows it works to 32 lanes at the default recursion limit and to 128 with the limit raised above 256.

What the branch's own ledger shows. Within its one slice the frozen implementors were edited four times (`hilavitkutin-frozen-implementors/BREAKS.toml`), two of them so that a threaded runner could exist: "hands its staged side out as lanes cut by write set and by position, its committed view Sync" and "keeps its staged side a column at a time so that it cuts by column". The frozen author was edited once, to add `Roles`. These are pre-lock edits and the ledger is honest about them, but they are the property failing on its first exercise on the implementor side: threading arrived and the store contract changed.

Judgement. The property is served by the value form, and the value form is already hilavitkutin's: it is `RecordOp`, opt-in and one-to-one, where it should be the main door and list-to-list. It is not served by `WorkUnit` as it stands, for the three leaks above, and not served better by the branch's separate trait family than by the same form inside the api, because what protects the author is the signature (values in, values out, nothing else in reach), not the crate it is declared in. On the implementor side the branch is the weaker of the two: it puts the visiting order and the cut of the staged side in the implementor's hands, where the existing design keeps both in the engine and asks an implementor only for memory and columns.

Not established. Whether a widened `RecordOp` with a derived context compiles against the existing `provider_generic!` accessors and index witnesses (S1 used its own `Has<M, P>` on `notko-hlist`). Whether any of this holds under morsels: "no runner here works in morsels" (S1, `:110`). Whether the value form costs anything against the `each` closure or a hand loop: S1's asm note is "an ad-hoc reading ... not a measurement", and no bench exists. Whether the existing planner, fusion and per-core dispatch can run value kernels over a frame-buffered column with no change to the author; that is the claim the whole lean rests on and nothing has tried it.

## 6. What I could not establish

- Whether `origin/dev` of hilavitkutin builds today, here. It pins arvo at a rev over ssh; I ran no build.
- Whether the branch builds or its tests pass. I ran nothing, and of its tests I read part of one file, `tests/threads.rs` (gate two).
- Whether `arvo-hash` at the pinned rev already holds the digest's fold. The rev is not in a local clone I looked for.
- Whether fusion closed the `engine_vs_std` gap. No committed number was found after the fusion round.
- Whether the erased-shape parity of the storage bench carries from resources to columns, or to a second host.
- What kaski needs beyond the panel's section 1 and five tenets I opened (`engine.toml:396`, `:559`, `:669`, `:678`, `:689`); I did not read the other rows, and all are provisional by his order.
- Whether content adding a column without Rust is a requirement. It is the hinge between a typed and a data-bound contract and it is his.
- The per-record cost of `Read::get` and `Stage::put` through a handle and an eight-byte word, and of a neighbour read through a key search; unmeasured anywhere.
- Whether the roadmap's later slices (emission, journal, keyframes) map as cleanly onto `Accum`, `Virtual` and `hilavitkutin-persistence` as section 2 suggests. I mapped only what the branch holds at `5603f85`.
- The loimu heritage file, the seed data model and the polka-dots imports, which may hold more of what he remembers on formats than the files I read.
