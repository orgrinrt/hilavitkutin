# Findings: facade WorkUnit AccessSet vs ContainsAll and the const grouping

**Sketch:** `202606090100_facade-accessset-containsall` (roadmap r2 section 7-4a)
**Toolchain:** nightly-2026-05-28, release profile, fat LTO, codegen-units=1
**Engine state:** post-E4 (MetaBlock / WVirt / MP EngineCtx shape), branch `feat/hilavitkutin-parallel-engine-gate2`

## HYPOTHESIS

A facade WorkUnit (standing in for a runtime-loaded cdylib extension whose real
data access is unknown at host compile time) can register on the real builder,
pass build()'s `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>`
bound, and group correctly under the const grouping (the BundleMasks fold behind
`phase_of` / `trunk_of`), producing a correct conservative plan. Refinement: the
feared wall ("if the facade must carry an over-approximating concrete AccessSet,
ContainsAll may reject it") never materialises, because the SOUND facade declares
only its BRIDGE stores (the host I/O it marshals across the ABI), and the
plugin's "unknown access" is non-host data, correctly absent from every host
AccessSet. No synthetic or over-approximating AccessSet is needed.

## OUTCOME

WORKS

## Evidence

Three probes, all against the real engine crates (`mock/crates/` path deps), all
green, binary exit 0.

1. **build() + the real `Scheduler::run()`.** A bundle with TWO facade shapes
   (bridge facade `Read=Column<In>` / `Write=Column<Out>` calling an opaque
   black_box'd fn pointer per record, and a maximally opaque facade
   `Read=Empty` / `Write=Empty`) plus a real accumulator Consumer
   (`Read=Column<Out>` / `Write=Accum<Sum>`) compiles and builds. The
   ContainsAll bound is a where-clause, so compiling IS the proof it passed
   with facades present. `scheduler.run()` (the shipped dispatch: const
   grouping, plan ordering, per-trunk monos, meta pipeline) produced correct
   data over 256 records: `Out[i] = plugin_transform(In[i])`, Sum accumulated
   from Out, 256 transform calls and 256 side-effect calls on plugin-private
   memory (static atomics, never a host store).

2. **What the const grouping computes.** Introspected via
   `group_n` / `phase_of` / `trunk_of` over the same real WU types (the blanket
   `UnitAccess` covers any WorkUnit) with stores `[In@0, Out@1, Accum<Sum>@2]`
   and units `[FacadeBridge, Consumer, FacadeOpaque]`:

   | unit | phase | trunk |
   |---|---|---|
   | FacadeBridge | 0 | 0 |
   | Consumer | 0 | 0 |
   | FacadeOpaque | 0 | 2 |

   The bridge facade and its consumer join one trunk through the RAW conflict
   on `Column<Out>` (correct: the consumer is sequenced after the facade inside
   the trunk). The opaque Empty/Empty facade floats in its own trunk with no
   synthetic edges: it neither serialises the pipeline nor gains
   depends-on-everything edges. Exactly the conservative plan wanted.

3. **Anti-topo counter-probe.** Registering Consumer BEFORE FacadeBridge fails
   `build()` with `BuildError::NonTopologicalRegistration`. The facade's bridge
   AccessSet genuinely participates in the dependency analysis; its edges are
   seen, not dropped.

## The working shape

The sound plugin facade is an ordinary bridge WU:

- `type Read` / `type Write` name exactly the host stores the facade marshals
  across the ABI (the input column it reads, the output column it writes back).
  Nothing else.
- The opaque plugin call (in production: a fn-ptr resolved through the
  hilavitkutin-extensions ProviderId/CapabilityId ABI, held in a
  `Resource<CapabilityVtable>` singleton the facade also Reads) lives inside
  `execute()`. The plugin's own memory and the values handed across the seam
  are not host stores and appear in no AccessSet.
- A no-host-access facade (`Read=Empty`, `Write=Empty`) also builds, groups,
  and dispatches; it floats as its own trunk.

ContainsAll is a registration check, not a coverage check: it requires only
that named stores be registered (`ContainsAll<Empty>` is blanket-true; the
recursive arm needs each member `Contains`-present). It has no notion of "the
plugin's full access" to approximate against, so the over-approximation wall
cannot arise. A plugin reaching host data it was not handed is the
global-reach-in anti-pattern (hilavitkutin-workunit-mental-model), not a facade
requirement.

## What this does not prove

The per-record opaque call here is faithful for the build/grouping/plan
question but not for the hot-path cost question. The per-morsel amortisation of
the ABI hop and the zero-blr host walk are sibling sketch
`202606090200_facade-per-morsel-abi-hop` (section 7-4b).

## History

First authored 2026-06-09 against the pre-E4-slice-3 engine, driving a local
copy of the fiber walk; that run's conclusions matched this one. Re-validated
2026-06-11 after the engine API gained the MetaBlock / WVirt / MP EngineCtx
parameters: the local walk was dropped and the sketch now drives the REAL
`Scheduler::run()` plus the real const grouping fns, which is strictly stronger
(the original never exercised the grouping at all).

## Unblocks

The plugin-facade integration pattern for the PLUGIN phase (post-GATE-1):
section 7-4a was the one genuine remaining feasibility unknown there. No
Step-11 op-decision triggered; no wall.
