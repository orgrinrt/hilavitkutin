# Passages only the recovery branch carries

The branch `recover/unlanded-rounds-and-benches` was cut from `dev` on 20
August 2026 and carries thirteen commits of 30 August 2026 that never landed.
Most of its documents are on `dev` already, byte for byte: the design rounds,
the research notes, the seed and the sketches reached `dev` by another way.
What `dev` does not have is the bench findings under `mock/benches/results/`
and the changes the branch made to a few design templates.

This file holds the template passages of that second kind that say something
about the design, copied by command from the branch at commit `b24f158` so
that a registry row can cite them at a path this branch has. Each passage
stands under a heading that names its file and its lines on the recovery
branch. The copy is exact. When the recovery branch lands, the rows that cite
this file are pointed at the templates and this file goes.

The bench findings are not copied here. They are listed in `LEDGER.md` and
their key findings are quoted in `ANSWERS.md` where an answer rests on them.

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 23 to 23

| thread | 20 | Pre-allocated pool, hybrid wake, core pinning; head+tail convergence designed, not yet built |

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 291 to 308

program. That per-phase walk is fiber-outer, matching `run`: for each fiber
descriptor in plan order the worker restricts the walk to that fiber's members
and windows `[0, record_count)` by that fiber's plan-baked
`FiberDispatch.morsel_size`, so a fiber's units run over one morsel before the
next and its co-located columns stay L1-hot for the window run on every core
rather than only on the single-core path (`RunCfg::MORSEL_SIZE` remains the
fallback for a degenerate zero window). Trunk-rank ownership is unaffected,
because the rank advance is independent of the member restriction, so each core
owns the same trunks in every per-fiber walk. Single-trunk phases are the
case trunk parallelism cannot help, since there is one trunk to distribute. Canon
answers them with head+tail convergence: two threads walking one commutative fiber
from opposite ends, meeting in the middle, skipped where resource accumulation is
non-commutative. That mechanism is not built yet, so a single-trunk phase currently
runs serially. An earlier engine split such phases N ways by record range, one
slice per core, and this paragraph described that split as head+tail convergence;
it was neither, and both the branch and the description are gone. Either way there
is no type-erased per-unit dispatch slot: the column-capable walk is the only
dispatch path.

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 310 to 320

Because both paths now dispatch under the union of the per-fiber member masks
rather than an all-ones mask, the descriptor set must cover every live unit: a
unit that no descriptor names does not run at all. Coverage is established by
the flatten in `derive_phase_dispatch_order`, which assigns each live unit to
exactly one fiber, so the union over descriptors is total. That invariant is
currently checked by debug assertions only, and the flatten truncates silently
once the descriptor pool is full, so a plan too large to describe degrades in a
release build into units that quietly never execute rather than into a build
failure. Making that truncation visible is tracked as follow-up; until it lands,
any change to the flatten bounds or to descriptor sizing is changing a
correctness invariant that both dispatch paths depend on.

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 552 to 561

regions of the single reserved buffer. The accumulator reserves cap =
build-time record count (the global at-most-one-append-per-record bound),
and core c appends into the sub-region at element offset `lo_c`, where the
regions tile `[0, total)` one per core (`per = ceil(total/ncores)`,
`lo = (c*per).min(total)`, `hi = (lo+per).min(total)`), with cap `hi_c - lo_c`
and its own live-length cell. This N-way regioning is what the design asks for
on the accumulator: each core owns an exclusive region and the main thread
merges them after the workers rejoin, preserving append order. It is a
different mechanism from head+tail convergence, which splits input records
between two threads; they share the ceiling formula and nothing else.

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 563 to 574

A `Replaceable` swap installs the whole value as one blob write through the
same `Selector` witness the drain and the projection read use, plus a
store-dirty mark; no other pointer derivation exists on the swap path. Under
the bench-decided storage layout a resource value is one reserved blob record
(scalar snapshot plus live-streamed collections), so the whole-blob install is
the specified mechanism, not an excluded shortcut. The earlier per-leaf-copy
rule was grounded on the refuted pre-bench layout, and the revert of the
one-line install that followed it was a mis-citation; the witnessed blob write
is canonical. Collection-bearing values gain per-record element writes for
their `Seq` and `Map` halves only when the collection wiring lands (tracked by
the catalogued swap test); until then the collection halves of a swapped value
are out of the swap's scope by design, not by omission.

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 576 to 582

A plan-affecting swap goes through `replace_resource<T: PlanAffecting>`, which
locates `Resource<T>` through the same witness shape, performs the identical
install and store-dirty mark, and sets the plan-dirty bit for `T`. `run` and
`run_parallel` enter the plan band when it is the first frame or any
plan-dirty bit is set, consuming and clearing the bits on entry; a plain
`Replaceable` swap never forces the band, which is the intended cost asymmetry
between a value swap's dirty cone and a plan recompute.

## `mock/crates/hilavitkutin/DESIGN.md.tmpl`, lines 584 to 592

The worker's region boundaries and the merge's read offsets are the same
function of `(total, ncores, core)`, computed once and shared rather than
spelled twice. That is the property that makes the merge correct: it reads
core c's region at exactly the offset core c wrote. Two independent copies of
the arithmetic would satisfy it only by coincidence, and a divergence between
them is silent, since the merge would read well-formed memory that simply
belongs to a different core. Offset `lo_c` (not a
separate `c*per` buffer) tiles the reserved buffer exactly because the slice
sizes sum to total. Disjoint byte ranges plus per-core cells mean no shared

## `mock/crates/hilavitkutin-api/DESIGN.md.tmpl`, lines 616 to 624

`Replaceable` and `PlanAffecting` are mutually exclusive, with
`PlanAffecting` dominant: the crate ships
`impl<T: PlanAffecting> !Replaceable for T` under
`#![feature(negative_impls)]` (WATCH tier in the workspace
unstable-features tables), so a consumer writing both impls for one
type gets E0751 at its own impl site. A plan-affecting value routes
through `replace_resource`, never `replace_value`. See the
exclusivity sketch at `mock/research/sketches/202607193310` (both
arms confirmed).

## `mock/crates/hilavitkutin-api/DESIGN.md.tmpl`, lines 953 to 963

`deallocate` takes the same `align` the matching `allocate` was given,
alongside the same `ptr` and `len`. All three are part of the safety
contract, not just the first two. An allocator underneath may require the
full layout to free correctly (`std::alloc` does), and nothing at the free
site can recover the alignment on its own: the caller is the only party
that knows it. A provider whose free path ignores alignment, such as a
bump arena, may simply not read the parameter.

The hazard is concrete rather than theoretical. Column slabs are reserved
at 64-byte alignment, so a provider left to guess a word would free every
column with a mismatched layout.

## `mock/crates/hilavitkutin-api/DESIGN.md.tmpl`, lines 1105 to 1117

### `PlanAffecting` open marker

Open marker trait on resource value types, per-type consumer opt-in
mirroring `Replaceable`. Resources implementing `PlanAffecting` dirty
the plan cache when replaced via `Scheduler::replace_resource`;
resources without it are cheap value-swaps. Default is NOT
plan-affecting. `PlanAffecting` and `Replaceable` are mutually
exclusive with `PlanAffecting` dominant (see the `Replaceable`
section's negative impl); a plan-affecting swap performs the identical
witnessed blob install plus the plan-dirty bit.

```rust
pub trait PlanAffecting {}

## `mock/research/202607181300_engine-roadmap-r6.md`, lines 211 to 220

**G2C-M also carries the per-fiber walk-overhead arm (added 2026-07-19).** Serving per-fiber L1
windows on the flat carrier costs re-walks, and the sketch
`202607191200_a4-fibercons-nest-wireability` established that the cost is inherent to the mandated
mechanism rather than a shape choice: with F fibers of m morsels each over U trunk units, dispatch
does F x m x U runtime `dirty.bit` tests where the old shared-window shape did m x U, for identical
executed work, because `Member::IS` is const per trunk rather than per fiber. Measure per-fiber
windowed dispatch against the shared-window shape at fiber counts 1, 2, 4 and 8, on both fiber
kinds. This applies to `run` as much as to `run_core_phase`, since A2b shipped the same shape. If
the locality win does not cover the walk cost at realistic fiber counts, that is a finding about the
mechanism to surface to op rather than absorb. If bandwidth saturates by 4 cores on the heavy fiber, that is the ceiling

