# Storage and Resources

Sketch demonstration of a seed chapter: in adoption this text is the storage
section of `mock/research/seed/DESIGN.md`. It consolidates the effective
design for domains 07 and 19, precedence applied once at consolidation; the
original spec sections, the storage addendum, and the round topics become
unreferenced paper trail behind the git history. After the seed freezes,
every change to this design happens in the registry, never here, and where a
registry row conflicts with this text, the row is the later canon.

## The resource model

A resource value consists of exactly three field types. `Field<T>` where
`T: ColumnValue` (16 bytes or less) is a scalar, stack-local cached. `Seq<T,
const N>` is a const-sized array and `Map<K, V, const N>` a const-sized map,
both with their elements in a separate arena attached to the handle store.
There are no dynamic collections: allocation size is const generic, known at
plan time, which is what lets morsel sizing and arena reservation be computed
once. Seq and Map element values are not constrained to 16 bytes; only
`Field<T>` carries the ColumnValue limit.

## The value layout

A resource's value is one contiguous one-record blob, bumped from the arena,
never decomposed into per-member columns and never shape-bound shared. This
was bench-decided across two runs of the six-variant bench: the decomposed
layout loses up to 3.1x intra-resource locality at 64 members and crosses the
column-count cap on realistic resource sets; the shape-bound shared layout
carries a 3.4x cross-core false-sharing penalty plus a
resize-invalidates-all-sharers hazard; neither buys a hot-loop win. The
`Decompose` seam survives only for the size fold and the collection ptr+len;
the value bytes stay contiguous. An earlier reading that asserted per-member
decomposition was refuted by the bench and retracted.

## Scalar snapshot and live-streamed collections

Scalar `Field` members are snapshotted to a stack local before the morsel
loop; the snapshot copies the scalars and each collection member's ptr+len
view, never the elements. The mechanism is real in codegen (the no-snapshot
variant reloads every iteration, the snapshot variants hoist) and is kept as
an architectural guarantee: it is wall-clock-neutral on M1 where scalars are
L1-resident, and the spec's older 1.28 to 1.40x scalar figure is a March-2026
distillation the bench did not reproduce there. Collection members are
live-streamed: elements are read from their column inside the loop, because
streaming beats snapshot-copy by about 2.5x once a collection exceeds cache
at 64 MiB, with parity below about 4 MiB, so copying is never a win.

## The handle store and noalias

`Resource<T>` is a handle, never an inline value: value bytes live behind
pointer indirection in external slab storage, because writing inline data
through `&self` is undefined behaviour under LLVM's noalias on `&self`. The
handle store holds the pointers to the value blob and the collection columns,
keyed by resource id, with pointer provenance distinct from the value
columns; that separation is what lets snapshotted scalars stay in registers
across the morsel loop. The separate arena the resource model names for Seq
and Map attaches to the handle store, not to a resource-private value arena.

## Erased static-shape addressing

Value bytes are addressed through an erased static-shape descriptor with
backcast on access, globally: every resource uses it, so any resource can
cross a cdylib or wasm plugin boundary and interoperate with builtin
resources near-natively without a per-resource design change. The bench
measured erased addressing at parity with native monomorphised on every axis
(within plus-minus 1.7 percent across both runs), so it carries no in-process
penalty; the future-proofing upside decided it, with backcast complexity
ruled out as an anti-axis.

## ColumnStorage contract

ColumnStorage hands out raw pointers, never slices (slices assert noalias
that fused WorkUnit access violates), with type-native stride and 64-byte
alignment on column arrays. The consumer provides the backing memory; the
library never calls an allocator, with the `MemoryProvider` platform trait as
the valve for any allocation strategy. Every column, including each
resource's one-record blob and each collection member's element column, is
reserved once at plan time and never reallocated while the scheduler lives; a
reallocation would invalidate every recorded pointer with no mid-life
re-resolution mechanism. Column lifetime uses the release-advisory
consumer-count model: the schedule pre-computes readers, decrements on fiber
completion, and release fires at zero.

## Morsel budget

Write resource collections count toward the L1 morsel budget; read-only
resource collections do not, riding the L2 prefetcher. The window formula is
L1_usable divided by the sum of write column sizes plus write resource
collection sizes, with collection sizes known statically from const generic
N. Large collections wrecking L1 is the consumer's design problem.

## Replacement remains unspecified

Swap semantics for `Replaceable` resources are deliberately not specified
here: the storage round left them explicitly open, a memo sentence proposing
member-by-member copying was expert input mis-cited as canon by two rounds,
and the specifying design round is commissioned. Until its rulings land as
registry rows, `replace_value` marks dirty without installing, and the
absence of a swap-semantics ruling row is the authoritative signal that canon
has no answer.
