# A1-1 Expanded: Ordering Theories Benched, RCM Confirmed Where It Matters

**Date:** 2026-07-19
**Status:** bench-decided; supersedes the evidence base of
`202607200600_a1-1-rcm-order-bench-resolution.md` (the ruling stands, the
grounds change). Op directed the expansion: more arms than the original
two, rival ordering theories rather than RCM alone, and realistic size
bounds up to the lifted input ceiling.
**Benches:** `mock/benches/` benches `rcm_order` (chain, arms `rcm_adj` /
`rcm_rev` / `rcm_half` / `rcm_scr`) and `rcm_rivals` (3x4 grid, arms
`riv_rcm` / `riv_spec` / `riv_snake` / `riv_rowm` / `riv_scr`), sizes
n = 64 through 16384 (records = n x 1024; 256 KiB through 64 MiB per
column). Results committed at `mock/benches/results/`.

## What the expansion asked

The original resolution proved only "adjacency beats one scramble" on a
chain, at three sizes. Two sharper questions replaced it. First, is
column adjacency actually the mechanism (a dose-response across 7/7,
4/7, and 0/7 adjacent transition orders, plus a reversed-numbering
control)? Second, is RCM the right THEORY, against rivals with different
objectives: spectral/Fiedler ordering minimising total reuse distance
(the minimum-linear-arrangement objective), greedy nearest-neighbour
maximising immediate consecutive sharing, and naive row-major as the
registration-order baseline? The chain cannot separate those theories
(they collapse to the same order on it); the 3x4 grid sharing topology
(columns are the 17 grid edges, WU n reads the edges incident to node n,
all write-disjoint at one topo depth) separates them measurably:
bandwidth 3 to 9, total linear-arrangement distance 35 to 65, adjacent
transitions 0/11 to 11/11 across the five arms.

## The harness incident the expansion surfaced

The first expanded run aborted at the large sizes and, worse, its
completed stages carried skewed numbers: the harness's validation phase
then ran every variant in the orchestrator process, so each variant's
cached gigabyte-class engine state accreted there and the timed passes
of later stages ran on a memory-starved host. The tell in the data:
dose-response inverted at large sizes and the reverse-adjacency control
split from adjacency by tens of percent. Fixes shipped upstream before
any number below was accepted: validation moved into per-variant worker
subprocesses (mockspace PR #278), and results became transactional,
staged in-flight and promoted only on orderly run completion, with
crash-borne trees auto-voided to `results/void/` (PR #279). Every
number below is from one clean full run under the fixed harness, one
process resident at a time.

## Results (warm medians, M1, two harness runs x three passes)

Chain (`rcm_order`), spread of all four arms relative to the best arm
per size: 5.2% (n=64), 8.2% (256), 9.0% (1024), 5.4% (2048), 4.4%
(4096), 4.2% (8192), 7.4% (16384). No arm wins consistently, the
ordering of arms reshuffles per size, and the adjacency dose-response
(adj at 7/7 fastest, scr at 0/7 slowest) never materialises; the
reversed control diverges from adjacency in both directions. Verdict:
on a one-depth chain with one shared column per neighbour pair, the
dispatch-order effect is inside the noise band. The original 1.10x
chain claim is retracted as an artifact of the three-size sample.

Grid (`rcm_rivals`), medians at the two largest sizes:

| n | column | riv_rcm | riv_snake | riv_scr | riv_spec | riv_rowm |
|---|---|---|---|---|---|---|
| 8192 | 32 MiB | 31.87 ms | 31.84 ms | 31.77 ms | 31.86 ms | 31.70 ms |
| 16384 | 64 MiB | **105.04 ms** | 108.44 ms (1.03x) | 109.99 ms (1.05x) | 123.17 ms (1.17x) | 172.90 ms (1.65x) |

At n = 64 through 8192 (working set up to ~1 GiB) the five theories sit
within ~5% with no stable ranking: whenever the working set is
cache-and-prefetch tractable, dispatch order is close to free. At
n = 16384 (~1.9 GiB working set, deep DRAM regime, the engine's actual
large-entity-count target) the theories separate decisively: RCM's
bandwidth-minimising order wins outright; greedy immediate-reuse and
even the no-consecutive-sharing scramble trail it modestly; spectral's
total-distance-minimising order loses 1.17x; the naive row-major
baseline collapses at 1.65x. Bounding the MAXIMUM reuse distance is
what keeps a shared column's second reader inside a survivable window
when the whole set is DRAM-resident; minimising the average (spectral)
tolerates a few long-distance edges that each miss the window, and
row-major concentrates its distance on all eight vertical edges at
once.

## The ruling this records

1. Canon Step 5 stands, on stronger grounds than before: the RCM row
   order is the execution order, now validated against rival ordering
   theories rather than against a single scramble, at realistic scale.
   RCM is best-in-class among the tested orderings exactly in the
   regime the engine targets, and the naive registration order it
   replaces is the worst there by a wide margin.
2. The mechanism refines: the win is NOT generic consecutive-column
   adjacency (the chain shows that effect is noise) but bounded maximum
   reuse distance under DRAM-resident working sets. Step 5's wording
   needs no change; this note records the sharper why.
3. Below the DRAM-dominated scale, dispatch order among valid
   topological orders is measured as near-neutral (~5%). The auto-RCM
   application (the guarded-walk mechanism, sketch `202606090300`)
   remains the implementing work and remains justified by the
   large-scale regime; landing it still dissolves the provisional
   producer-before-consumer registration constraint.
4. The A1-1 fork stays closed. The seed plan chapter's Step 5 record
   now cites this memo as the evidence; the `202607200600` memo remains
   as paper trail of the first, weaker sample.
