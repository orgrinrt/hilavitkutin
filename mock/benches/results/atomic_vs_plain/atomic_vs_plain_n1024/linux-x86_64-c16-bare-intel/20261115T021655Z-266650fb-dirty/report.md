# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.15 us to slowest 1.15 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### acc_atomic_relaxed is fastest but the noisiest (CV 55.2%)

acc_atomic_relaxed wins on median (1.15 us) yet has the highest variance (CV 55.2%), while acc_atomic_seqcst is the steadiest (CV 43.7%, 1.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (634 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### acc_plain's comparison is tie-heavy (11% tied pairs)

11% of paired samples for acc_plain are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader acc_atomic_relaxed vs stability leader acc_atomic_seqcst (+0% speed for 1.3x steadier)

acc_atomic_relaxed is fastest (1.15 us, CV 55.2%); acc_atomic_seqcst gives up 0.0% median for 1.3x lower variance (CV 43.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### acc_atomic_relaxed is inconsistent: worst-20% is 2.3x its best-20%

acc_atomic_relaxed's best 20% of batches run at 939 ns but its worst 20% at 2.14 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 1147.6 ns median
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
| Counter frequency | 2112779839 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 3379ns | 3024ns | 2474ns | 2913ns | 5682ns | base |
| acc_atomic_seqcst | 3175ns | 3024ns | 2473ns | 2896ns | 4715ns | -6.04% |
| acc_plain | 3409ns | 3027ns | 2732ns | 3028ns | 5229ns | +0.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1279ns | 939ns | 2140ns | base | 0.801 |
| acc_atomic_seqcst | 1201ns | 939ns | 1773ns | -6.04% | 0.852 |
| acc_plain | 1290ns | 1037ns | 1968ns | +0.86% | 0.794 |

## Performance model

- Peak throughput: **1.091 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.892 | 81.8% |
| acc_atomic_seqcst | 0.892 | 81.8% |
| acc_plain | 0.892 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3379ns | 3379ns | base |
| acc_atomic_seqcst | 3175ns | 3175ns | -6.04% |
| acc_plain | 3409ns | 3409ns | +0.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| acc_atomic_seqcst | 1148ns | no significant difference | [-58, +0]ns | [1148, 1148] | no (adj: YES) | 0.0450 | 0.0450 | 4 |
| acc_plain | 1148ns | no significant difference | [+0, +5]ns | [1148, 1148] | no (adj: YES) | 0.0450 | 0.0355 | **18** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -18.2% | +0.0% |
| 1 | 2 | 1148ns | -18.2% | +0.0% |
| 1 | 3 | 939ns | +22.2% | +22.2% |
| 1 | 4 | 939ns | +22.2% | +22.2% |
| 1 | 5 | 1148ns | -18.2% | -0.0% |
| 1 | 6 | 1148ns | +0.0% | -0.0% |
| 1 | 7 | 1148ns | -0.0% | -18.2% |
| 1 | 8 | 939ns | +22.2% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.058 | ok |
| acc_atomic_seqcst | 0.058 | ok |
| acc_plain | -0.161 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 3/8, lost 3/8
- **acc_plain**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 26.6ns | 1278.6ns | 2.1% |  |
| acc_atomic_seqcst | 25.0ns | 1201.4ns | 2.1% |  |
| acc_plain | 26.7ns | 1289.6ns | 2.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 939.0-2139.8 ns)
    939.0 |###################
    999.0 |
   1059.1 |
   1119.1 |########################################
   1179.1 |
   1239.2 |
   1299.2 |
   1359.3 |
   1419.3 |
   1479.3 |
   1539.4 |
   1599.4 |
   1659.5 |
   1719.5 |
   1779.5 |
   1839.6 |
   1899.6 |
   1959.7 |
   2019.7 |
   2079.7 |
  (8 below, 16 above range)

acc_atomic_seqcst (n=160, range 939.0-1772.9 ns)
    939.0 |###################
    980.7 |
   1022.4 |
   1064.1 |
   1105.8 |
   1147.5 |########################################
   1189.2 |
   1230.8 |
   1272.5 |
   1314.2 |
   1355.9 |
   1397.6 |
   1439.3 |
   1481.0 |
   1522.7 |
   1564.4 |
   1606.1 |
   1647.8 |
   1689.5 |
   1731.2 |
  (9 below, 12 above range)

acc_plain (n=160, range 1037.1-1968.1 ns)
   1037.1 |
   1083.6 |
   1130.2 |########################################
   1176.7 |
   1223.3 |
   1269.8 |
   1316.4 |
   1362.9 |
   1409.5 |
   1456.0 |
   1502.6 |
   1549.1 |
   1595.7 |
   1642.2 |
   1688.8 |
   1735.3 |
   1781.9 |
   1828.4 |
   1875.0 |
   1921.5 |
  (17 below, 13 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=49.6% (high variance, measurements may be unstable)
- **acc_atomic_seqcst**: CV=41.7% (high variance, measurements may be unstable)
- **acc_plain**: CV=43.4% (high variance, measurements may be unstable)
