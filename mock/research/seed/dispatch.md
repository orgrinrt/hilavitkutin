# Dispatch: Devirtualised Codegen

Dispatch turns the plan's phase/trunk/fiber structure into executable code
with zero indirect calls. The no-alloc constraint is load-bearing here, and
ExpandedLto ([[foundations]]) is required. Monomorphisation is the dispatch:
there is no `dyn`, no TypeId, and no function-pointer table on any hot path.

## Devirtualisation rules

Bench-established, and they shape everything downstream. What
devirtualises: local `&[fn]` slices with known values, monomorphised trait
dispatch, unrolled function parameters. What does not: struct-field
function-pointer arrays (12.6x penalty; LLVM cannot prove contents through a
struct reference), const-generic `&[fn; N]` parameters (5.8x), global
mutable slots, and a shared `#[inline(never)]` dispatch helper (one function
for all callers defeats the proof).

The founding dispatch-approach menu, bench-validated relative to hand-fused
code: monomorphised tuple dispatch at parity; unrolled parameters slightly
better; one indirect pointer per fiber per morsel at 1.17x (rejected); trunk
mega-function at 1.02x; whole-schedule mega-function at 0.97x. Record count
selects the shape: under 10K records the smaller per-fiber or per-trunk
bodies win on icache and register allocation; above 10K the whole-schedule
shape wins as call setup amortises. Because the schedule is static (R6),
approach selection is a plan-time choice among pre-monomorphised variants; a
workload whose record count varies across frames switches approach by a
per-frame branch over a bounded variant menu (the Approach-2 pattern, proven
devirt-free: cost is code size linear in variant count plus one off-hot-path
branch). Unbounded runtime-computed grouping is impossible devirt-free and
is not a design goal.

## The carrier and the const-eval grouping mechanism

The registered WUs form one flat type-level carrier (a cons-list in
registration order, which build-time validation holds to producer-before-
consumer order per [[contracts]]). The grouping never lives in the carrier's
type shape: a type-level N-way partition of a heterogeneous carrier requires
the forbidden full `specialization` (proven repeatedly; A1 constraint note
1). The canonical mechanism (r4, sketch-proven end to end) is const
evaluation plus dead-code elimination over the flat carrier:

1. **Access types to const masks.** Each WU's Read/Write access set folds to
   a const bitmask over the global column numbering via a recursive
   associated-const fold (no partition, no specialization). Collected over
   the carrier into const mask arrays keyed by carrier position.
2. **Const-fn grouping.** A const fn runs the plan logic over the mask
   arrays: read-after-write edges, longest-dependency-depth phase assignment,
   within-phase column-disjoint trunk components, and the lifecycle rank
   renumber. Output: const phase and trunk assignments per position.
3. **Const-gated walk.** The dispatch walk threads carrier position as a
   const generic and gates each position on compile-time membership tests
   (trunk-root and member associated consts). A member position dispatches
   through the per-fiber walk; a non-member position's body is statically
   false.
4. **DCE to member-only programs.** Monomorphising the walk per trunk value
   yields one function per trunk in which every non-member position folds
   away: a true isolated per-trunk program, member-only machine code, zero
   indirect branches, run one per core.

The partition lives in const data, not in types; const-eval plus DCE is the
codegen flattener. The column numbering the masks use is the store's
position in the global access-set list, resolved by the same index-witness
machinery the plan projection uses.

Toolchain ceilings that shaped this (elevated to canon, A1 constraint note
3): `generic_const_exprs` rejects field access on generic constants (so
capacities cannot be config-struct-sized today) and limits inline const
blocks in bounds (worked around by associated-const carrier structs). The
runtime elements that legitimately remain runtime: the phase-pass index
match, the per-unit dirty bit, and core ownership.

A recorded, bench-gated deviation sits above this mechanism: the fully
compile-time per-core program emission (each core's entire pipeline as one
monomorphised function with baked record ranges) walls in pure Rust, and the
shipped realisation keeps core ownership as a runtime predicate over the
per-trunk monos. The op-blessed escalation, if the bench ever demands it, is
a build-script or proc-macro codegen step emitting the real per-core monos;
the codegen substrate is reserved for that target. See [[execution]] for the
deviation ledger context.

## The fiber flattener

For each fiber the codegen emits a monomorphised function and decides how
the fiber's WUs execute. The common fused case (the rust-pipe pattern):
snapshot resources to stack locals, read fiber-input columns at morsel
start, run the WU bodies as a pure function pipeline through locals
(fiber-internal columns never touch memory; dead-store elimination removes
their stores), group all output stores at the end of the loop body
(store-buffer friendly), accumulate resources through stack-cached pointers.
This measures at 0.95 to 0.96x of hand-fused code and beats hand-written
assembly, because LLVM schedules instructions better than we do. Deep
single-fiber pipelines get store-reload elimination between WU boundaries;
multi-fiber layouts need no flattening because fiber boundaries are natural
materialisation points.

Inlining discipline: `#[inline(never)]` on fiber dispatch (each fiber
optimises as its own unit), `#[inline(always)]` on WU functions within
their fiber (LLVM sees the whole body, fuses, vectorises),
`#[inline(never)]` on the per-core outer program.

## Resource handling in codegen

Resource storage and column storage must have separate pointer provenance:
two raw pointers originating from the same struct may alias in LLVM's view,
which forces resource reloads every iteration (the founding bench's 1.28 to
1.40x figure; the later storage bench did not reproduce it as a scalar
wall-clock effect on M1, and the mechanism is kept as an architectural
guarantee, see [[storage]]). The fix is the layout itself, not a compiler pass: resource
data snapshots to a stack-local region LLVM can prove non-aliasing with
column pointers, all resource accesses promote to registers, and mutable
resources write back once after the loop. The caching is mechanical, derived
from the declared resource access sets, emitted automatically by the
flattener. The full storage model, including the live-streaming rule for
collection members, is [[storage]].

Resource accumulation across records has cross-record dependencies through
the resource; under head+tail convergence each thread gets its own
stack-local accumulator, additive accumulations merge after, and
non-commutative accumulations fall back to sequential for that fiber.

## Sync primitives in generated code

Progress counters are one AtomicUsize per fiber, monotonic record index:
producer publishes with a plain release store (a single `stlr` on aarch64,
never a read-modify-write), consumer reads with an acquire load. Phase
boundaries spin with `isb` plus `ldapr`. The counters live in
scheduler-owned memory reached by raw pointer, never behind Arc. Lock-free
by construction is a library guarantee: static dispatch structure,
one-writer-N-readers per fiber, deterministic morsel assignment, no locks
and no CAS anywhere in the dispatch path; the only atomics are the progress
counters and barrier words.

## Intrinsics and microkernels

Idiomatic Rust first, profile, replace with asm only where measurement
confirms the win. What helps, bench-validated: LLVM auto-vectorisation
through no-dyn monomorphisation (the design is the vectorisation contract);
count-leading/trailing-zero scans for flag and mask iteration; bulk
copy/zero intrinsics for column ops; the `isb`+`ldapr` spin; LLVM's own
indexed addressing (better than hand-written pointer advances); NEON
auto-vectorised stores.

What hurts, bench-validated and therefore banned in dispatch loops: explicit
prefetch hints (about 2x slower on Apple Silicon, even morsel-level
pipelining variants); `likely`/`unlikely` (1.40x slower, blocks
vectorisation); `get_unchecked` is neutral (bounds checks already eliminate
for range iteration); hand-written aarch64 asm loses to LLVM.

The day-one microkernel set: cache-line zero (`dc zva` / `write_bytes`),
paired load-store (`ldp`/`stp`), LSE atomics, non-temporal stores, memory
fences (aarch64 only; x86 TSO needs none), worker parking (`wfe`/`sev`,
`pause` plus futex), trailing-zero scan. Explicit prefetch was removed from
this set and is reserved for genuinely random access patterns, never
sequential column scans.

The ASM verification gate for any fiber dispatch function: zero indirect
branches in the body, indexed addressing on column accesses, no stack
reloads in the per-record inner loop, morsel size as an immediate, no calls
to sizing or dispatch helpers from the loop. The founding bench passed all
checks on all 26 dispatch functions; the discipline stands as a standing
fixture-backed gate.
