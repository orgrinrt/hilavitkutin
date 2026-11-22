# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (5.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_atomic_seqcst at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (26 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 3 variants sit between 5.06 us and 5.07 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_atomic_seqcst's edge over baseline is significant but tiny (-4 ns, 0.07%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by -4 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_atomic_seqcst** at 5062.3 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 5062.3 ns, slowest 5068.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 7288ns | 7180ns | 7161ns | 7233ns | 7581ns | base |
| acc_atomic_seqcst | 7180ns | 7166ns | 7158ns | 7167ns | 7239ns | -1.49% |
| acc_plain | 7192ns | 7168ns | 7160ns | 7171ns | 7287ns | -1.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 5148ns | 5059ns | 5349ns | base | 0.796 |
| acc_atomic_seqcst | 5072ns | 5058ns | 5114ns | -1.47% | 0.808 |
| acc_plain | 5080ns | 5057ns | 5148ns | -1.33% | 0.806 |

## Performance model

- Peak throughput: **0.810 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.808 | 99.8% |
| acc_atomic_seqcst | 0.809 | 99.9% |
| acc_plain | 0.809 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 7288ns | 7288ns | base |
| acc_atomic_seqcst | 7180ns | 7180ns | -1.49% |
| acc_plain | 7192ns | 7192ns | -1.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 5069ns | base | --- | [5065, 5098] | --- | --- | --- | --- |
| acc_atomic_seqcst | 5062ns | -3.8ns (-0.1%) | [-24, -1]ns | [5062, 5064] | YES | 0.0000 | 0.0000 | 5 |
| acc_plain | 5063ns | -3.4ns (-0.1%) | [-22, -2]ns | [5062, 5064] | YES | 0.0001 | 0.0001 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 5064ns | -0.0% | -0.0% |
| 1 | 2 | 5249ns | -3.6% | -3.2% |
| 1 | 3 | 5063ns | -0.0% | -0.0% |
| 1 | 4 | 5063ns | +0.0% | +0.0% |
| 1 | 5 | 5065ns | -0.0% | -0.1% |
| 1 | 6 | 5064ns | -0.0% | +0.1% |
| 1 | 7 | 5253ns | -3.6% | -3.6% |
| 1 | 8 | 5247ns | -3.6% | -3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.054 | ok |
| acc_atomic_seqcst | -0.021 | ok |
| acc_plain | -0.311 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 0/8
- **acc_plain**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.6ns | 5148.1ns | 0.0% |  |
| acc_atomic_seqcst | 2.5ns | 5072.3ns | 0.0% |  |
| acc_plain | 2.7ns | 5079.6ns | 0.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 5059.1-5349.2 ns)
   5059.1 |########################################
   5073.6 |####
   5088.1 |##
   5102.6 |##
   5117.1 |#
   5131.7 |##
   5146.2 |#
   5160.7 |#
   5175.2 |
   5189.7 |
   5204.2 |#
   5218.7 |
   5233.2 |######
   5247.7 |######
   5262.2 |
   5276.7 |#
   5291.2 |
   5305.7 |#
   5320.2 |#
   5334.7 |
  (14 below, 15 above range)

acc_atomic_seqcst (n=160, range 5058.0-5114.4 ns)
   5058.0 |########################################
   5060.9 |######################
   5063.7 |######################
   5066.5 |###########
   5069.3 |#
   5072.1 |
   5075.0 |
   5077.8 |
   5080.6 |###
   5083.4 |
   5086.2 |
   5089.1 |
   5091.9 |#
   5094.7 |
   5097.5 |
   5100.3 |
   5103.1 |
   5106.0 |#
   5108.8 |
   5111.6 |
  (14 below, 15 above range)

acc_plain (n=160, range 5057.3-5148.3 ns)
   5057.3 |########################################
   5061.8 |#############################
   5066.4 |########
   5070.9 |###
   5075.5 |#
   5080.0 |##
   5084.6 |
   5089.1 |
   5093.7 |
   5098.2 |##
   5102.8 |##
   5107.3 |
   5111.9 |
   5116.4 |#
   5121.0 |
   5125.5 |
   5130.1 |
   5134.6 |#
   5139.2 |
   5143.7 |
  (14 below, 11 above range)

```
