# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (1.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_atomic_seqcst at 1.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (500 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader acc_atomic_seqcst vs stability leader acc_atomic_relaxed (+0% speed for 1.1x steadier)

acc_atomic_seqcst is fastest (1.15 us, CV 43.5%); acc_atomic_relaxed gives up 0.0% median for 1.1x lower variance (CV 40.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### acc_plain is inconsistent: worst-20% is 1.9x its best-20%

acc_plain's best 20% of batches run at 939 ns but its worst 20% at 1.82 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.0% of the fastest

All 3 variants sit between 1.15 us and 1.15 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_plain's edge over baseline is significant but tiny (-0 ns, 0.02%)

acc_plain differs from baseline acc_atomic_relaxed by -0 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_atomic_seqcst** at 1147.6 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1147.6 ns, slowest 1147.7 ns)

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
| Counter frequency | 2112780760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 3262ns | 3028ns | 2732ns | 3030ns | 4489ns | base |
| acc_atomic_seqcst | 3247ns | 3025ns | 2476ns | 2994ns | 4778ns | -0.44% |
| acc_plain | 3149ns | 3023ns | 2474ns | 2807ns | 4849ns | -3.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1235ns | 1037ns | 1692ns | base | 0.829 |
| acc_atomic_seqcst | 1229ns | 939ns | 1800ns | -0.48% | 0.833 |
| acc_plain | 1190ns | 939ns | 1820ns | -3.62% | 0.860 |

## Performance model

- Peak throughput: **1.091 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.892 | 81.8% |
| acc_atomic_seqcst | 0.892 | 81.8% |
| acc_plain | 0.892 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3262ns | 3262ns | base |
| acc_atomic_seqcst | 3247ns | 3247ns | -0.44% |
| acc_plain | 3149ns | 3149ns | -3.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| acc_atomic_seqcst | 1148ns | no significant difference | [-0, +0]ns | [1148, 1148] | no | 0.0987 | 0.0987 | 13 |
| acc_plain | 1148ns | -0.2ns (-0.0%) | [-11, -0]ns | [950, 1148] | YES | 0.0000 | 0.0000 | 15 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 939ns | +22.2% | +0.0% |
| 1 | 2 | 1148ns | +0.0% | +0.0% |
| 1 | 3 | 1148ns | -18.2% | +0.0% |
| 1 | 4 | 1148ns | +0.0% | -18.2% |
| 1 | 5 | 1148ns | +0.0% | +0.0% |
| 1 | 6 | 1148ns | -18.2% | -18.2% |
| 1 | 7 | 1148ns | +0.0% | -18.2% |
| 1 | 8 | 1148ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.018 | ok |
| acc_atomic_seqcst | -0.375 | moderate- |
| acc_plain | -0.375 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 2/8, lost 1/8
- **acc_plain**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 25.6ns | 1234.7ns | 2.1% |  |
| acc_atomic_seqcst | 25.5ns | 1228.8ns | 2.1% |  |
| acc_plain | 24.8ns | 1190.1ns | 2.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 1037.0-1692.5 ns)
   1037.0 |
   1069.8 |
   1102.5 |
   1135.3 |########################################
   1168.1 |
   1200.9 |
   1233.6 |
   1266.4 |
   1299.2 |
   1332.0 |
   1364.7 |
   1397.5 |
   1430.3 |
   1463.1 |
   1495.8 |
   1528.6 |
   1561.4 |
   1594.2 |
   1626.9 |
   1659.7 |
  (17 below, 8 above range)

acc_atomic_seqcst (n=160, range 939.0-1799.7 ns)
    939.0 |###########
    982.0 |
   1025.1 |
   1068.1 |
   1111.1 |########################################
   1154.2 |######
   1197.2 |
   1240.2 |
   1283.3 |
   1326.3 |
   1369.3 |
   1412.4 |
   1455.4 |
   1498.4 |
   1541.5 |
   1584.5 |
   1627.6 |
   1670.6 |
   1713.6 |
   1756.7 |
  (11 below, 11 above range)

acc_plain (n=160, range 939.0-1820.1 ns)
    939.0 |###################################
    983.0 |
   1027.1 |
   1071.1 |
   1115.2 |########################################
   1159.2 |####
   1203.3 |
   1247.4 |
   1291.4 |#
   1335.5 |
   1379.5 |
   1423.6 |
   1467.7 |
   1511.7 |
   1555.8 |
   1599.8 |
   1643.9 |
   1688.0 |
   1732.0 |
   1776.1 |
  (13 below, 11 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=37.7% (high variance, measurements may be unstable)
- **acc_atomic_seqcst**: CV=40.7% (high variance, measurements may be unstable)
- **acc_plain**: CV=43.4% (high variance, measurements may be unstable)
