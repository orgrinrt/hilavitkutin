# Pre-Freeze Open-Question Dossier

**Date:** 2026-07-19
**Status:** evidence assembly for the seed pre-freeze resolution batch
(`seed/governance.md`, open items). Each section states the question, the
assembled evidence, and a recommendation for op's ruling. The swap spec
(item 1 of the governance list) has its own artifact
(`202607200200_replaceable-swap-semantics-spec.md`); the RCM order fork
(item 2) is bench-decided and its bench rides the sketch
`sketches/202607200400_rcm-order-locality-bench/`. This dossier covers the
rest.

## D1: the six GATE-2 agent-call deviations (governance item 5, A2-4)

Ledger reference: `pre_seed/202606072100`. The A2-4 standard: canon shape
built or sketched, benched where the ledger names a trigger, then a
bless-or-rebuild ruling. Assembled evidence and per-entry recommendation:

**Ledger 4, main-orchestrated waist barrier: RESOLVED BY SUPERSESSION.** The
worker-side sense-reversing `waist_barrier` shipped after the ledger was
written (r5 correction header): one publish/await per frame, workers hot
across phases, which is the canonical worker-side shape the ledger named as
the escalation. There is nothing left to bless or rebuild; the entry closes
as superseded-by-canonical-build. Recommendation: close.

**Ledger 5, park-immediately with no spin tier: REBUILD ALREADY SCHEDULED.**
The canonical spin-then-park tiers cannot be built before the telemetry
widening (the per-phase predicted-wait array), and both are charted as
roadmap bands (the PoolFrame phase-axis widening and the parking-tier
wiring). The parking primitive itself is canonical. The deviation is not a
candidate for blessing as an end state because canon's predictive parking is
an explicit design commitment; it is an acknowledged interim whose rebuild
is on the roadmap. Recommendation: record disposition rebuild-scheduled;
no bench needed (the bench would compare against a mechanism that is being
built regardless).

**Ledger 2, inline PoolFrame + Pin receiver, and ledger 10, inline GATE-2
scratch: ONE QUESTION, TWO SYMPTOMS.** Canon places the runtime data plane
in plan-stage arena memory reached by raw pointer; shipped code inlines it
in the Scheduler struct, which forces the `Pin` receiver on the parallel
entry (the workers need a stable address) and puts several KB of
always-present scratch on every scheduler including single-core consumers.
The named trigger is consumer ergonomics (viola, vehje, and the asymmetric
`&mut self` versus `Pin<&mut Self>` receivers). Evidence status: the arena
route requires a build-time raw allocation path the Scheduler currently
lacks (it retains no MemoryProvider after build); no sketch has proven or
refuted it. The cap-lifting arc (B1) touches the same scratch arrays.
Recommendation: bless the shipped shape as the recorded interim NOW (it is
sound, and no consumer has yet hit the trigger), with the arena relocation
kept as the named reconciliation, folded into the cap-lifting arc's scope
so the scratch question is answered once, not twice. Alternative if op
prefers structural resolution now: commission the arena-placement sketch
before freeze.

**Ledger 6, pointer-size spawn + exit-counter join: BLESS.** The spec's
spawn signature and "join when complete" did not prescribe a mechanism.
The shipped realisation (compile-time-guarded pointer-size closure, no heap
box; detached threads with an exit-counter barrier at Drop) is sound, keeps
the pool contract fire-and-forget, and its one real limit (consumer pools
cannot take fat closures) is a documented contract, not a defect. No bench
trigger was named. Recommendation: bless, with the pointer-size limit
stated in the platform contract's documentation.

**Ledger 7, discipline-sound raw scheduler aliasing: BLESS WITH
OBLIGATION.** Workers hold a type-erased pointer to the whole Scheduler;
soundness rests on the parked-between-frames invariant plus column
disjointness, not the borrow checker. This is inherent to a persistent pool
over a single generic Scheduler value (the alternative, N independent baked
programs, is the compile-time materialisation escalation already covered by
the ledger-1 blessing). The swap spec's S1 leans on the same invariant.
Recommendation: bless, with the invariant recorded as a hard correctness
obligation (any future mid-frame scheduler access breaks it) and the
standing aliasing-audit follow-up as its verification task.

**Ledger 1, runtime ownership mask (context): ALREADY OP-BLESSED** at the
time of the ledger, bench-gated, with the build-script codegen escalation
named. Not part of this batch; listed to keep the six-entry accounting
whole (the sixth agent-call entry is the barrier, resolved above).

## D2: the spectral role deviation (governance item 3)

Canon step 7 forms trunks by Fiedler bisection over a fiber-conflict graph
(gate: more than 5 fibers). Shipped code forms trunks from block-diagonal
connected components and uses spectral to form fibers within wide blocks
(gate: more than 5 units). The proposed oracle (the definitive
spectral-versus-greedy fiber bench) is gated on machinery that does not
exist yet (the real greedy former at runtime scale plus consumption of the
grouping by the runner), so the bench cannot run in this batch.

Classification: this is not a canon ambiguity. Canon's wording exists and
stands; the divergence is a mechanism-status fact about shipped code, held
under evidence-then-bless with a named oracle. The seed can freeze with
canon's step 7 as written and the deviation recorded (registry mechanism row
with status deviated, trigger the bench). Recommendation: confirm this
classification; no interim re-wording of canon.

## D3: head+tail mid_slot semantics (governance item 7)

The variant's documentation says record boundary; the name says arena slot;
nothing constructs it yet. The roadmap of record already assigns the call to
the convergence builder. Decision taken here, to be recorded: **record
boundary.** Head+tail convergence is defined on the record range (two
walkers from opposite ends meeting in the middle); the meeting point is a
record index, and no arena-slot semantics is needed by any consumer of the
mechanism. The implementing item renames the field to match
(`mid_record`-shaped naming) and the arena-slot reading dies. This is a
design call inside the convergence design, not an op gate; listed for
visibility, objection welcome.

## D4: LIGHT_THRESHOLD (governance item 8)

Canon names the constant and deliberately never values it; the strategy
selector compares producer weight against it. Classification: a bench-set
tunable constant. It enters the registry as a constant row flagged tunable
with the bench as its source-to-be; it is not a design question and does not
block the freeze beyond this classification being recorded.

## D5: resource collection accessor shape (governance item 4)

Answered by the swap spec's S3: collection access, read and write, is the
ptr+len view over the member's collection-column region behind the erased
descriptor; the concrete consumer-facing API lands with the collection
wiring (#344), shaped by that clause. The founding spec's open item closes
into that resolution.

## What this dossier does not cover

The swap spec ratification (own artifact) and the RCM order bench (its
sketch reports its own finding; bench-decided forks self-rule per the
standing standard, so the finding is presented, not asked).
