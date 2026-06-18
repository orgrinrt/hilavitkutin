**Date:** 2026-06-07
**Scope:** GATE-2 N-core dispatch mechanism (post S3-wall web research)
**Source:** S3 sketch FAILS (`mock/research/sketches/202606070906_gate2-pairs-enumeration-dispatch`), op directive "do web searching, figure out more solutions"

## The problem, restated

GATE-2 runs column-disjoint trunks on separate cores, joined by waist barriers
between phases. op's mandate: single-core is the 1-core degenerate, no special
path; the per-core program should be the const-gated DCE flattener (devirt-clean,
no proc-macro). The shipped const grouping (R2) gives `phase_of(pos)` /
`trunk_of(pos)` as const fns; `run_one_trunk::<PHASE,TRUNK>` runs one fixed pair
devirt-clean (Sketch B, 2.84x). The OPEN piece is assembling a per-core program
that runs an ARBITRARY core's set of trunks.

The S3 sketch proved that ENUMERATING the distinct (phase,trunk) pairs into
per-pair const monos over the flat carrier WALLS in pure Rust: const-generic
`{I+1}` recursion overflows (E0275, abstract impl WF), GCE rejects field/array
access in const-generic argument position, free-fn const recursion infinitely
monomorphizes. A type-level pair-list terminates but cannot be built from const
DATA without reflection.

## What the web research established

- **Const-generic recursion over `0..N` has no working pure-Rust solution.** The
  canonical write-up (dev.to "Generic constant expressions") states the two
  blockers (stop the recursion; impose the recursive bound) and concludes the
  feature is incomplete with no working pattern. Matches the S3 wall.
- **`macro_rules!` (declarative, NOT proc-macro) can generate a fixed-count array
  of fn pointers over const-generic combinations**, indexed at runtime (the
  rust-lang users thread "macro_rules for generating const-generic parameters":
  `[Self::impl_::<$($a),+>(), ...]`). Two consequences: it works without
  proc-macros, but (a) it enumerates a FIXED combination space hand-listed in the
  macro call (it cannot read the grouping's const data), and (b) fn pointers are
  runtime-indirect calls (not devirt-clean at the call).
- **build.rs codegen is the established "emit specialized Rust from data" tool**
  (`phf_codegen` generates perfect-hash source in a build script). This is op's
  ORIGINAL "codegen flattener" Option-1 direction (hilavitkutin-build emitting the
  per-core program). It is devirt-clean (real generated const monos) but needs the
  WU registration reachable at build time (a manifest the build script reads), and
  cannot reference the consuming crate's types directly.
- **Rust has no Zig-`comptime`-style const-data-to-code.** `const fn` evaluates
  values, never emits monomorphized call sites. Turning const data into per-element
  type-level instantiations requires a (proc-)macro or a build script. Confirmed
  fundamental, same class as the #673 type-level-partition wall.
- **Runtime dispatch is industry-standard for parallel schedulers** (rayon and
  peers dispatch runtime work items / closures, not compile-time monos, and are
  state-of-the-art fast). The per-work-item branch is negligible against the work;
  the parallelism win comes from cores, not per-item DCE.

## The expanded option space

1. **Runtime per-core trunk-ownership mask.** Each worker walks the flat carrier
   gated by a runtime mask of its owned trunks (reuse shipped `run_gated`).
   Single-core = one worker owns all = the existing flat walk (the natural 1-core
   degenerate, no special path). Unit BODIES still devirt (`RunFiber`); only the
   per-unit ownership check is a runtime branch (cheap, predictable, correct since
   trunks are column-disjoint). No toolchain wall, no macro, no build step. The
   per-unit branch is the only non-devirt cost; bench says whether it matters.

2. **Macro-generated fixed-slice const-program table (derived this session).** A
   `macro_rules!` emits `[CoreFn; MAX_SLICES]` where slice `S`'s program walks the
   flat carrier gated by `const { trunk_of(pos) % MAX_SLICES == S }` (const ->
   DCE -> that slice's member-only program, devirt-clean inside). Runtime
   distributes the `MAX_SLICES` table entries across the actual worker count
   (worker `i` runs entries `i, i+ncores, ...`); single-core = worker 0 runs all
   entries = all trunks. The only indirection is one fn-pointer call per slice
   entry (MAX_SLICES times total, NOT per record), negligible. Enumerates a FIXED
   count (MAX_SLICES, e.g. 64), not the (phase,trunk) space, so no explosion and
   no data-read needed. Waist barriers: each slice program is phase-ordered (the
   carrier is monotonic) and barriers at const phase transitions across slices.
   Devirt-clean per slice, pure-Rust + macro_rules (no proc-macro / build.rs).

3. **build.rs / hilavitkutin-build codegen flattener (op's original Option 1).**
   A build script reads the WU registration (from a manifest) and emits the
   enumerated per-core const-mono programs as generated source. Fully devirt-clean.
   Cost: the WU set must be expressed where build.rs can read it (architecture
   change), and generated source is build-time-fixed (no runtime core scaling
   without regeneration).

4. **Fixed compile-time core count.** Each core a compile-time-fixed trunk set,
   gate fully const, devirt-clean, but the pool size becomes a compile constant
   (no adapting to runtime core count).

5. **Accept the reframe (bench-first).** Single-core is already correct via the
   flat walk regardless of choice. Ship Option 1 (runtime mask) now, bench N-core
   parity (branching/accumulator perf arms vs Sketch B's 2.84x), and escalate to
   2/3 ONLY if the bench shows the per-unit branch is a real cost. This is the
   arvo Kind-2 discipline (naive baseline -> bench -> escalate on evidence).

## Recommendation

Option 5 framing with Option 1 as the baseline: implement the runtime per-core
trunk mask (simplest, no wall, single-core falls out, unblocks the N-core bench),
then let the bench decide whether the devirt-clean Option 2 (macro slice table)
or Option 3 (build.rs) is worth its complexity. The parallelism win is from cores;
the per-unit ownership branch is almost certainly negligible, and the bench is the
oracle (op: perf forks get benched). Option 2 is the strongest devirt-clean
fallback if the bench demands it (pure-Rust + macro_rules, no proc-macro/build.rs,
fixed bounded mono count).
