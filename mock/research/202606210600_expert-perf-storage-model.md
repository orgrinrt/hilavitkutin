# Expert analysis: performance, soundness, and bench gap of the resource handle storage model

**Date:** 2026-06-21
**Scope:** PERFORMANCE / soundness / bench-gap dimensions of the handle + per-member + shape-bound + unified-store model proposed in `mock/research/202606210600_resource-storage-model-canonical-addendum.md`.
**Mandate:** stress the model; do not assume it is optimal; mark every unmeasured claim and design the bench that decides it. No verdict (op's call).
**Oracle precedence:** consolidation spec R5 is canonical (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`); the addendum is an intermediate round and where it diverges from R5 the addendum is the thing to stress, not the truth to adopt (per `canonical-design-outranks-intermediate-rounds.md`).

## A load-bearing premise: resources are singletons

A `Resource<T>` is a singleton store: one `T`, resolved by type, not an array of records. The shipped drain reserves `USize(1)` per resource (`bindings.rs:342`), and the ctx accessor is type-keyed (`resolve_resource`, `engine_ctx.rs:1258`), not index-swept. This is the engine's mental model (singleton store; `Column<T>` is the N-of store). It changes the whole locality analysis below: you never loop member-major across resources, because there is no array of resources to loop over. A shape-bound column shared by resources A and B holds their fields at indices {0,1} and is **never swept**; A and B are each resolved independently by type. So the access pattern is *structurally* resource-major, and the "winner flips by access pattern" symmetry that an ECS array-of-entities model would suggest is false here. The consequence runs through section 2 (Bench B is reframed out) and section 4 (the blob wins the structural common case, not a coin flip).

## Summary of the central finding

The addendum bundles three claims of very different footing under one "this is what R5 says" banner. Separating them is the spine of this analysis:

1. **Handle, not inline blob.** Canonical (R5:1689 "pointer indirection to external slab storage. NOT inline"). The shipped `bindings.rs:22` one-record blob is genuine drift. This part is safe; do not question it.
2. **Unified type-erased store with shape-bound (stride-keyed) shared columns.** This is the addendum's *addition*, unmeasured, and in direct tension with a *retained* R5 layout decision (spec:396-399, co-located per-fiber arena). It is a real layout fork, not a reading nuance.
3. **The noalias win (1.28-1.40x).** The measured mechanism in R5 (spec:1714-1724) is a stack-local **value snapshot**: resources are *copied* into a fresh stack array with distinct provenance before the morsel loop. The addendum (item 6) instead says the **pointers** go stack-local while values stay in the unified store. Those are not the same mechanism, and the second one plausibly forfeits the win the addendum cites as its own justification (section 3 below).

The model is defensible as intent but rests on an unmeasured layout choice that pulls against retained canon and against its own cited justification. Both central forks are bench-decidable and neither has been benched.

## 1. The bench gap, explicitly

### What has actually been measured

- **Column-only workloads.** `mock/benches/engine_vs_std/` covers `element_wise`, `branching`, `accumulator`, `wide_parallel` (src files confirmed). Every arm is `Column<T>`-only; none registers, reads, or writes a `Resource<T>`, a `Seq`, or a `Map`. The standing perf gate (`tests/perf_gate.rs`) gates these four and nothing resource-shaped.
- **The 1.28-1.40x noalias finding is NOT a current-repo measurement.** Spec:1701 attributes it to "T6 L1654-1658". T6 is an early design-distillation topic (2026-03-15, per the spec's topic table lines 17-18), not a hilavitkutin bench artifact in this repo. The current `benches/` tree contains zero resource/provenance/snapshot benchmark (grep for `noalias|provenance|stack.local|resource` over `benches/` hits only prose findings docs and unrelated XOR-shift provenance, never a resource bench).
- **Doubly unmeasured: the mechanism is also unimplemented.** The dispatch path reads a resource directly through its `ResourcePtr<T>`: `resolve_resource` returns `unsafe { &*ptr.as_ptr() }` (`dispatch/engine_ctx.rs:1258-1267`), a borrow straight into store memory with no intervening stack copy, and the per-record read/write go through `ptr.as_ptr().add(idx)` (`:1310`, `:1354`). The read path is a direct deref, not a snapshot. So the 1.28-1.40x is inherited from a March-2026 distillation AND not realised by current code. Any claim resting on it is twice-removed from evidence.

### What has NOT been measured (every open fork)

- **Shape-bound shared columns vs type-unique columns.** No data on column-count blow-up (the addendum's stated reason to reject type-unique) versus the access-locality cost of sharing one `Field<u32>` column across many unrelated resources. The "type-unique is too wide" claim is asserted, not measured.
- **Per-member decomposition vs one-record blob.** No data comparing scattered member columns against a co-located resource blob for the two access patterns that flip the answer (see section 4).
- **Morsel-window-internal member fetch vs stack-local cache fork.** The addendum (item 6) lists this as an explicit open fork. Unbenched; also the deeper soundness question (section 3).
- **Cache behaviour of scattered member columns vs a co-located resource blob.** This is the single most consequential unmeasured axis, and because resources are singletons (resource-major access is structural), it reduces to: does scattering one resource's members across distinct shape-bound columns cost more cache lines than holding them co-located in a blob? R5's *retained* design (spec:396-399) is per-fiber co-located arena, "all columns in a fiber share one arena allocation... columns across fibers are separate." A unified store keyed by shape scatters one resource's members across distinct stride-keyed columns far apart in the address space. That is the opposite locality decision from canon, and nobody has measured the cost.
- **`Seq`/`Map` ptr+len under morsel windowing.** No data on how a ptr-to-first + length collection behaves when the owning resource is read once per morsel vs per record, nor on resize/replace cost.
- **Column-count effects on the slot table.** `ArenaColumnStorage` caps distinct columns at `Dim<256>` (`storage.rs:37`). Per-member shape-bound decomposition multiplies the column count by the average member arity; whether 256 holds, and the cache cost of a fuller slot table, is unmeasured.

## 2. Concrete bench plan (one bench per fork)

Each bench states inputs, what is measured, and the signal that decides the fork. All run under the existing release profile (fat LTO, cgu=1) and assert checksum equality first, matching `perf_gate.rs` discipline. Add them as new arms in `engine_vs_std` (or a sibling `resource_storage` bench crate) so the standing oracle covers resources too, which today it does not.

### Bench A: noalias / register-residency (the cited-win bench)

- **Inputs.** A morsel loop over N records doing `out[i] = col_in[i] * res.scale + res.offset`, where `res` is a `Resource` with two `Field` scalars read on every iteration. N swept across the existing `SIZES`.
- **Arms.** (A1) resource read live from its store every iteration through `ResourcePtr` (today's code). (A2) resource value snapshotted into a stack-local `let` before the loop, distinct provenance (the R5:1714-1724 mechanism). (A3) resource members read live from the *unified shared* store (the addendum model) while a sibling column is written.
- **Measured.** Wall-clock ratio A1/A2 and A3/A2, plus a disasm check (extend `disasm_5check.rs`) counting resource loads inside the loop body: snapshot should show zero in-loop resource memory ops, live-read should show a reload per iteration.
- **Signal.** If A2 beats A1 by ~1.28-1.40x, the snapshot mechanism reproduces in this repo and is worth implementing. If A3 regresses toward A1 (reloads return), the unified store forfeits the win unless the snapshot step is retained on top of it. This is the bench that confirms or kills the addendum's own justification.

### Bench B: scattered shape-bound members vs co-located blob (the singleton locality bench)

The original symmetric "member-major vs resource-major" design is dropped: resources are singletons, so the member-major sweep across an array of resources does not occur (premise section above). The remaining genuine locality question is intra-resource.

- **Inputs.** One `Resource<T>` with M `Field` members, all M touched inside a WU per morsel (the structural resource-major pattern), N records, N swept across `SIZES`.
- **Arms.** (Blob) members co-located in one record. (Shape-bound) each member in its own stride-keyed column, scattered across the unified store.
- **Measured.** Cycles + L1 line touches for the member-fetch.
- **Signal.** The hypothesis (section 4) is the blob wins outright: one cache line carries the whole resource, while shape-bound forces one line per member from scattered columns, with no offsetting member-major win to recover (there is no sweep). If confirmed, the unified shape-bound store's *only* remaining justification for singletons is column-count (Bench C), not locality. The varying-locality question that does exist for resources is `Seq`/`Map` *array* members read within one resource; that lives in Bench D, not here.

### Bench C: column-count / slot-table pressure

- **Inputs.** Sweep R (resource count) and M (member arity) so the distinct-column count crosses 64, 256, 1024.
- **Arms.** Shape-bound (column count ~ distinct member shapes, bounded) vs type-unique (column count ~ R*M, unbounded).
- **Measured.** Plan-build time, slot-table footprint, `column_ptr` resolution cost, and whether `Dim<256>` (`storage.rs:37`) is exceeded.
- **Signal.** Quantifies the addendum's "type-unique explodes the column count" claim. If shape-bound stays under 256 while type-unique blows past it on realistic consumer shapes (viola/loimu resource sets), the addendum's column-count argument holds; otherwise the cap is the real constraint, not the layout.

### Bench D: `Seq`/`Map` ptr+len under morsel windowing + resize

- **Inputs.** A resource with a `Seq<T, N>` member, read once per morsel and (separately) per record; plus a replace/resize cycle.
- **Measured.** Per-morsel fetch cost vs per-record, and resize/replace cost (consecutive-stride compaction).
- **Signal.** Confirms whether ptr+len over consecutive strides interacts cleanly with the morsel formula `L1_usable / (Σ write_sizes)` (`footprint.rs:3-8`), and whether resize forces a full column rewrite.

## 3. Soundness / aliasing analysis

### Does the unified-store + separate-handle-provenance model preserve the noalias guarantee?

**Not automatically; it specifically threatens it.** The reasoning is first-principles LLVM semantics from the spec text itself (1695-1700): noalias is a property of the *pointed-to memory's provenance*, not of where the pointer variable is stored. The win in R5 comes from the resource value living in a **stack-local region** (spec:1703 "resource data lives in a stack-local region", 1716-1718 "snapshots all accessed resources to a stack-local array") that LLVM can prove disjoint from the heap column data being written.

The addendum (item 6) keeps the *pointers* stack-local but leaves the *values* in the unified store. If a resource member is read from the same unified store that a column is written to, the read pointer and the write pointer share the store's provenance. That is exactly the case spec:1695-1700 names: "two raw pointers from the same struct may alias... LLVM cannot prove that a `cw()` store to one column doesn't affect a `ru()` read from a resource... LLVM reloads resources from memory every iteration." So the unified-store model, *as written*, reintroduces the aliasing it cites as the problem it solves. The fix is to retain the stack-local **value snapshot** on top of the unified store (copy the member into a stack `let` before the loop), at which point the "unified store" is only the canonical home and the hot path reads a stack copy. That snapshot step is not in current code (section 1) and is not stated in the addendum.

Frame the conclusion as the hypothesis Bench A decides, not as a settled verdict; but the mechanism is solid enough that the burden is on the unified-store model to show the win survives.

### Type-erased shared shape-bound columns: UB risk from multiple resources aliasing one column

A shape-bound `Field<u32>` column shared by many resources means several resources' members live in one type-erased buffer at distinct indices. Soundness requires: (a) each resource owns a distinct index/slot in the shared column (no two resources write the same slot), enforced at handle-construction time; (b) the type-erasure round-trip is consistent (`reserve::<T>` then `column_ptr::<T>` with the same `T`, as `storage.rs:66/114` already require). The risk is a write through resource A's handle and a read through resource B's handle to the *same shared column*: LLVM sees one allocation, so it cannot assume A's write does not affect B's read, which (again) defeats register residency for any resource whose column is also written by a sibling. This is a sharper version of the section-3 aliasing problem: shape-sharing maximises the number of distinct logical values packed into one provenance domain. Bench A arm A3 measures it. No memory-unsafety as long as indices are disjoint, but the optimiser cost is real.

### `Seq`/`Map` ptr+len

ptr-to-first + length is sound provided length is validated against the reserved capacity (the existing `AccumBinding` already saturates appends at `cap`, `bindings.rs:120-125`, as the model to follow). The morsel-windowing risk: if a `Seq` member's elements live in a type-erased store and the morsel windows over *records* (not over the `Seq`'s elements), the `Seq` is read whole per resource access, which is fine for read-only (rides L2 prefetcher per spec:551) but counts toward the L1 write budget when written (spec:552-553, implemented in `footprint.rs`). No new UB; the open question is purely cache budget (Bench D).

### Resize / replace

The shipped `ArenaColumnStorage::reserve` frees the prior allocation and reallocates on re-reserve (`storage.rs:70-81`), invalidating every recorded pointer into that column. Under the handle model, a resize of one shape-bound shared column invalidates the handles of *every* resource sharing it, not just the resized one. That is a correctness hazard the blob model does not have (a blob resize touches only that resource). The handle store must be re-resolved after any shared-column resize, or shared columns must be sized once at plan time and never resized (consistent with the schedule-once-reuse model). State this as a hard constraint on the design, not a bench.

## 4. Performance reasoning where a bench is not yet runnable (hypotheses to bench)

- **Blob wins the structural common case for singletons (Bench B).** Because resources are singletons, access is structurally resource-major: a WU resolves a resource by type and touches its members together. Hypothesis: the blob wins outright. One 64-byte line carries the whole resource; shape-bound scatters the members across distinct columns, one line per member, with no member-major sweep to recover the cost (there is no array of resources to stream over). This is not a coin flip by access pattern; the singleton structure removes the pattern that would have favoured shape-bound. Consequence: for `Field` members, the unified shape-bound store's only justification is column-count reduction (Bench C), not locality.
- **Co-located per-fiber arena (retained canon, spec:396-399) likely beats a global unified store for the common case.** Hypothesis: hilavitkutin's dominant access shape is a WU touching a small fixed set of resources + columns within one fiber, repeatedly, per morsel. The retained design puts all of a fiber's columns in one arena with static offsets and one base pointer per morsel (spec:397-398). A global shape-keyed store scatters those across the address space, costing TLB/page locality and forfeiting the single-base-pointer codegen. Note the tension bites specifically on *cross-fiber* sharing: spec:399 says "columns across fibers are separate allocations", so a shape-bound column shared across fibers directly contradicts canon, while within one fiber shape-bound columns could still co-locate. A fiber-shaped macro-bench would quantify it.
- **The snapshot subsumes the layout question for read-heavy resources (Bench A).** Hypothesis: if the stack-local value snapshot is implemented, the *hot-loop* layout of the canonical store stops mattering for read-only resource members (they are copied to the stack once per morsel regardless of source layout). In that regime the unified-vs-co-located choice only affects the once-per-morsel snapshot cost, not the per-record cost, shrinking the stakes of fork 2 considerably. This means the *implementation order* matters: build and bench the snapshot first (Bench A), because its result may make the layout fork (Bench B) low-stakes for the common read path and only consequential for written collections.

## Constraints and open decisions for synthesis

- Fix the blob drift (handle model, claim 1): canonical, do this regardless.
- The snapshot mechanism (claim 3) is the cited win, is unimplemented, and is unmeasured in this repo. Implement + bench (Bench A) before any claim rests on the 1.28-1.40x number.
- The unified shape-bound store (claim 2) is an addendum addition in tension with retained canon (spec:396-399). Because resources are singletons, it has near-zero locality upside (no member-major sweep exists); its only justification is column-count reduction (Bench C), and the locality hypothesis predicts the blob wins the structural case (Bench B). Do not lock it as canonical on the strength of the addendum's reading; R5 is silent on it and the addendum's own gap-1 topic admits the silence.
- Shared-column resize invalidates all sharers' handles: hard constraint, size shared columns once at plan time or re-resolve handles after resize.
- For `Field` members the layout choice reduces to column-count (shape-bound) versus locality (blob); for `Seq`/`Map` array members the within-resource access question (Bench D) is the live one.
