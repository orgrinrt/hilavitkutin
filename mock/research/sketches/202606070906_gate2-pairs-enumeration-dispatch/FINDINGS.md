# FINDINGS: gate2-pairs-enumeration-dispatch (GATE-2 chart S3 / R3a-pre)

**Hypothesis.** The engine can enumerate the distinct (phase, trunk) pairs of a
grouping at compile time and dispatch one `run_one_trunk::<PHASE,TRUNK>` mono per
pair, over the FLAT carrier, in pure Rust on the pinned nightly (no proc-macro,
no forbidden type-level partition), output-equivalent to the flat walk. This is
the mechanism R3a needs because op's "single-core = 1-core degenerate, NO special
path" means single-core must use the same structured (phase,trunk) dispatcher as
N-core, and the `PhaseCons`/`TrunkCons` value-nest cannot be built from the flat
carrier (forbidden N-way type-level partition, #673).

**Outcome: FAILS.** The candidate mechanism walls on multiple independent rustc
limitations. Arbitrary data-derived enumeration of const-generic monos is not
expressible in pure Rust on nightly-2026-05-28.

## What walled

Two encodings of "for I in 0..NPAIRS, call `run_one::<{PAIRS[I].0},{PAIRS[I].1}>`"
were tried; each hit a distinct, fundamental rustc wall.

**Alt A: trait bool-dispatch recursion.** `RunStep<const I: usize, const CONT:
bool>` with a base impl (`CONT = false`) and a recursive impl (`CONT = true`)
whose where-clause requires `RunStep<{I+1}, {I+1 < NPAIRS}>`:

```
error[E0275]: overflow evaluating the requirement
  `(): RunStep<{ I + 1 }, { I + 1 < NPAIRS }>`
  ... 126 redundant requirements hidden ...
```

rustc checks the recursive impl's where-clause for the GENERIC parameter `I`, not
the concrete instantiation. For symbolic `I` it cannot evaluate `{I+1 < NPAIRS}`
to `false` to select the base impl, so it recurses `I -> I+1 -> I+2 -> ...`
without bound. Raising `recursion_limit` does not help: the recursion is
structurally unbounded at the generic level, not depth-bounded. This is the
`{N+1}` overflow the round-2a finding warned of, recurring.

**Alt B: free-fn const-generic recursion** (`fn run_step<const I>(...) where
[(); NPAIRS - I]:`), recursing via `if I + 1 < NPAIRS { run_step::<{I+1}>(...) }`:

```
error: overly complex generic constant
   { PAIRS[I].0 }  -- field access is not supported in generic constants
error: unconstrained generic constant  -- [(); NPAIRS - I]:
```

Two blockers. (1) Tuple field access (`.0`) is unsupported in a generic constant;
splitting into `PHASES[I]` / `TRUNKS[I]` array-index form is the suggested
workaround, but array indexing in const-generic position is itself "overly
complex" in this position and the suggested escape ("move into a const fn") does
not apply (a const fn call is also rejected in const-generic argument position).
(2) Even setting the GCE aside, the recursion does not terminate at
monomorphisation: `run_step::<{I+1}>` is referenced syntactically inside the `if`,
so it is monomorphised regardless of the runtime guard, giving
`run_step::<0> -> ::<1> -> ... -> ::<NPAIRS> -> ::<NPAIRS+1> -> ...` unbounded.
Rust has no const-`if` that elides instantiation of the dead branch here.

## Why this is fundamental, not a coding slip

Both encodings are the standard ways to express compile-time bounded iteration
with const generics, and both hit rustc-level restrictions, not authoring
mistakes: abstract impl-WF recursion (Alt A) and GCE-expressivity + monomorphisation-termination (Alt B). The only pure-Rust escape that terminates structurally is a TYPE-LEVEL cons-list of the pairs (walked with a `Nil` base, like the shipped Peano `Pos` walk in `run_one_trunk`). But building that pair-list TYPE requires turning the grouping's const DATA (the `phase_of`/`trunk_of` const fns) into a type-level list, which Rust cannot do without a proc-macro (no const-data-to-type reflection). That is the same class as the #673 type-level-partition wall: the const-gated mechanism works for a FIXED, known `(PHASE, TRUNK)` (Sketch B dispatched hand-picked monos, 2.84x), but NOT for engine-derived enumeration over an arbitrary grouping.

## Bearing on the design

The wall sits exactly on op's GATE-2 mechanism mandate (const-gated DCE
flattener, no proc-macro). It means a per-core program that runs a compile-time
ENUMERATED set of per-trunk monos is not achievable in pure Rust for an arbitrary
grouping. Compounding this: N-core trunk-to-core assignment depends on the runtime
worker count, so the per-core `(PHASE, TRUNK)` set is not compile-time-known
anyway. Both pressures point away from compile-time mono enumeration.

This is a roadmap-changing finding; it is escalated to op (the GATE-2 dispatch
mechanism is the human-owned course-correction per chart-the-path step 11).

## Viable alternatives (for the design decision)

1. **Runtime per-core trunk-ownership mask** (reuse the shipped `run_gated`
   shape). Each core walks the flat carrier gated by a RUNTIME mask of its owned
   trunks; single-core = one core owns all = the existing flat walk (the natural
   1-core degenerate, no special path). The unit BODIES still devirt (`RunFiber`);
   only the per-unit ownership check is a runtime branch (cheap, predictable,
   correct since trunks are column-disjoint). No toolchain wall. The parallelism
   win (trunks on cores) is preserved; bench-validate against Sketch B's 2.84x.
2. **Proc-macro / `app!`-generated pair list** (#295, parked). A macro emits the
   type-level (phase, trunk) cons-list from the registered bundle, walked
   devirt-clean. Achieves const-DCE per-trunk monos but reintroduces a proc-macro
   op previously disfavoured.
3. **Fixed compile-time core assignment** (each core a compile-time-fixed trunk
   set, no runtime worker count). Avoids enumeration but gives up runtime core
   scaling.

Single-core is unaffected by the choice: the existing flat `RunFiber` walk is
already the canonical 1-core degenerate (waist-phase is monotonic in registration
order), so it is output-correct regardless. The fork is purely about the N-core
per-core-program mechanism.
