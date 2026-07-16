# Expert synthesis: loimu storage heritage vs hilavitkutin resource model

**Date:** 2026-06-19
**Round:** 202606210600
**Purpose:** Durable deliverable for the pressure-test topic
(`mock/design_rounds/202606210600_topic.storage-model-pressure-test.md`). Mines
loimu's storage design, extracts the how and why, then assesses whether
hilavitkutin's handle + per-member + shape-bound + unified-store model has blind
spots relative to loimu's reasoning.
**Sources:**
- `~/Dev/loimu/mock/discussions/2026-02-17_unified-storage-model-discussion.md`
- `~/Dev/loimu/mock/design_rounds/unified-storage-animation-gpu/2026-02-17_unified-storage-model.md`
- `~/Dev/loimu/mock/design_rounds/unified-storage-animation-gpu/2026-02-21_unified-storage-model_changelist.md`
- `~/Dev/loimu/docs/LOIMU_STORAGE_MARKER_STORAGE.md`
- `~/Dev/loimu/mock/DESIGN.md.tmpl` (resource model sections)
- hilavitkutin consolidation spec R5 (lines 535-566, 1682-1710)
- `mock/research/202606210600_resource-storage-model-canonical-addendum.md`
- `mock/design_rounds/202606210600_topic.resource-handle-storage-model.md`
- `mock/design_rounds/202606210600_topic.r5-underspecification.md`
- `mock/design_rounds/202606210600_topic.storage-model-pressure-test.md`

---

## 1. How loimu's storage model works

### The central concept: `define_record!` as the universal data shape

Before the 2026-02-17 design round, loimu had scattered storage: `SlabColumn<T>`
as the user-visible atom, `DenseNodeMap` for per-node marker data, and resources
that held their data directly. The unified storage model replaced all of this with
a single concept: `define_record!` makes any data shape a `StorageRecord`, and
one storage engine manages every `StorageRecord`. (Discussion doc, section 3, "The
record model -- the core insight.")

Resources are records with cardinality 1. Markers become boolean tags whose
associated data is a record. Collection/Dictionary items require `T: StorageRecord`.
Signals are explicitly NOT records: they are ephemeral, arena-allocated, managed by
a separate "snap backend." The tree owns node topology; the storage engine owns
per-node record data linked to nodes via markers. GPU bindings are NOT records
either; CPU records flow into GPU memory via explicit upload.

(Discussion doc section 3; changelist.md loimu-storage section 1.2; `DESIGN.md.tmpl`
line 323: "Resource -- State that behaviors read/write. Singleton. Record with
cardinality 1.")

### Where values live: the storage engine, not the resource

The critical transfer from the old model: resources hold HANDLES to storage, not
data directly. The storage engine owns all collection data. The changelist captures
this directly: "Resources with COLLECTIONS carry handles to storage, not the data
itself. The storage engine owns all collection data." (Changelist recovered
decisions C-5.) Resource scalar fields live in the record backend as a singleton
record (cardinality 1), but the storage engine still manages the allocation and
layout.

Specifically, the storage engine is the single entry point for all CPU-side data.
It has three backends:

1. **Record backend.** Persistent structured data. Views, SoA layout, double-buffered
   (frozen/live), temporal-field triple-buffered (old/current/projected).
2. **Snap backend.** Ephemeral one-pass data (signals, transients). Double-buffered
   arenas, bump allocation, pointer swap at pass boundary.
3. **Reel backend.** Pre-computed future values. Contiguous FIFO array with read
   cursor for tolerance > 2 temporal fields; also used for animation curves,
   procedural generation, AI decisions.

(Changelist loimu-storage DESIGN.md.tmpl, "Three storage backends" section.)

### Views: the access-pattern-driven colocation mechanism

Within the record backend, the storage engine builds "views." A view is a
contiguous memory region grouping record fields that are frequently accessed
together. Views are not user-facing; the engine computes them from behavior access
declarations using affinity analysis.

The system has two layers:

- **Static schema (build time):** column co-access graph computed from all behavior
  access declarations produces affinity groups and view templates.
- **Dynamic population (runtime):** view instances are created/destroyed based on
  population thresholds with hysteresis (promote at 1.5x threshold, demote at 0.5x).

Within a view, SoA layout: each field is its own dense column array, same node
order. Temporal fields get 3 columns (old/current/projected); non-temporal get 2
(frozen/live). Node-to-view mapping: `SparseArray<ElementId, (ViewId, DenseIndex)>`
per record type, O(1). Rare-combination elements below the population threshold fall
into a sparse fallback store.

(Discussion doc section 5; changelist, views section; recovered decision D-1 through
D-5.)

This is the key divergence from ECS archetypes: the user never declares views, the
engine computes them; views can be restructured on module load/unload; grouping is
driven by access patterns, not by user-declared component combinations.

### Columnar layout and access

Within a view, each field of a record is its own `SlabColumn`: mmap-backed,
fixed-stride, contiguous, zero-copy growth. Type erasure (`ErasedColumn`) allows
heterogeneous storage under the same engine. SoA columns are directly uploadable as
GPU buffers (no copy for GPU upload paths).

Behaviors iterate via `each (field_a, field_b, &mut field_c)` which pushes
per-element ops to the storage engine queue. The engine groups ops by shared read
columns and view membership, then executes one fused iteration pass per group with
all compatible ops per element. This is "operation fusion."

Two-level scheduling: the DAG scheduler handles correctness (behavior ordering);
the storage engine handles efficiency (data-affinity fusion and contiguous iteration).
The fusion invariant: disabling fusion produces identical results, just slower.

(Discussion doc sections 4-5; changelist loimu-storage fusion section.)

### Resources: handles to storage, singletons at cardinality 1

`define_resource!` delegates to `define_record!` and adds singleton semantics. The
storage engine holds the resource's record data. The resource itself is registered
into a `ResourcePool`. Behaviors access singletons via `read name: Type;` /
`write name: Type;` in the handler body; the macro derives access sets from these
declarations, not from explicit access blocks.

A key constraint: resources must NEVER hold per-node data (that is marker territory).
Resources with collection fields (`Collection<T>`, `Dictionary<K, V>`) carry handles
to storage, not the data inline. (`DESIGN.md.tmpl` recovered decision C-5; changelist
loimu-resource DESIGN.md.tmpl additions.)

---

## 2. Why loimu chose it: the reasoning and tradeoffs

### Starting pressure: temporal extrapolation forced the memory layout question

The 2026-02-17 round started with a narrow question: how to add temporal
extrapolation. The answer exposed the underlying problem. Should all columns be
triple-buffered (uniform stride, wasted space) or only temporal fields?

The first proposal -- separate columns per temporal field, batching by concrete type
across all resources -- was rejected. The user's correction: "Instead of batching by
concrete type across all resources, batch by 'stored shape.' This way we can do all
operations (including extrapolation) in a single pass." (Discussion doc section 1.)

This single correction cascaded: if you batch by stored shape (marker combination),
you need a mechanism that puts co-accessed columns in contiguous memory. That is the
view system.

### Why the record model, not per-resource layout

The granularity problem (section 2 of the discussion doc) was decisive: with
`#[temporal]` at the resource-field level, a `Collection<MyStruct>` where only some
inner fields are temporal required all-or-nothing temporal tracking. The user wanted
field-level granularity inside a collection element.

Option A (resource-field-level `#[temporal]`): too coarse.
Option B (`#[temporal]` on inner type's fields): would break "SlabColumn is the
universal atom."
Option C (both levels): two semantics.

The resolution: define_record! as the universal data shape, with `#[temporal]` at the
field level anywhere in the record, including inside collections. "If we had a
specified, designed unit that goes in there, we could mandate then specifying the
temporal attribute on its fields too." (Discussion doc section 2.)

This is the why: field-level temporal granularity required a canonical unit for stored
data shape. The record model is that unit.

### Why views, not ECS archetypes or SparseArray

The multi-column access problem: a behavior reading Velocity AND Physics per element
faces the issue that DenseNodeMap packs only populated elements, so Velocity (800
elements) and Physics (300 elements) have different dense indices. Lockstep iteration
without indirection requires contiguous co-location.

Options considered and rejected:

- **SparseArray at NodeId position** (one column entry per NodeId, always accessible):
  wastes memory for unpopulated slots.
- **Full archetypes (ECS-style)**: fragmentation on marker changes, transition cost,
  user has to manage archetype declarations.
- **Accept indirection** (DenseNodeMap + SparseSet lookup per secondary column):
  fusion benefit partially lost.

Chosen: storage-engine-driven co-location (archetype-lite). The engine detects
frequently-joined columns from access declarations and creates views. User never sees
views. (Discussion doc section 5, "Chosen: Option 4.")

### Why not a separate extrapolation pass

The fused extrapolation insight: when a behavior iterates a resource's columns, it
already touches frozen and live. The projected column is right there. Computing
`predict(frozen, live)` per element costs ALU, but that cost is negligible against
the memory access cost already paid. Extrapolation fuses into the normal iteration
path. Zero extra pass, zero extra cache pollution. (Discussion doc section 1, "Chosen
approach.")

### What loimu explicitly decided NOT to do

These rejections are as load-bearing as the choices:

- **Runtime layout adaptation.** "Databases spent 30 years on adaptive query
  optimization. We don't attempt this for v1." Static heuristics at startup, view
  restructuring on module load/unload. (Discussion doc section 9.)
- **Custom global allocator.** The storage engine is a facade, not a system-allocator
  replacement. Manages framework data, not all memory.
- **Signals as records.** Ephemeral; no persistent identity, handles, or lifetime.
  Snap backend stays separate.
- **Tree topology in storage.** Parent/child relationships are structural, not data.
  Tree owns topology; storage owns per-node record data.
- **Separate loimu-anim crate.** Animation is extrapolation. Animation curves are hints
  to `Temporal::predict`, not a separate computation engine.
- **Resources holding per-node data.** Hard constraint. Per-node data is marker
  territory, always.

---

## 3. Where loimu and hilavitkutin's proposed model agree and diverge

### The core agreement: handles, not inline values

Both land on the same fundamental shape. Loimu: resources carry handles to storage,
not data directly. Hilavitkutin (canonical addendum): `Resource<T>` is a handle,
not an inline-value store. Members decompose per-member into columns; the resource
record holds pointers/indices to its members' columns in a separate handle store.

This is not coincidence -- they share heritage. The insight is the same: the storage
engine owns values; the resource is an access path.

### The shared pointer-provenance insight

Loimu: "frozen data is immutable; multiple behaviors reading the same frozen column
have NO data dependency." The storage engine exploits this for fusion.

Hilavitkutin (spec 1695-1704): "Resource storage and column storage must have separate
pointer provenance ... resource data lives in a stack-local region that LLVM can prove
non-aliasing with column pointers." The stack-local hoist + separate provenance earns
the 1.28-1.40x noalias win.

These are the same insight applied at different layers: separate provenance enables
either fusion (loimu) or register-allocation across the morsel loop (hilavitkutin).

### Divergence 1: many-N-entity vs few-small-resource workloads

This is the most load-bearing divergence. Loimu is a UI/game framework. Its storage
model is designed around MANY nodes, each with a SMALL record (a handful of fields).
The view system, affinity analysis, and fusion all assume: lots of entities; behavior
access patterns determine which entity-batches need co-location; contiguous
per-entity-field columns unlock SIMD iteration and GPU upload.

Hilavitkutin is a pipeline execution engine. Resources are few and small (pipeline
configuration, metrics). Columns are independent typed stores for pipeline records.
The morsel is a window into a column, not a per-entity structure. The resource model
(handle + shape-bound columns) exists to keep resource reads out of the hot morsel
loop register pressure, not to enable per-resource SIMD batching.

Loimu's views and fusion are optimizations for the many-small-entity path. They do not
directly transfer to hilavitkutin's few-small-resource path. Hilavitkutin's resources
are more analogous to loimu's `define_resource!` singletons than to loimu's per-node
marker-bound records.

### Divergence 2: views vs shape-bound column sharing

Loimu's unit of co-location is the **view**: an engine-computed grouping of entity
records sharing a common marker combination, with co-located SoA columns inside.
Views exist because MANY nodes exist and co-accessing patterns benefit from
contiguous iteration.

Hilavitkutin's unit of sharing is the **shape-bound column**: columns are keyed by
stride (stored shape), not by concrete resource type. A `Field<u32>` appearing in
many resources collapses to one shape-bound column. This is column-reuse by type
identity, not co-location by access pattern.

Loimu does not have a "shape-bound column sharing" concept, because in loimu each
record type has its own fields, and the view system co-locates fields of frequently
co-accessed record types. Loimu would say: "a `Vec3` field in Velocity and a `Vec3`
field in Physics are different fields, not the same column." Hilavitkutin's
shape-bound sharing is more aggressive: it collapses same-stride members across
different resources.

Whether shape-bound sharing is correct depends on whether hilavitkutin's resources
are expected to have many instances (where sharing saves column count) or a fixed
small number (where the saving is marginal but the complexity is real).

### Divergence 3: temporal system vs double-buffering

Loimu has a full temporal system: `#[temporal]` fields, `Temporal::predict`, triple
buffers (old/current/projected), `CadenceMap`, rotation groups, reel backend for
pre-computed future values. This is a first-class runtime optimization for UI/game
state that changes at frame cadence.

Hilavitkutin's columns are double-buffered (frozen/live) for the pass boundary. The
resource handle model adds noalias wins. There is no temporal prediction or triple
buffering in hilavitkutin's design. The workload does not need it: a pipeline
execution engine does not extrapolate column values between passes -- it computes
them.

### Divergence 4: `std` allowed vs `no_std + no_alloc`

Loimu targets standard platforms with `std` and the system allocator. `SlabColumn`
is mmap-backed. The storage engine uses `Vec`-equivalent internals. There is no
`no_std` constraint.

Hilavitkutin is `#![no_std]`, no alloc. Every collection is either const-sized or
consumer-provided via `MemoryProvider`. This is the most fundamental structural
divergence: loimu's entire view system and dynamic population tracking assumes heap
allocation is available. Hilavitkutin cannot adopt those mechanisms directly without
a heap. Shape-bound columns must have const-known strides and compile-time or
plan-time allocation paths.

---

## 4. Blind spots: what loimu surfaces that hilavitkutin's model may miss

### Blind spot A: the "batch by stored shape, not by concrete type" principle is correct but under-applied

Loimu's key insight (discussion doc section 1): batch by stored shape (the
combination of field strides in a record), not by concrete type. This enables a
single pass over contiguous data to do all operations including temporal prediction.

Hilavitkutin's shape-bound column model is moving in the same direction: keying
columns by stride, not by concrete resource type. But the addendum and topic files do
not explicitly state the "batch by stored shape" principle as the guiding rationale.
They state the implementation (shape-bound = shared by stride) without stating why
shape-bound sharing is the natural result of "iterate by stored shape."

This is not a wrong decision, but the rationale is underspecified. If the pressure-
test is to confirm or deny the shape-bound model, loimu's "batch by stored shape"
reasoning is the missing justification. Hilavitkutin should inherit this explicitly:
the shape-bound column is what makes "iterate by stride" possible across resources,
exactly as loimu's view system makes "iterate by marker combination" possible across
entities.

### Blind spot B: the operation fusion model has no equivalent in hilavitkutin

Loimu has a two-level scheduling model: the DAG scheduler handles ordering (correctness);
the storage engine handles fusion (efficiency). Behaviors push per-element ops to the
engine queue; the engine groups ops by shared read columns and emits one fused
iteration per group.

Hilavitkutin has per-fiber morsel dispatch, with the WU `execute()` being the
per-element hot path. There is no "ops queue, then fuse" model. The per-WU
monomorphised function IS the fused unit.

This is appropriate for hilavitkutin's workload: compile-time monomorphization and
the const-gated flattener achieve what loimu does at runtime with the fusion engine.
But it means there is no storage-engine-level operation fusion available for
heterogeneous read patterns that cut across multiple WUs. If two WUs read the same
column in adjacent fibers, hilavitkutin cannot currently fuse those reads into one
pass. Loimu can.

This is not a blind spot that requires adding loimu's fusion engine to hilavitkutin.
But it IS a constraint to be aware of: hilavitkutin's fused dispatch is WU-granular
(compile-time), not storage-engine-granular (runtime). If the engine later finds that
cross-WU column reads could benefit from locality-driven grouping, loimu's fusion
model is the reference for how to build that.

### Blind spot C: resource collections should own handles to storage, not inline data

Loimu is explicit and emphatic: "Resources should NEVER have per-node data (that is
marker territory). Resources with COLLECTIONS carry handles to storage, not the data
itself. The storage engine owns all collection data." (Recovered decision C-5.)

Hilavitkutin's addendum and topic files state that `Resource<T>` is a handle, that
members decompose per-member into shape-bound columns. But the language "a `Seq`/`Map`
value is a pointer-to-first-element plus a length" raises a question loimu has
explicitly answered: when a resource has a collection (`Seq`/`Map` of N elements),
those elements are stored in the unified columnar store, not in a resource-private
arena. The resource carries only the handle (ptr+len).

The drift in the shipped hilavitkutin impl (`DrainStores` as a one-record opaque
column per `Resource<T>`) is exactly the mistake loimu's design explicitly forbids.
Loimu's "storage engine owns all collection data, resource carries handle" is a
direct corroboration of the canonical addendum and a direct indictment of the drift.

### Blind spot D: the view-restructuring / population threshold model has no hilavitkutin analog

Loimu's views are dynamically restructured: created for marker combinations above a
population threshold, destroyed when population drops, rebuilt on module load/unload.
This is a runtime adaptive mechanism driven by real access patterns.

Hilavitkutin has no analog. Its plan is computed at plan time, reused across frames.
Shape-bound column sharing is static (keyed by stride, not by runtime access pattern).
The no-alloc / no-std constraint makes a dynamic view system nearly impossible to
implement faithfully.

This is not a blind spot that requires adoption. But it is a workload difference to
be explicit about: loimu's view system is appropriate for a framework where module
load/unload changes which record combinations are active. Hilavitkutin's pipeline is
static: WorkUnits are registered at plan time, and the plan is reused. Dynamic
population tracking would be over-engineering for hilavitkutin's workload.

What hilavitkutin DOES need from this: the "access pattern analysis drives column
grouping" principle. Loimu does this at runtime (view affinity analysis). Hilavitkutin
does this at plan time (RCM ordering, waist-based grouping, fiber assignment). The
principle is the same; the timing differs. Hilavitkutin's plan-time grouping is the
correct analog of loimu's build-time affinity analysis.

### Blind spot E: signals and ephemeral data should NOT be records

Loimu explicitly documented the boundary: signals are NOT records, because they are
ephemeral communication, not persistent data. They get a separate snap backend
(double-buffered arenas, bump allocation, pointer swap). The rationale: signals have
no persistent identity, handle, or lifetime. Making them records would conflate
communication with storage.

Hilavitkutin has `Virtual<T>` for event-shaped markers. These are not records; they
are scheduling annotations. The signal boundary is implicit in the engine's model.
Loimu's explicit "signals are NOT records" framing is a useful crisp principle that
hilavitkutin should inherit, especially as the adapt subsystem (E8) introduces
`On<meta::ScheduleEnd>` hooks that blur the line between "meta lifecycle virtuals"
and "events."

The risk: if the adapt subsystem introduces ephemeral per-pass state that is
materialized as resource members (records), it violates the signals-not-records
boundary. Per-pass transient state should go through a snap-backend-equivalent
(in hilavitkutin's terms: a column with explicit pass-boundary clearing, not a
resource member that persists). Loimu's three-backend taxonomy is the cleaner
framing of this invariant.

### Blind spot F: the handle store's separate provenance must be explicit as an architectural invariant

Loimu achieves fusion wins by keeping frozen columns immutable and leveraging that
fact to batch reads across behaviors. Hilavitkutin achieves the noalias win by keeping
handle store and value columns in separate provenance regions so LLVM can prove
non-aliasing.

Both win from provenance separation. But hilavitkutin's design documents state the
win (1.28-1.40x, spec 1698) without stating the invariant that protects it: **the
handle store must NEVER alias the value columns, at any point, including during morsel
execution.** If an implementation decision later puts handle pointers inside the value
columns (e.g., embedding a resource ptr as a column element for some optimization),
the provenance separation breaks and the noalias win evaporates.

Loimu does not face this exact issue (it does not have the noalias objective), but
its "three backends with separate concerns" design maintains a clean separation that
incidentally achieves the same thing. Hilavitkutin should make the "handle store does
not alias value columns" invariant explicit and lintable -- not inferred from the
measured win.

---

## 5. Workload context: what this means for which lessons transfer

Loimu's storage model is designed for a UI/game framework with:
- Many (hundreds to thousands) of node entities, each with a small record.
- Frequent per-entity iteration (behaviors running at 60Hz over all nodes).
- Dynamic module load/unload changing which record combinations are active.
- Temporal prediction as a first-class runtime need (smooth frame presentation).
- `std` + heap allocation available.

Hilavitkutin is a pipeline execution engine with:
- Few (tens to hundreds) resources, each a small singleton.
- Large columns (potentially millions of records) accessed via morsel windows.
- Static pipeline composition at plan time; plan reuse across frames.
- No temporal prediction need (columns are computed, not predicted).
- `#![no_std]`, no alloc; consumer-provided memory via `MemoryProvider`.

The lessons that transfer directly:

1. Resources carry handles to storage, not inline data. (Corroborates hilavitkutin's handle model.)
2. Storage engine owns all value data; the resource is an access path. (Direct corroboration.)
3. Signals/ephemeral data should not be records; they need a separate pass-boundary-cleared path. (Applies to adapt/Virtual layer.)
4. "Batch by stored shape, not by concrete type" is the rationale for shape-bound column sharing.
5. Provenance separation between handle layer and value columns is an invariant, not an optimization choice.

The lessons that do NOT transfer (different workload):

1. The view system and dynamic population tracking. (No equivalent needed; plan-time grouping covers it.)
2. Temporal prediction and triple-buffering. (Not applicable; hilavitkutin computes, not predicts.)
3. Operation fusion via ops queue. (Covered differently by compile-time monomorphization.)
4. Dynamic view restructuring on module load/unload. (Static pipeline; no analog.)

---

## 6. Summary verdict

Hilavitkutin's handle + per-member + shape-bound + unified-store model is
**corroborated** by loimu's heritage on the core architectural principles:

- Resource as handle: yes, correct.
- Storage engine owns values: yes, correct.
- Values in a unified columnar store, not per-resource arenas: yes, correct.
- Separate provenance for handle vs value: yes, correct, and it is an invariant not an
  optimization.

The blind spots identified are:

- **A (underspecified rationale):** "batch by stored shape" is the missing WHY for
  shape-bound sharing. Should be stated explicitly.
- **B (no fusion):** operation fusion is covered differently (compile-time, not
  storage-engine-runtime). This is correct for the workload, but the constraint should
  be explicit.
- **C (collections must own handles):** loimu is emphatic that resources must never
  hold collection data inline. The shipped drift is exactly the mistake loimu forbids.
  This corroborates the drift-fix direction unconditionally.
- **D (view system):** does not transfer; plan-time grouping is the analog.
- **E (signals-not-records boundary):** the adapt/Virtual layer should inherit loimu's
  explicit boundary. Per-pass transients are not records.
- **F (provenance invariant not stated):** the noalias win depends on an
  invariant ("handle store does not alias value columns") that should be an explicit
  architectural rule, not just a measured outcome.

None of the blind spots are "the handle model is wrong." They are specification gaps
and invariant-naming omissions. The handle model is the right design. Loimu's
experience confirms it independently.
