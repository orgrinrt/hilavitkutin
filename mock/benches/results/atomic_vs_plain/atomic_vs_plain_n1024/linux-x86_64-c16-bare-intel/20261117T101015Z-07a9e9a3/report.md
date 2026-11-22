# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 939 ns to slowest 939 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (461 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (939 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### acc_plain's comparison is tie-heavy (15% tied pairs)

15% of paired samples for acc_plain are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### acc_atomic_seqcst is inconsistent: worst-20% is 2.2x its best-20%

acc_atomic_seqcst's best 20% of batches run at 939 ns but its worst 20% at 2.06 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 939.1 ns median
- Spread: 1.00x (fastest 939.1 ns, slowest 939.1 ns)

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
| Counter frequency | 2112846992 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 2902ns | 2485ns | 2480ns | 2522ns | 4463ns | base |
| acc_atomic_seqcst | 3093ns | 2484ns | 2478ns | 2503ns | 5476ns | +6.58% |
| acc_plain | 3139ns | 2492ns | 2480ns | 2613ns | 5375ns | +8.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 1094ns | 939ns | 1673ns | base | 0.936 |
| acc_atomic_seqcst | 1167ns | 939ns | 2055ns | +6.60% | 0.878 |
| acc_plain | 1184ns | 939ns | 2021ns | +8.15% | 0.865 |

## Performance model

- Peak throughput: **1.091 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 1.090 | 100.0% |
| acc_atomic_seqcst | 1.090 | 100.0% |
| acc_plain | 1.090 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 2902ns | 2902ns | base |
| acc_atomic_seqcst | 3093ns | 3093ns | +6.58% |
| acc_plain | 3139ns | 3139ns | +8.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 939ns | base | --- | [939, 939] | --- | --- | --- | --- |
| acc_atomic_seqcst | 939ns | no significant difference | [+0, +0]ns | [939, 939] | no | 0.1505 | 0.1505 | **20** (12%, HIGH) |
| acc_plain | 939ns | no significant difference | [+0, +0]ns | [939, 949] | no | 0.1427 | 0.0714 | **24** (15%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 939ns | +0.0% | -0.0% |
| 1 | 2 | 939ns | +79.3% | +0.0% |
| 1 | 3 | 939ns | +3.0% | +38.2% |
| 1 | 4 | 939ns | +0.0% | +0.0% |
| 1 | 5 | 1005ns | -6.6% | -6.6% |
| 1 | 6 | 939ns | +0.0% | +0.0% |
| 1 | 7 | 939ns | +0.0% | +0.0% |
| 1 | 8 | 1148ns | -18.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.090 | ok |
| acc_atomic_seqcst | -0.131 | ok |
| acc_plain | -0.231 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 2/8, lost 2/8
- **acc_plain**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 22.8ns | 1094.3ns | 2.1% |  |
| acc_atomic_seqcst | 24.3ns | 1166.6ns | 2.1% |  |
| acc_plain | 24.8ns | 1183.5ns | 2.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 938.9-1672.9 ns)
    938.9 |########################################
    975.6 |
   1012.3 |
   1049.0 |
   1085.7 |#
   1122.4 |########
   1159.1 |
   1195.8 |
   1232.5 |
   1269.2 |
   1305.9 |
   1342.6 |
   1379.3 |
   1416.0 |
   1452.7 |
   1489.4 |
   1526.1 |
   1562.8 |
   1599.5 |
   1636.2 |
  (26 below, 10 above range)

acc_atomic_seqcst (n=160, range 938.9-2055.1 ns)
    938.9 |########################################
    994.7 |##
   1050.5 |
   1106.3 |
   1162.2 |
   1218.0 |
   1273.8 |
   1329.6 |
   1385.4 |
   1441.2 |
   1497.0 |
   1552.8 |
   1608.6 |
   1664.4 |
   1720.3 |
   1776.1 |
   1831.9 |
   1887.7 |
   1943.5 |
   1999.3 |
  (22 below, 17 above range)

acc_plain (n=160, range 938.9-2021.3 ns)
    938.9 |########################################
    993.1 |
   1047.2 |#
   1101.3 |#########
   1155.4 |#
   1209.5 |
   1263.7 |
   1317.8 |
   1371.9 |
   1426.0 |
   1480.1 |
   1534.3 |
   1588.4 |
   1642.5 |
   1696.6 |
   1750.7 |
   1804.9 |
   1859.0 |
   1913.1 |
   1967.2 |
  (18 below, 15 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=42.1% (high variance, measurements may be unstable)
- **acc_atomic_seqcst**: CV=46.9% (high variance, measurements may be unstable)
- **acc_plain**: CV=45.4% (high variance, measurements may be unstable)
