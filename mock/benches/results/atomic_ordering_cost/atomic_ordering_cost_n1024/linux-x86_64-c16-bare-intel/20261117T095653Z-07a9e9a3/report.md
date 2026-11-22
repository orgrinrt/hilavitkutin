# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (896 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_seqcst at 670 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_seqcst beats baseline by 39% (significant)

atomic_seqcst is -346 ns (39%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (atomic_seqcst, atomic_relaxed) are a dead heat (<1%)

atomic_seqcst (670 ns) and atomic_relaxed (673 ns) differ by 0.52%, inside the noise, even though the wider field spreads 33.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (227 ns) is smaller than the fastest variant's own run-to-run std-dev (453 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### atomic_seqcst is inconsistent: worst-20% is 3.1x its best-20%

atomic_seqcst's best 20% of batches run at 548 ns but its worst 20% at 1.68 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_seqcst** at 669.5 ns median (-25.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.34x (fastest 669.5 ns, slowest 896.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 2112683200 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 2860ns | 2442ns | 2435ns | 2550ns | 4212ns | base |
| atomic_relaxed | 3183ns | 2554ns | 2085ns | 2518ns | 6273ns | +11.30% |
| atomic_seqcst | 3338ns | 2551ns | 2086ns | 2665ns | 6607ns | +16.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 1049ns | 896ns | 1533ns | base | 0.976 |
| atomic_relaxed | 835ns | 548ns | 1639ns | -20.37% | 1.226 |
| atomic_seqcst | 864ns | 548ns | 1678ns | -17.65% | 1.186 |

## Performance model

- Peak throughput: **1.869 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.143 | 61.1% |
| atomic_relaxed | 1.522 | 81.4% |
| atomic_seqcst | 1.529 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2860ns | 2860ns | base |
| atomic_relaxed | 3183ns | 3183ns | +11.30% |
| atomic_seqcst | 3338ns | 3338ns | +16.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 896ns | base | --- | [896, 896] | --- | --- | --- | --- |
| atomic_relaxed | 673ns | -345.3ns (-38.5%) | [-348, -342]ns | [580, 675] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 670ns | -345.9ns (-38.6%) | [-346, -335]ns | [604, 670] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 1095ns | -49.9% | -49.9% |
| 1 | 2 | 896ns | -24.6% | -38.6% |
| 1 | 3 | 1095ns | -38.5% | -38.8% |
| 1 | 4 | 896ns | -38.2% | +32.9% |
| 1 | 5 | 896ns | -1.6% | -25.2% |
| 1 | 6 | 896ns | -38.8% | +2.1% |
| 1 | 7 | 896ns | -38.8% | -38.8% |
| 1 | 8 | 896ns | +10.8% | -38.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.208 | moderate- |
| atomic_relaxed | -0.363 | moderate- |
| atomic_seqcst | -0.054 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 7/8, lost 1/8
- **atomic_seqcst**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 23.1ns | 1048.8ns | 2.2% |  |
| atomic_relaxed | 28.8ns | 835.2ns | 3.5% |  |
| atomic_seqcst | 30.5ns | 863.7ns | 3.5% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 895.8-1532.8 ns)
    895.8 |########################################
    927.6 |
    959.5 |
    991.3 |
   1023.2 |
   1055.0 |
   1086.9 |################
   1118.7 |
   1150.6 |
   1182.4 |
   1214.3 |
   1246.1 |
   1278.0 |
   1309.8 |
   1341.7 |
   1373.5 |
   1405.4 |
   1437.3 |
   1469.1 |
   1501.0 |
  (10 below, 9 above range)

atomic_relaxed (n=160, range 547.9-1639.2 ns)
    547.9 |########################################
    602.5 |#
    657.0 |##########################
    711.6 |###
    766.2 |###
    820.7 |####
    875.3 |###
    929.9 |##
    984.4 |###
   1039.0 |
   1093.6 |
   1148.1 |
   1202.7 |
   1257.2 |
   1311.8 |
   1366.4 |#
   1420.9 |#
   1475.5 |
   1530.1 |##
   1584.6 |#
  (18 below, 14 above range)

atomic_seqcst (n=160, range 548.4-1678.4 ns)
    548.4 |########################################
    604.9 |
    661.4 |#################################
    717.9 |##
    774.4 |
    830.9 |###
    887.4 |
    943.9 |
   1000.4 |#
   1056.9 |#
   1113.4 |
   1169.9 |
   1226.4 |#
   1282.9 |
   1339.4 |
   1395.9 |######
   1452.4 |##
   1508.9 |####
   1565.4 |##
   1621.9 |
  (21 below, 13 above range)

```

## Diagnostics

- **atomic_acquire**: CV=38.6% (high variance, measurements may be unstable)
- **atomic_relaxed**: CV=55.9% (high variance, measurements may be unstable)
- **atomic_seqcst**: CV=52.4% (high variance, measurements may be unstable)
- **atomic_seqcst**: worst_20/best_20 = 3.1x (possible bimodal distribution)
