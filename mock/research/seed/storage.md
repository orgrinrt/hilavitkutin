# Storage: The Resource Model

This chapter consolidates the effective design for resource storage: the
founding spec's R5 resource model as corrected and completed by the
bench-decided storage addendum (`202606210600`, revised 2026-07-02 with op's
hybrid ruling). Precedence is applied once here; the addendum's sibling
expert memos are input material, not canon.

## The resource model

A resource value consists of exactly three field types. `Field<T>` where
`T: ColumnValue` (16 bytes or less) is a scalar, stack-local cached, with
only accessed fields loaded. `Seq<T, const N>` is a const-sized array and
`Map<K, V, const N>` a const-sized map, both with their elements in a
separate arena attached to the handle store. There are no dynamic
collections: allocation size is const generic, known at plan time, which is
what lets morsel sizing and arena reservation be computed once. Seq and Map
element values are not constrained to 16 bytes; only `Field<T>` carries the
ColumnValue limit.

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
(within plus or minus 1.7 percent across both runs), so it carries no
in-process penalty; the future-proofing upside decided it (op's hybrid call,
2026-07-02), with backcast complexity ruled out as an anti-axis.

## Morsel budget interaction

Write resource collections count toward the L1 morsel budget; read-only
resource collections do not, riding the L2 prefetcher. Collection sizes are
known statically from the const generic N (the `ResourceFootprint` derive
reports them). Large collections wrecking L1 is the consumer's design
problem. The full sizing formula lives in [[plan]].

## Replaceability

Resource replacement is an opt-in surface with two disjoint markers:
`Replaceable` gates `replace_value` (a between-frames whole-blob install of
a value swap), and `PlanAffecting` gates `replace_resource` (the same
install plus the plan-dirty bit that triggers the parameter-side recompute
at the next frame entry). Both are open marker traits consumers implement
on resource value types (op ruling A2-3, 2026-07-19: the property is
intrinsic to the type, not to one app's registration), and they are
mutually exclusive by construction: `impl<T: PlanAffecting> !Replaceable
for T` (`PlanAffecting` dominant), so a type cannot claim both channels.
The install semantics both channels share are the S1-S7 spec below,
implemented and awaiting ratification.

## Replacement semantics: specified, implemented, awaiting ratification

The founding spec left swap semantics open (a memo sentence proposing
member-by-member copying was expert input mis-cited as canon by two rounds;
recorded in A2-5). The specifying round has since run and shipped the
explicit spec S1-S7 under the bench-decided layout (blob record, erased
addressing, collection elements in collection columns): S1 witnessed
whole-blob `ptr::write` install for `replace_value`; S2 the same install
plus the plan-dirty bit for `replace_resource`; S3 collection element
writes gated behind the unified-storage work (#344/#654); S4 the negative
impl making the markers exclusive; S5 the cost-asymmetry caller contract
(an every-frame `replace_value` is a supported pattern; an every-frame
`PlanAffecting` swap is misuse, paying the plan band each frame); S6 the
test suite; S7 the swap benches
(install 2 ns / 85 ns / 1.81 us at 64 B / 1 KiB / 64 KiB; next-frame band
flat for clean and plan arms, value-cone scaling for the value arm). The
implemented-and-benched spec awaits op ratification as registry rows
(evidence record `202607201100`); until that ruling lands, the registry
row absence remains the authoritative canon signal.

## Open dependent item

The typed accessor shape for consumers reading resource collections
post-pipeline (Seq/Map backed by the handle-store arena) follows from the
storage layout work; it cannot be designed before the layout exists in
source. Registered open since the founding spec.
