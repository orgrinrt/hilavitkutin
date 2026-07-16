# Expert survey: alternative storage models for the resource model

**Date:** 2026-06-19
**Phase:** research deliverable for round 202606210600 (storage-model pressure-test)
**Scope:** survey alternative storage-model designs for `Resource<T>` and judge them against the
engine's goals and against the proposed handle + per-member + shape-bound + unified-store model.
**Oracle:** consolidation spec R5 (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` lines 535-566, 1682-1717); the canonical addendum (`mock/research/202606210600_resource-storage-model-canonical-addendum.md`); the three round topics (`mock/design_rounds/202606210600_topic.*.md`).

## The framing that the survey turns on

The proposed model in the addendum bundles two claims that are independent and must be scored
separately. Collapsing them into one "the handle model" hides the real decision.

**Claim 1: provenance separation plus stack-local hoist.** The resource value lives behind a
pointer whose provenance is distinct from the column-value pointers; the fiber dispatcher snapshots
the accessed resources into a stack-local region before the morsel loop. This is what earns the
1.28-1.40x noalias win (spec 1693-1706). It is the *only* part of the model that has been benched.
It is solidly R5-grounded (1689 "NOT inline", 1695-1704 "separate pointer provenance"). The current
source already implements it: `ResourcePtr<T>` and `ColumnPtr<T>` are distinct `#[repr(transparent)]`
`NonNull<T>` wrappers precisely so LLVM proves noalias (`mock/crates/hilavitkutin/src/resource/provenance.rs:1-10`).

**Claim 2: per-member shape-bound column decomposition.** A `Resource<T>` is a handle; `T`
decomposes per `Field`/`Seq`/`Map` member; each member becomes a column in the unified type-erased
store, keyed by **shape (stride)** so a `Field<u32>` appearing in many resources collapses to one
shared column; the resource record holds only pointers/indices to its members' columns. This is the
addendum's *inference*. It is unbenched. R5 does not state it: R5 says resource data lives in a
"stack-local region" (1703), not in unified columns, and the addendum itself concedes R5 is "silent
on this axis entirely" (`r5-underspecification.md` gap 1). Claim 2 is the actual open question.

The two claims are orthogonal. A model can take the provenance win (claim 1) and reject the
decomposition (claim 2) entirely. The cleanest way to read every alternative below is: *which of the
two claims does it keep, and at what cost?* The ranking is anchored on the benched claim 1; every
choice about claim 2 is "bench it, or it stays the open decision," matching the pressure-test
topic's own framing (`storage-model-pressure-test.md` point 2).

## The judging criteria, made concrete

The engine's hard constraints (from `hilavitkutin/.claude/CLAUDE.md`, the workspace rules, and the
spec): `#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, no `std::any`; monomorphisation is dispatch;
const-known sizes and strides preferred (the engine const-gates and DCEs over them); morsel
cache-locality; the provenance/stack-local noalias win; consumer ergonomics for viola/saalis/loimu/
polka-dots without leaking `*Ref`/`*Handle`/`*Key` types into consumer surfaces
(`hilavitkutin-workunit-mental-model.md`); plan mostly compile-time, amortised across frames;
soundness under the unified store; implementability under the pinned nightly and the engine's
existing const-gating machinery.

Two facts about the present source set the baseline. First, `ColumnValue` is blanket-implemented for
every `Copy + 'static` type with `BIT_WIDTH = size_of::<Self>() * 8`
(`mock/crates/hilavitkutin-api/src/column_value.rs:25-34`); the spec's "Field<T> where T <= 16
bytes" is a design note, not an enforced bound. Second, the shipped drift stores the *whole resource
value* as a single one-record `ColumnValue` blob behind one `ResourcePtr<T>`
(`mock/crates/hilavitkutin/src/resource/bindings.rs:22-30`, `309-366`). So claim 1 is already real in
source; claim 2 is not, and neither is its supposed prerequisite (the spec's per-member `Field`/`Seq`/
`Map` split is type-level marker vocabulary in `store.rs:189-241`, with no storage decomposition
behind it).

The criterion the task hands the survey that most pressures the proposed model: **resources are
"typically few and small" (config, interners, metrics, providers).** Claim 2's entire rationale is
"type-unique columns explode the column count." With few resources, type-unique explodes nothing.
This is the strongest dissent lever and the survey follows it honestly below.

## The alternatives

### (a) Proposed: handle + per-member + shape-bound + unified store

Keeps claim 1 and the strongest form of claim 2. Each `Field`/`Seq`/`Map` member is its own column
in the unified store, shared by stride across resources; the resource is a handle of member
pointers/indices in a separate handle store; `Seq`/`Map` are ptr-to-first plus length over
consecutive strides.

- **no_std / const / monomorphisation:** decomposition is expressible. `T` would need a derive
  (`ResourceFootprint` already exists, `#163`/`#164`) emitting a member-shape cons-list; the handle
  store is a cons-list of pointers, the same shape `bindings.rs` already builds. Member strides are
  const-known. No `dyn`, no `TypeId`. Implementable, but the most machinery of any option: a
  per-member decomposition trait, shape-keyed column allocation (two resources' `Field<u32>` must
  resolve to the *same* `StoreId`), and a handle-of-indices indirection at access time.
- **cache + noalias:** claim 1 is preserved for the handle pointers. But the decomposition *weakens*
  the noalias story for the member values, it does not strengthen it. If many resources' `Field<u32>`
  members live in one physical shared column, a column-WU that touches that column and a resource read
  of a member in the same column now plausibly alias from LLVM's view, which is the exact thing
  provenance separation exists to prevent. Worse, two different resources' members sharing one cache
  line is textbook false sharing once the parallel engine writes them from different cores. The
  addendum does not address either. The win it claims for claim 2 (uniform per-store stride dissolving
  the morsel-size fold) is real but is a build-time-ergonomics win, not a runtime-locality win.
- **column count:** minimised by sharing. This is the headline benefit, and it is only a benefit when
  resources are *many*. For "few and small," there is almost nothing to collapse.
- **consumer ergonomics:** the handle store stays scheduler-internal (the `bindings` cons-list is not
  consumer-visible), so it does not violate the `*Handle`/`*Key` rule. Consumers still write
  `Resource<T>` and reach members through the Context accessor. Good, *if* the indirection is hidden.
- **soundness:** the shared-column aliasing and false-sharing concerns above are candidate
  disqualifiers that need a sketch + bench before this can be called sound under the parallel engine.
- **verdict:** strongest on column-count, which is the criterion that matters least for the stated
  workload; carries the most machinery and the only new soundness questions. Not obviously the best
  fit for "few and small."

### (b) Type-unique columns per `T` (keep claim 1, reject shape sharing)

Each resource gets its own column(s); a `Field<u32>` in resource `A` and one in resource `B` are
distinct `StoreId`s. Provenance separation and stack-local hoist unchanged.

- **no_std / const / monomorphisation:** simplest decomposition. Each `T`'s members map to a fixed
  small set of columns at fixed `StoreId`s; no shape-keying, no cross-resource column resolution. The
  per-member size fold the drift struggled with becomes a straight per-`T` const sum.
- **cache + noalias:** *best* noalias story. Each resource's members have their own provenance and
  their own columns; no shared column means no spurious aliasing and no false sharing. Claim 1 holds
  cleanly and the member values inherit the same isolation.
- **column count:** higher than (a) in the abstract, but the "explosion" the addendum fears does not
  occur for few resources. A dozen resources averaging three members is 36 columns against a 256-slot
  default (`ArenaColumnStorage` `Dim<256>`, `storage.rs:37`). No explosion.
- **consumer ergonomics:** identical to (a) from the consumer's seat; the difference is internal.
- **soundness:** strongest. Nothing shared, nothing to alias.
- **verdict:** for "few and small," this is the per-member story without the shape-sharing machinery
  or its soundness questions. It is claim 1 plus the *weak* form of claim 2.

### (c) One-record opaque blob column per resource (the shipped drift)

The whole `T` is one `ColumnValue` in a one-record column behind one `ResourcePtr<T>`
(`bindings.rs:309-366`). Keeps claim 1, rejects claim 2 entirely (no decomposition).

- **no_std / const / monomorphisation:** trivially implementable; it is what ships. Provenance
  separation present.
- **cache + noalias:** claim 1 holds. But a resource read pulls the *whole* `T` even when a WU touches
  one field, and the stack-local hoist copies the whole `T`. For a small config struct this is fine;
  for a resource holding a `Seq<T, N>` inline it is not (the spec's `Field<T> <= 16 bytes` note exists
  precisely to keep the scalar hot path register-promotable, and a blob ignores it).
- **column count:** one per resource. Lowest possible.
- **consumer ergonomics:** fine; same `Resource<T>` surface.
- **soundness:** sound today. The real defect is that it cannot express `Seq`/`Map` members as
  ptr+len: a `Seq<T, N>` inside a blob resource is stored inline in the blob, contradicting R5's
  "separate arena" and the const-sized-collection design.
- **verdict:** correct for pure-scalar small resources, structurally wrong for any resource carrying a
  collection member. It is the floor, not the target. Its known failing (the heterogeneous bare-vs-
  Resource-vs-Column size fold) is real but is fixed equally well by (b) without the blob's collection
  problem.

### (d) Co-located resource struct in a dedicated stack-local region (the literal R5 reading)

Resources are *not* decomposed into the unified column store. Instead the resource set lives in a
small, dedicated, separately-provenanced arena; the fiber dispatcher snapshots accessed resources
into a stack-local array before the morsel loop (spec 1703-1717 verbatim: "resource data lives in a
stack-local region that LLVM can prove non-aliasing with column pointers"). Members stay inline in the
struct within that region; `Seq`/`Map` members are ptr+len into a sibling const-sized arena.

- **no_std / const / monomorphisation:** straightforward. The region is a const-sized arena; member
  offsets are const. No shape-keying, no per-member columns, no handle-of-indices.
- **cache + noalias:** this is the model the *benched* number was measured against. The stack-local
  snapshot is exactly what 1714-1717 describes. Claim 1 in its purest, most literal form. A resource
  read hits the stack-local copy; cross-resource members never share a cache line with column data.
- **column count:** zero resource columns in the unified store. Resources are not columns at all.
- **consumer ergonomics:** `Resource<T>` surface unchanged. Members reached through the Context
  accessor over the snapshot.
- **soundness:** the cleanest. Resources are an island with their own provenance, which is what 1695-
  1704 literally says. No shared-column aliasing, no false sharing.
- **verdict:** this is the literal reading of R5 and the model the only benchmark actually measured.
  The addendum reinterprets "separate arena" (566, 1686) as "only the handle store is separate, values
  are unified columns," but that reinterpretation is the inference under question, not R5's text. For
  "few and small" resources, the co-located region is the most faithful to both the spec and the
  benched win.

### (e) loimu's `StorageRecord` / views model as-is

`define_record!`-defined records, engine-computed archetype "views" co-locating frequently-joined
columns, generational handles, deferred-write triple buffers, SoA per view
(`~/Dev/loimu/.../2026-02-17_unified-storage-model-discussion.md`).

- **constraints:** disqualifies on nearly every hard constraint. It is alloc-ful (engine-managed
  allocations, view restructuring on module load/unload), runtime-adaptive (views recomputed from
  access analysis at runtime), generational-handle (the §3 acknowledged "array index + generation
  check" indirection), and explicitly *defers* static-vs-runtime layout to future work (§9 "runtime
  layout adaptation: not done"). hilavitkutin's plan is compile-time and amortised; loimu's views are
  a runtime query optimiser.
- **what is worth grafting:** the *affinity insight* (§5 option 4): co-locate members that WUs read
  together. In hilavitkutin terms that is a build-time decision the existing waist/trunk analysis
  already approximates (fibers that read the same resources run together). The triple-buffer temporal
  model is loimu-domain (UI extrapolation), not engine substrate.
- **verdict:** survey it as heritage, do not adopt the machinery. Its handle indirection is the very
  pattern `hilavitkutin-workunit-mental-model.md` forbids at the consumer level, and its runtime
  adaptivity contradicts the compile-time plan. Graft only the read-affinity framing, which the engine
  already has in another form.

### (f) Slotmap / generational-handle arena

A central arena indexed by `(index, generation)` handles; resources are slots; access validates the
generation.

- **constraints:** the generation check is a runtime branch on every access, against the engine's
  monomorphise-and-DCE model where a resource access should lower to a const-offset load. Generations
  exist to catch use-after-free of dynamically inserted/removed slots; the engine's resource set is
  static (registered at build, never removed mid-run), so the generation guards against a failure mode
  that cannot occur. It is pure overhead here.
- **column count / cache:** no decomposition; one slot per resource, similar to (c) but with an index
  indirection and a generation word per slot.
- **verdict:** rejected. Generational handles solve dynamic lifetime problems the static engine does
  not have, and the runtime check is exactly what the const-known-offset design avoids. This is the
  `*Handle`/`*Key` reinvention the workspace rule names.

### (g) ECS-style SoA archetype decomposition

Group resources by their member-set "archetype"; store each archetype's members SoA; access by
archetype + index.

- **constraints:** archetypes earn their keep when there are *many* entities sharing member-sets and
  membership changes at runtime (the transition cost is amortised over population). Resources are
  singletons, few, and static. There is no population to amortise over and no membership churn. The
  archetype machinery (fragmentation bounding, transition handling, view caps) is all dead weight.
- **verdict:** rejected for resources. (The engine already uses the SoA/columnar idea for `Column<T>`,
  which is the place entities-as-records actually live; resources are not that.)

## Scoring summary

The criteria that separate the credible options (a, b, c, d) are: faithfulness to the benched claim 1,
soundness under the parallel engine, machinery cost, and fit to "few and small." Loimu (e), slotmap
(f), and ECS (g) disqualify on the hard constraints and are heritage/negative results.

| Model | Claim 1 (benched provenance win) | Claim 2 (decomposition) | noalias / false-sharing | machinery | fit to "few and small" |
|---|---|---|---|---|---|
| (a) shape-bound shared columns | preserved for handles | strong form | *weakened*: shared-column alias + false-sharing risk | highest | poor (nothing to collapse) |
| (b) type-unique columns per T | preserved | weak form | best (nothing shared) | medium | good |
| (c) opaque blob (shipped drift) | preserved | none | fine but whole-T pulls; cannot express Seq/Map | lowest | scalar-only |
| (d) co-located stack-local region | purest / literal | none (resources not columns) | cleanest (island) | low | best |

## Ranking and recommendation

1. **(d) co-located stack-local region** for the common case (scalar `Field` resources), with
   **(b) type-unique columns** as the mechanism for `Seq`/`Map` collection members.
2. **(b) type-unique columns** as a uniform alternative if a single mechanism is preferred over the
   (d)+(b) split.
3. **(a) shape-bound shared columns** only if a bench proves the column-count pressure is real for an
   actual consumer workload *and* the shared-column aliasing/false-sharing concerns are shown benign.
4. (c) is the floor to migrate off of; (e)/(f)/(g) are out of scope on the constraints.

The recommendation **dissents from the proposed model (a) as the default.** The reasoning is the
"few and small" criterion the task supplies. Shape-bound sharing's only headline benefit is
suppressing a column-count explosion, and that explosion does not happen for a handful of small
resources against a 256-slot store. In exchange for a benefit that does not materialise, (a) adds the
most machinery (per-member shape-keying, cross-resource column resolution, handle-of-indices) and
introduces the only new soundness questions in the set: a shared physical column reintroduces the
exact LLVM-aliasing ambiguity that provenance separation (the one benched win) exists to remove, plus
cross-core false sharing once the parallel engine writes different resources' members in one line.
The addendum does not trace either, and the pressure-test topic is right to flag the model as
"largely unbenched" beyond claim 1.

The literal R5 reading (d) is both the most faithful to the spec text ("stack-local region", 1703;
"separate arena", 566) and the model the only benchmark measured. The addendum's reinterpretation of
"separate" as attaching to a handle store rather than to a resource value arena is an inference, and
it is the inference that drags the design toward (a). Read R5 as written, and resources are an island
with their own provenance, not unified columns. That island is (d).

The one real defect (c) exposed (the heterogeneous bare-vs-Resource-vs-Column morsel-size fold,
`canonical-addendum.md` lines 58-64) is fixed equally well by (b) or (d): under either, every resource
member has a const-known stride and the fold is uniform. The fold problem does not require shape
sharing to dissolve; it requires *decomposition into const-stride members*, which (b) and (d) both
provide without (a)'s sharing.

## What to graft from the alternatives if (d)+(b) is taken

- From **(a)**: the per-member decomposition machinery itself (the `ResourceFootprint`-derived member
  cons-list, `#163`/`#164`) is reusable verbatim; (b) is (a) minus the shape-keying step, so the derive
  and the const-stride sum carry over directly. Only the cross-resource column-sharing resolution is
  dropped.
- From **(d)**: the stack-local snapshot dispatcher step (spec 1714-1717) is the load-bearing piece
  and is independent of how members are stored; it should be implemented regardless of (b)-vs-(d) for
  the member values, because it is what realises claim 1.
- From **(e) loimu**: the read-affinity framing (co-locate what WUs read together) as a *build-time*
  hint to ordering, not a runtime view system. The engine's waist/trunk grouping already approximates
  this; nothing new to build, but it confirms the engine's existing analysis is the right home for the
  idea rather than a resource-storage concern.

## Open items a bench must close before locking any of (a)/(b)/(d)

1. The shared-column aliasing question for (a): does a WU reading a resource member from a shared
   physical column defeat the noalias hoist that (a) claims to preserve? Sketch + bench, or (a) is
   disqualified on its own headline win.
2. False sharing under the parallel engine for (a): two cores writing two resources' members in one
   cache line.
3. The (d)-vs-(b) member-storage choice for `Seq`/`Map`: const-sized arena (d) vs unified type-unique
   column (b). Both are sound; the choice is a locality/ergonomics bench, matching the round's
   "morsel-window-internal fetch vs stack-local cache" fork (`storage-model-pressure-test.md`).
4. Whether the column-count pressure (a) addresses ever materialises for a real consumer
   (viola/saalis). If no consumer has enough resources to pressure the 256-slot store, (a)'s rationale
   is moot and the decision is between (b) and (d) on locality alone.
