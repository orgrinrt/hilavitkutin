# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_intrinsic_const dominates: 22% faster than the next best (rot_manual_shifts)

rot_intrinsic_const (21 ns) leads rot_manual_shifts (25 ns) by 22%, a clear separation rather than a photo finish. CV 107.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rot_variable_amount is an outlier: 2.8x slower than the field

rot_variable_amount (59 ns) is 2.8x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 107.1%)

rot_intrinsic_const wins on median (21 ns) yet has the highest variance (CV 107.1%), while rot_variable_amount is the steadiest (CV 85.5%, 59 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (21 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### rot_manual_shifts's comparison is tie-heavy (29% tied pairs)

29% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_manual_shifts is inconsistent: worst-20% is 3.8x its best-20%

rot_manual_shifts's best 20% of batches run at 21 ns but its worst 20% at 79 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 20.9 ns median
- 1 variant significantly slower than baseline
- Spread: 2.82x (fastest 20.9 ns, slowest 58.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 2112817215 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2576ns | 1560ns | 1553ns | 1875ns | 5703ns | base |
| rot_manual_shifts | 2616ns | 1901ns | 1554ns | 1856ns | 5959ns | +1.56% |
| rot_variable_amount | 2715ns | 1939ns | 1586ns | 2020ns | 5930ns | +5.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 34ns | 21ns | 75ns | base | 7.479 |
| rot_manual_shifts | 35ns | 21ns | 79ns | +1.45% | 7.373 |
| rot_variable_amount | 82ns | 48ns | 178ns | +139.08% | 3.128 |

## Performance model

- Peak throughput: **12.330 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.249 | 99.3% |
| rot_manual_shifts | 10.079 | 81.7% |
| rot_variable_amount | 4.350 | 35.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2576ns | 2576ns | base |
| rot_manual_shifts | 2616ns | 2616ns | +1.56% |
| rot_variable_amount | 2715ns | 2715ns | +5.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 21ns | base | --- | [21, 21] | --- | --- | --- | --- |
| rot_manual_shifts | 25ns | no significant difference | [+0, +0]ns | [25, 25] | no (adj: YES) | 0.0001 | 0.0001 | **46** (29%, HIGH) |
| rot_variable_amount | 59ns | +38.0ns (+181.8%) | [+38, +38]ns | [59, 59] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 21ns | +0.0% | +131.7% |
| 1 | 2 | 25ns | +0.0% | +131.5% |
| 1 | 3 | 21ns | +21.5% | +130.6% |
| 1 | 4 | 21ns | +0.0% | +131.7% |
| 1 | 5 | 21ns | -0.5% | +181.8% |
| 1 | 6 | 21ns | +0.0% | +183.2% |
| 1 | 7 | 21ns | +21.8% | +182.5% |
| 1 | 8 | 21ns | +22.6% | +183.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.156 | ok |
| rot_manual_shifts | 0.129 | ok |
| rot_variable_amount | 0.065 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 1/8, lost 3/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 31.8ns | 34.2ns | 92.8% | HIGH |
| rot_manual_shifts | 32.3ns | 34.7ns | 92.9% | HIGH |
| rot_variable_amount | 33.3ns | 81.8ns | 40.6% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 20.8-75.1 ns)
     20.8 |########################################
     23.5 |######
     26.2 |
     28.9 |
     31.6 |
     34.4 |
     37.1 |
     39.8 |
     42.5 |
     45.2 |
     47.9 |
     50.7 |
     53.4 |
     56.1 |
     58.8 |##
     61.5 |
     64.3 |##
     67.0 |#
     69.7 |###
     72.4 |
  (12 below, 14 above range)

rot_manual_shifts (n=160, range 20.8-78.6 ns)
     20.8 |################################
     23.7 |########################################
     26.6 |
     29.5 |
     32.3 |
     35.2 |
     38.1 |
     41.0 |#
     43.9 |
     46.8 |
     49.7 |
     52.6 |
     55.5 |
     58.3 |
     61.2 |
     64.1 |#
     67.0 |
     69.9 |#
     72.8 |#
     75.7 |###
  (5 below, 15 above range)

rot_variable_amount (n=160, range 48.1-177.5 ns)
     48.1 |############
     54.6 |########################################
     61.1 |
     67.5 |
     74.0 |
     80.5 |
     87.0 |
     93.4 |
     99.9 |
    106.4 |
    112.8 |
    119.3 |
    125.8 |
    132.2 |
    138.7 |
    145.2 |
    151.6 |##
    158.1 |###
    164.6 |##
    171.0 |###
  (20 below, 11 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=65.4% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **rot_intrinsic_const**: bridge=92.6% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=65.3% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **rot_manual_shifts**: bridge=92.5% of algo (FFI overhead may distort results)
- **rot_variable_amount**: CV=61.5% (high variance, measurements may be unstable)
- **rot_variable_amount**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_variable_amount**: bridge=40.6% of algo (FFI overhead may distort results)
