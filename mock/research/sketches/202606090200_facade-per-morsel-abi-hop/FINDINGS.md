# Findings: facade per-morsel ABI hop, host walk zero blr

**Sketch:** `202606090200_facade-per-morsel-abi-hop` (roadmap r2 section 7-4b)
**Toolchain:** nightly-2026-05-28, release profile, fat LTO, codegen-units=1
**Engine state:** post-E4 (MetaBlock / WVirt / MP EngineCtx shape), branch `feat/hilavitkutin-parallel-engine-gate2`

## HYPOTHESIS

A facade WorkUnit's `execute()` can hop across an extern-"C"-compatible ABI
boundary once per MORSEL (never per record) without violating
no_std / no_alloc / no_dyn, and the host's own per-record dispatch walk stays
zero-blr with the facade present in the same built pipeline. The minimal wire
shape is a morsel range handed to an opaque fn pointer.

## OUTCOME

WORKS

## Evidence

The sketch drives the engine's SHIPPED `RunFiber` walk (fiber_run.rs, MetaBlock
plus epoch threaded), not a local copy, through `#[inline(never)]` driver fns
that monomorphise one symbol per fiber. Binary exit 0.

Runtime: the host fiber (Producer writes `Column<Cv>` from a host-seeded
`Column<Inp>`; ConsumerAccum reads Cv and appends `Accum<Sum>`) ran 256 records
correct (`Cv[i] = Sum[i] = i*PM`). The facade fired the opaque plugin
capability exactly 8 times = ceil(256/32) morsels, NOT 256 times, and the
plugin filled its own buffer via its private cursor (work happened behind the
seam on plugin-owned memory).

objdump (arm64, measured 2026-06-11):

| symbol | instrs | blr | bl |
|---|---|---|---|
| dispatch_morsel_outer for the Producer host fiber | 82 | 0 | 0 |
| dispatch_unit_outer for the ConsumerAccum host fiber | 23 | 0 | 0 |
| dispatch_morsel_outer for the FacadePerMorsel fiber | 38 | 1 | 0 |

The facade symbol's single `blr x8` sits between the morsel-loop head and the
`b.lo` back edge, with NO inner per-record loop around it: one indirect call
per morsel, args x0 = relative start (0), x1 = morsel length. The host fiber
symbols contain zero indirect and zero direct calls even though a facade lives
in the same built pipeline: the facade's presence pulls no indirection into the
host's monomorphised dispatch.

## The working shape

The per-morsel-capability / sub-engine shape:

- The facade declares `Read=Empty` / `Write=Empty` (the plugin owns its data
  behind the ABI; nothing it touches is a host store, per the 7-4a finding).
- `execute()` uses the engine's own `BatchApi` (`ctx.batch().run(|start, len|
  ...)`, one closure call per morsel) and invokes the opaque capability ONCE
  with the morsel range.
- Wire shape across the seam: `fn(usize, usize)` carrying the morsel-relative
  range. Extern-"C"-compatible scalars; no host pointer crosses in this shape;
  no alloc, no dyn, no std anywhere near the seam. The plugin keeps its own
  absolute cursor and consumes "the next batch of len records".
- The cdylib seam is modelled by a black_box'd fn pointer, which is exactly the
  production shape: a capability fn-ptr resolved through the
  hilavitkutin-extensions ProviderId/CapabilityId ABI.

## Finding for the build phase (mechanical, not a wall)

`BatchApi` (and `EachApi`) hand the body a morsel-RELATIVE range; the engine
adds `morsel.start` internally inside `reader()` / `writer()`. A facade that
hands an EXTERNAL plugin a host column SLICE by ABSOLUTE index (the
host-data-bridge variant) therefore needs a morsel-absolute accessor the per-WU
Context does not yet expose: e.g. `ctx.morsel_range()` returning the absolute
range, or `ctx.read_slice::<T>()` / `write_slice::<T>()` returning the morsel's
slice. The shape proven here needs only the morsel length, so it works with
today's API; the bridge variant needs that small additive accessor. Note for
the plugin-facade build.

## History

First authored 2026-06-09 against the pre-E4-slice-3 engine with a local copy
of the fiber walk; the prior validation run was cut off before its artifacts
landed. Re-validated 2026-06-11: ported onto the engine's shipped `RunFiber`
(stale local walk deleted), lockfile refreshed to the current arvo dev revs,
runtime and objdump evidence re-measured (the table above is the measured
state, matching the original run's claims within one instruction on the
producer symbol).

## Unblocks

Together with 7-4a this clears the PLUGIN-phase feasibility pair: the facade
pattern builds, groups, plans, and amortises its only indirection to one
per-morsel ABI hop while the host walk stays fully devirtualised. The
host-column-bridge variant's morsel-absolute accessor is the one noted additive
API for the build phase.
