# Governance: What This Seed Is

This directory is the flat, self-contained expression of hilavitkutin's
effective engine canon as of 2026-07-19. It consolidates the founding
consolidation spec (design round `202604200055`, topic `202603181200`, the
22-domain single source of truth) with the full registered amendment chain
applied once, in precedence order, with superseded material left out. A reader
holding only these chapters holds the design; nothing outside them needs to be
cross-read to know what the engine is supposed to be.

## Sources and precedence

The precedence regime this seed applies is addendum A2's traced ruling
(`pre_seed/202607193200`, op-delegated trace, 2026-07-19):

1. The consolidation spec (`202603181200`), the founding canon.
2. Op-ruled and bench-decided amendments, each modifying canon where it
   explicitly speaks, in date order: the unified-engine
   amendment (`202606061000`), the GATE-2 rechart (`202606070100`; GATE-2
   names the arc's second build gate, the parallel-engine completion
   milestone, and its sub-rounds carry G2-prefixed codenames such as the
   G2-Nd convergence build round) with r3
   (`202606070200`), r4 (`202606070700`), the parallel-bench fairness
   correction (`202606081100`), r2 (`202606081500`), r5 (`202606081600`), the round-level
   amendments registered in A1 item 6, A1 itself (`202606111400`), the
   resource-storage addendum (`202606210600`), and A2 (`202607193200`). A
   later ruling wins over an earlier one on the same point; where an amendment
   is silent, the consolidation spec's wording is the tiebreak.
3. The commissioned standalone spec (`202606111800`) is a reading aid with no
   independent design authority.
4. Everything else (roadmaps, audits, expert memos, sketches, generated
   design docs, agent memory) is intermediate material, never canon.

All of those documents now live under `mock/research/pre_seed/` (or in the
archived design rounds) as paper trail. They are citable by
`git::commit::<hash>` reference for provenance, but they are no longer the
reference for what the design is. This seed is.

## Lifecycle

While `frozen = false` in `MANIFEST.md`, this seed is under construction and
chapters may still be corrected against their sources. The freeze is gated,
in order: first the collection pass completes (every chapter drained from its
sources with the manifest proving losslessness); then every open question
listed below is resolved in one batch of in-flight design work (bench-decided
forks are benched, op-gated calls are designed and ruled), and the chapters
are updated with the resolutions; then the independent verification passes
audit the seed against the full research backdrop; then the manifest declares
the seed frozen. A seed with unresolved open questions does not freeze.

Verification pass 1 ran 2026-07-19 (four independent audits; record
`202607210010_seed-verification-pass-1.md` with the deep-audit report
alongside): verdict FIT AFTER LISTED FIXES, and the fix batch is applied
to the chapters as committed. An op-ordered redo of the pass ran 2026-07-20
against the original sources (one deep audit verifying every manifest drain
claim, plus three parallel sweeps: founding-spec coverage, amendment-chain
coverage, currency and self-containment; record
`202607210300_seed-verification-redo.md`): verdict FIT AFTER LISTED FIXES,
no blockers, chain sweep clean, and the fix batch (stale wake and parking
prose, spectral currency, manifest staleness and tier labels, S5, term
definitions, consumer scale envelope) is applied as committed. The second
pass runs on the candidate registry after drain.

Once frozen, the seed text never changes again. From that point every
amendment, ruling, clarification, and status change to canon lands as rows
in the registry (`mock/registry/`, created at drain time), and a registry row outranks seed text
wherever the two conflict. The absence of a registry row on a point means
the seed chapter is the current word.

## Standards the seed carries forward

Two standing decision standards are part of canon, not process trivia:

**Evidence then bless (A2-4).** Shipped mechanisms that deviate from a
canonical mechanism are neither blessed by default nor rebuilt by default.
For each recorded deviation, the canonical shape is built or sketched,
benched against the shipped shape where a bench trigger is named, and the
evidence is presented for a bless-or-rebuild ruling. This governs the six
GATE-2 agent-call deviations and the spectral role deviation (see
[[execution]] and [[plan]]).

**Bench-decided forks.** Where the fork is which implementation, algorithm,
or data shape performs better, the resolver is to build the candidates and
bench them, then rule from the findings. Canon registers such forks open with
a named bench oracle rather than picking by argument. The open bench forks
are listed in the chapters that own them.

## Open questions blocking the freeze

Canon currently knows what it does not say. Each item below is open with a
named resolution channel, and each must be resolved (the design work done,
the bench run, the ruling made) in the pre-freeze resolution batch. The
chapters state each item in place today; the resolution batch replaces those
statements with the settled design:

1. **`Replaceable` swap semantics**: SPEC IMPLEMENTED AND BENCHED
   (2026-07-19, round `202607200500`; evidence record
   `202607201100_swap-semantics-ratification-evidence.md`). The S1
   witnessed blob install, S2 plan-dirty band trigger, and S4
   exclusivity ship with the S6 suite green; the S7 benches confirm the
   cost asymmetry (install is the linear memcpy; a value swap pays its
   cone, a plan swap pays the band, neither leaks). Awaits the op
   ratification of S1 through S7; S3 and Swap-D (the commissioned
   cold-write-then-stream collection-swap bench, deciding whether the
   morsel budget's write-collection term absorbs swap traffic) stay
   #344-gated inside the spec. See [[storage]].
2. **RCM row order as execution order**: RESOLVED by bench (2026-07-19,
   oracle A1-1; evidence record
   `202607200800_a1-1-ordering-theory-bench-expansion.md`, which
   supersedes the first record's chain-topology claim). RCM wins against
   rival ordering theories at the DRAM-resident scale the engine targets
   (naive registration order loses 1.65x there) and order is near-neutral
   below it; Step 5's wording stands. See [[plan]].
3. **The spectral role deviation** (canon forms trunks spectrally; shipped
   code forms fibers spectrally within wide blocks): EVIDENCE DELIVERED
   (2026-07-19, record `202607201200_spectral-role-bench-evidence.md`,
   bench `fiber_theory`). Both cost and grouping character favour
   canon's role split (greedy is linear and fiber-grained; spectral is
   two to three orders heavier and trunk-grained); the proposed ruling
   restores Step 7/8 roles via a plan-chain corrective round. Awaits the
   op bless per evidence-then-bless. See [[plan]].
4. **Resource collection accessor shape** (consumer reads of `Seq`/`Map`
   post-pipeline) waits on the storage layout work it depends on. See
   [[storage]].
5. **The six GATE-2 agent-call deviations** each await an
   evidence-then-bless ruling. See [[execution]]. Deviations 1 and 6
   (inline `PoolFrame` with `Pin`; inline GATE-2 scratch): EVIDENCE
   DELIVERED (2026-07-19, record
   `202607201400_gate2-deviation-1-6-evidence.md`, sketch
   `202607201300_arena-poolframe` WORKS): the whole-plane arena route is
   mechanically proven, dissolves the consumer `Pin` and the ~48 KiB
   single-core dead weight, and the proposed ruling is the relocation
   round. Deviation 5 (raw aliasing): AUDITED with a CONFIRMED hole
   (2026-07-19, record `202607201600`): the between-frames `&mut`
   surface aliases a parked worker's held `&Scheduler`
   (miscompilation-class, timing-independent), so the plane relocation
   is soundness-required and deviations 1, 5, and 6 resolve in one
   round; the finding is catalogued as an ignored test. Deviation 2 (core-ownership mask): EVIDENCE DELIVERED (record
   `202607202200`): the asm-gate five-check re-ran green (zero indirect
   calls in every dispatch body) and the perf gate stands as the second
   oracle; the bless holds with the build-script-codegen escalation
   armed. Deviation 4 (pointer-size spawn): DESIGN MEMO DELIVERED
   (record `202607202000`): sound as shipped, contract gap named, the
   proposed ruling blesses the mechanism, lifts the size check to
   compile time, and folds inline-storage widening into the plane
   relocation round. Deviation 3 (park-immediately): RESOLVED BY BENCH
   (2026-07-19, round 202607202310 + follow-through 202607202351,
   record `202607202340_wake-policy-evidence.md`, margins and mechanism
   corrected by the re-examination `202607210100`, the controlling
   evidence): the canonical
   spin-then-park middle tier was built (budget parameter on both
   frame waits, consumer-tunable `RunCfg::WAKE_SPIN_BUDGET`,
   budget-sweep equivalence test) and the wake_policy bench ruled for
   park-immediately, reproducibly: across seven wait policies, three
   sizes, and three invocations park is never worse and no spin
   policy is ever better (the round-1 point margins are retired as
   variance-inflated; the spin iteration is ISB on aarch64 at 9 to
   20 ns). The default is 0, the machinery ships tunable, and any
   future `pick_tier` telemetry selection must beat park-immediately
   on a paired within-process oracle. The batch's deviation channels are
   now all delivered.
6. **`PlanAffecting`** is ruled an open marker trait (A2-3); the ruling is
   made, and the remaining interplay detail (what a plan-affecting swap
   propagates) resolves inside the swap-semantics item. See [[storage]] and
   [[scheduler]].
7. **Head+tail `mid_slot` semantics**: RESOLVED as a made design call
   (2026-07-19, record `202607201500_mid-slot-semantics-call.md`):
   `mid_slot` is a RECORD boundary (uniform with `RecordRange::Full`,
   layout-independent, protocol-fitting), morsel-aligned by
   construction; the `mid_record` rename and the static-seed-versus-
   dynamic-cursor doc note land with the G2-Nd convergence build round.
8. **`LIGHT_THRESHOLD`**: RESOLVED by registration (2026-07-19, record
   `202607202100_light-threshold-gate-record.md`): a bench-set tunable
   constant whose registry row carries value BENCH-PENDING by design,
   the adaptive-versus-pipe-chase crossover sweep as its oracle, and
   the adapt-phase strategy build as the oracle's gate. The value lands
   as a post-freeze registry amendment when the gate opens.

## Chapter map

| Chapter | Covers |
|---|---|
| [[identity]] | What hilavitkutin is, crates, scope, vocabulary |
| [[foundations]] | arvo substrate, platform tiers, build-time optimisation |
| [[data-model]] | Column layout, value types, stores, storage contract, determinism |
| [[storage]] | The resource storage model (handle + blob + snapshot) |
| [[contracts]] | WorkUnit trait, Context access rules, virtual flags |
| [[plan]] | Phases, trunks, fibers, morsels, the nine plan steps |
| [[dispatch]] | Devirtualised dispatch, const-eval grouping, codegen, intrinsics |
| [[execution]] | Threading, strategy selection, adaptation, the perf gate |
| [[scheduler]] | Builder, self-hosting meta pipeline, errors, persistence, plugins |
| [[constraints]] | Cross-cutting bans, principles, toolchain constraint notes |

`MANIFEST.md` records, per chapter, which sources drained into it and what
was deliberately left out as superseded.
