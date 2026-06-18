# Engine roadmap r4: GATE-2 carrier mechanism (const-eval grouping + const-gated DCE)

**Date:** 2026-06-07
**Scope:** GATE-2 carrier-materialisation mechanism and its build sequence. Supersedes roadmap r3 (`202606070200_engine-roadmap-r3-gate2.md`) **only at stage G2-0c** (how the per-trunk programs come to exist). Everything else in r3 stands: the trunk-per-core model (the governing correction), the four walk levels (`RunPipeline -> RunPhase -> RunTrunk -> RunFiber`, all shipped from G2-0a/0b), sketches A/B/B2/C (they prove the runtime execution shape), and stage G2-N (core-pinning). r3's superseded G2-0c paragraph (`build()` constructs a nested `PhaseCons<TrunkCons<FiberCons<WuCons>>>` *type* from the plan) is replaced here.
**Oracle:** canonical consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, domain 17 the dispatch flattener (`:1540-1617`, the per-core program emission), R6 static composition (`:2435-2446`).
**Rationale + proof:** `202606071200_gate2-carrier-mechanism-fork.md` (the wall + the fork + op's answer), the two keystone sketches `sketches/202606071230_gate2-const-gated-trunk-isolation` and `sketches/202606071330_gate2-const-grouping-from-masks` (both PROVEN), and [[gate2-carrier-mechanism-mandate]]. Read the fork doc first.

## Why r3's G2-0c walled, and what replaces it

r3 said `build()` constructs a nested `PhaseCons<TrunkCons<FiberCons<WuCons>>>` carrier *type* from the plan structures, with the grouping carried in the type's shape. Grounding that against the source (scheduler build/run, the plan, sketch A, the 061400 D1b finding, a code-architect read) found it walls: deriving a heterogeneous nested type whose shape *is* the grouping, from a flat `WuVals` registration, requires a type-level N-way partition. Partition-by-key at the type-level boundary is inherently negative (sketch 202606061400 D1b: "the door is closed"), so it needs the forbidden full `specialization`; `min_specialization` cannot express it, and the grouping is in any case a global graph-connectivity computation (transitive write-column overlap), not a local pairwise type-fold. Sketch A proved the *walk machinery* devirts when the nest is hand-built; it never proved engine-derivation of the nest, and said so.

The canonical mechanism was always domain 17: a **codegen flattener emits the per-core program**. The grouping stays a computation; the dispatch carrier stays type-level and devirts; what the flattener emits is one monomorphised, member-only program per trunk. op confirmed the direction (Option 1, codegen flattener) and set the mechanism as the research job, priority order: stay in rustc/Rust first; proc-macros probably cannot express it; `extern "C"` + unsafe casts acceptable if in-Rust; custom LLVM pass only as last resort.

The mechanism is found, and it is the strongest rung of op's priority order: **pure Rust, no proc-macro, no build.rs, no LLVM pass.** It is const-eval grouping plus a const-gated flat-carrier walk that DCE collapses to member-only per-trunk programs. Both halves are sketch-proven against the real engine types.

## The proven mechanism

The full chain, WU access types to N isolated per-core programs, with no type-level partition anywhere:

1. **Access types to const masks.** Each WU's `Read`/`Write` `AccessSet` (a `Cons<Column<C>, ..>` cons-list) folds to a const `u64` bitmask over a global column numbering, via a recursive associated-const `ConstMask` trait (`const MASK: u64 = (1 << C::ID) | <T as ConstMask>::MASK`). No partition, no specialization: a plain associated-const fold. Collected over the carrier into `const READ_MASKS: [u64; N]` / `const WRITE_MASKS: [u64; N]`.

2. **Const-fn grouping.** A `const fn` runs the real plan logic over the mask arrays: read-after-write edges (`reads[j] & writes[i] != 0` gives edge i to j), longest-dependency-depth phase assignment by relaxation, then within-phase column-disjoint trunk components. Output: `const PHASE: [u64; N]` and `const GROUPING: [u64; N]`. This is ordinary const Rust over fixed-size arrays.

3. **Const-gated flat-carrier walk.** The dispatch walk threads carrier position `POS` as a const generic (`generic_const_exprs` for `{ POS + 1 }`) over the flat `WuCons` carrier, gating each position by `const { trunk_of(POS) == TRUNK }` (where `trunk_of` is a `const fn` indexing `GROUPING`, so the index lives in a const fn not a `const {}` block). A member position dispatches through the shipped `RunFiber` (as a single-WU `WuCons<H, WuNil>`); a non-member position's body is statically `false` and folds away.

4. **DCE to member-only monos.** Monomorphising the walk once per `TRUNK` value yields N functions; in each, every non-member position's dispatch is dead and the optimiser removes it. Each `run_one_trunk::<.., TRUNK>` mono is a true isolated per-trunk program: only its members' machine code, devirt-clean (zero `blr`), output-equivalent to the flat walk restricted to that trunk. Run one mono per core, zero sync (column-disjoint trunks do not alias), joined only by the waist barrier.

This realises op's "express the partitions without the typestate, enough for codegen to materialise them": the partition lives in const data (`GROUPING`), not in the carrier type; const-eval + DCE is the codegen flattener.

### Sketch proof

- `sketches/202606071230_gate2-const-gated-trunk-isolation` (`32306f1`), step 3+4: a flat-carrier walk gated by `const { trunk_of(POS) == TRUNK }` (hardcoded grouping). objdump: trunk 0 {SX,SZ} mono has fx not fy, trunk 1 {SY} mono has fy not fx, each `blr=br=bl=0`. DCE confirmed member-only per-trunk programs. PROVEN.
- `sketches/202606071330_gate2-const-grouping-from-masks` (`4ba2506`), step 1+2 (then driving 3+4 with the real computed grouping): `ConstMask` fold gives `READ_MASKS=[1,4,2]`, `WRITE_MASKS=[2,8,16]`; the const fn gives `PHASE=[0,0,1]`, `GROUPING=[0,1,2]` (all asserted); the three trunk monos objdump member-only (fx / fy / fz isolated), zero `blr`. PROVEN.

No feasibility unknown remains on the mechanism. What is left is build-integration: wiring the proven shape onto the real `WuVals` carrier, the registered `Stores`, and the shipped phase/waist machinery.

## Stage G2-0 build sequence (single-core, output-equivalent)

G2-0a and G2-0b already landed (r3: `FiberCons`/`RunTrunk` in `dispatch::trunk_run`; `TrunkCons`/`PhaseCons`/`RunPhase`/`RunPipeline` + degenerate `waist_barrier` in `dispatch::phase_run`; both output-equivalent to the flat `Scheduler::run`, both objdump zero `blr`). The four walk levels exist. G2-0c is replaced by the mechanism integration below. Each step is its own mockspace round on the branch, fail-first tested, single-core and output-equivalent so it validates against the merged GATE-1 oracle and must not regress `#664`.

### G-a. `ConstMask` over a WU access set. PROVEN (sketch 202606071330 U1), UNBUILT.
Add the recursive associated-const `ConstMask` trait to `hilavitkutin-api` (over `Cons<Column<C>, ..>` / `Empty`), parameterised so a column's bit id is its position in the global column numbering. Unit test: a hand-checked WU's `Read`/`Write` mask matches.

### G-b. Collect masks over the real `WuVals` carrier into const arrays. ROUTINE (const Rust), UNBUILT.
A recursive associated-const array fold over the `WuVals` cons-list produces `READ_MASKS: [u64; N]` / `WRITE_MASKS: [u64; N]` keyed by carrier position (= registration slot = topo index, build-validated, the same numbering the `RunFiber` walk threads as `pos`). Validate the array fold compiles and the values match a hand-built carrier in a small slice test (or a tiny sketch if the associated-const-array fold has a const-eval wrinkle; the masks themselves are proven, only the N-wide array collection is new).

### G-c. Parameterise the column numbering by the registered `Stores`. MECHANICAL, UNBUILT.
The column bit id in G-a must be the store's position in the global `Stores` access-set list, resolved by `Locate<Target, Index>` + `WitnessIndex::INDEX` (the same numbering `PlanInputs.reads`/`writes` use, project.rs; NOT StoreId space, which skips ZSTs). This aligns the const masks with the access-mask space the plan already uses. The E7 round already threaded `Stores` as the Scheduler 6th generic and resolves T to its access-mask position via `Locate`; reuse that resolver.

### G-d. Const-fn grouping over the mask arrays. PROVEN (sketch 202606071330 U2), UNBUILT.
Port the sketch's `const fn compute_phase` / `compute_trunks` into the engine plan module as the canonical grouping. Output `const PHASE` / `const GROUPING`. **r4 design call (decide at build time, bench if it affects perf): the const-fn grouping may REPLACE the runtime plan's graph grouping** (`compute_waists` / `block_diagonalise` / fiber grouping), so one grouping is consumed by both dispatch and the plan structures, no duplicated logic. If the runtime plan still needs its structures for non-dispatch reasons (RCM row recovery, arena layout), keep both but derive them from the same const source. Default: unify; fall back to dual only if a concrete plan consumer needs the runtime form.

### G-e. Const-gated trunk walk + DCE, driving `RunFiber`. PROVEN (sketch 202606071230 + 071330 U3), UNBUILT.
Add the `POS`-threaded, `const { trunk_of(POS) == TRUNK }`-gated walk over the `WuVals` carrier, each member delegating to the shipped `RunFiber`. `run_one_trunk::<.., TRUNK>` is the per-trunk program. Re-point `Scheduler::run` to dispatch trunk 0..K via these monos at single core (sequential, output-equivalent to the flat walk). Gate: objdump each trunk mono zero `blr` + member-only; output bit-identical to the flat walk; `#664` no-regress.

After G-e, `Scheduler::run` dispatches per-trunk programs derived entirely from the registered types, single-core. This is r3's stage G2-0 complete (trunks established, waists sectioned) by the proven mechanism instead of the walled type-nest.

## Stage G2-N (core-pinning) and the prior E-steps

Unchanged from r3. The per-trunk monos from G-e are the per-core programs: assign each to a pool thread (G2-Na, re-point `synthesise_core_programs` off `RecordRange::Full`/fibers onto trunks), run concurrently zero-sync with the shipped `phase_barrier_arrive` waist between phases (G2-Nb, proven by sketch B / B2's 2.84x), bridge fan-in (G2-Nc, sketch C), single-trunk head+tail (G2-Nd / E4b). E2 pool is the worker substrate; E3 hardens the waist barrier for multi-episode; E4 meta-WU pipelining overlaps phases; E5 P/E pinning policy; E6 the N-vs-1 oracle partitioned by trunk; E7 (shipped on dev) / E8 orthogonal runtime passes.

## Build order

G-a -> G-b -> G-c -> G-d -> G-e [single-core per-trunk dispatch lands, oracle-validated, replaces r3 G2-0c] -> G2-Na -> G2-Nb [concurrent trunks land, sketch B keystone] -> G2-Nd -> G2-Nc -> E3/E4 pipelining -> E5 affinity -> E6 oracle at N -> GATE-2 benches. The `#664` branching/accumulator red arms turn green here via real trunk parallelism + waisting, never single-core stopgaps.

## Sketch coverage (chart-the-path step 10)

The mechanism is proven end to end against the real engine types; the execution shape was already proven in r3.

- Mechanism, half 1 (compile-time grouping from types): `sketches/202606071330_gate2-const-grouping-from-masks` (`4ba2506`). PROVEN.
- Mechanism, half 2 (const-gated DCE to member-only per-trunk programs): `sketches/202606071230_gate2-const-gated-trunk-isolation` (`32306f1`). PROVEN.
- Execution shape (the walk levels the mechanism feeds): sketches A/B/B2/C (r3 step 10, all PROVEN). The mechanism produces the trunks; `RunTrunk`/`RunPhase`/`RunPipeline`/`RunFiber` are the emitted bodies.

The only un-proven-as-a-whole build step is G-b's N-wide associated-const array fold (the masks are proven; the array collection over the real carrier is the new bit). If it walls (a const-eval limit on associated-const arrays over a long cons-list), that is a build-time finding to surface, not a feasibility unknown in the mechanism. De-risk it in the first build slice or a one-file sketch before the G-b round locks.

## See also

`202606071200_gate2-carrier-mechanism-fork.md` (the wall, the fork, op's answer), `202606070200_engine-roadmap-r3-gate2.md` (the trunk model + execution-shape sketches this builds on), `202606070100_gate2-trunk-sectioning-rechart.md` (the trunk-model synthesis), `canonical-design-outranks-intermediate-rounds.md` (why the spec's flattener outranks r3's drifted type-nest), [[gate2-carrier-mechanism-mandate]].
