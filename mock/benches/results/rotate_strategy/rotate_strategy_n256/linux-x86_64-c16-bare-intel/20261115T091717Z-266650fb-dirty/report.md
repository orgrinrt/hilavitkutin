# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_intrinsic_const dominates: 22% faster than the next best (rot_manual_shifts)

rot_intrinsic_const (21 ns) leads rot_manual_shifts (25 ns) by 22%, a clear separation rather than a photo finish. CV 105.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rot_variable_amount is an outlier: 2.8x slower than the field

rot_variable_amount (59 ns) is 2.8x the fastest (21 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 105.8%)

rot_intrinsic_const wins on median (21 ns) yet has the highest variance (CV 105.8%), while rot_manual_shifts is the steadiest (CV 87.0%, 25 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (21 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### rot_manual_shifts's comparison is tie-heavy (22% tied pairs)

22% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_variable_amount is inconsistent: worst-20% is 3.7x its best-20%

rot_variable_amount's best 20% of batches run at 48 ns but its worst 20% at 179 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 20.9 ns median
- 1 variant significantly slower than baseline
- Spread: 2.81x (fastest 20.9 ns, slowest 58.8 ns)

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
| Counter frequency | 2112768874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2682ns | 1566ns | 1551ns | 2083ns | 5609ns | base |
| rot_manual_shifts | 2622ns | 1901ns | 1555ns | 1917ns | 5803ns | -2.24% |
| rot_variable_amount | 2644ns | 1938ns | 1585ns | 1910ns | 5908ns | -1.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 36ns | 21ns | 74ns | base | 7.177 |
| rot_manual_shifts | 35ns | 21ns | 77ns | -2.17% | 7.336 |
| rot_variable_amount | 80ns | 48ns | 179ns | +124.68% | 3.194 |

## Performance model

- Peak throughput: **12.354 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.249 | 99.1% |
| rot_manual_shifts | 10.079 | 81.6% |
| rot_variable_amount | 4.354 | 35.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2682ns | 2682ns | base |
| rot_manual_shifts | 2622ns | 2622ns | -2.24% |
| rot_variable_amount | 2644ns | 2644ns | -1.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 21ns | base | --- | [21, 25] | --- | --- | --- | --- |
| rot_manual_shifts | 25ns | no significant difference | [+0, +0]ns | [25, 25] | no (adj: YES) | 0.0001 | 0.0001 | **35** (22%, HIGH) |
| rot_variable_amount | 59ns | +38.0ns (+181.8%) | [+32, +38]ns | [59, 59] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 21ns | +21.5% | +130.6% |
| 1 | 2 | 21ns | +0.0% | +131.7% |
| 1 | 3 | 21ns | +0.0% | +183.2% |
| 1 | 4 | 21ns | +0.5% | +183.2% |
| 1 | 5 | 21ns | +0.0% | +183.2% |
| 1 | 6 | 21ns | +22.6% | +183.2% |
| 1 | 7 | 25ns | +0.0% | +89.8% |
| 1 | 8 | 25ns | +0.2% | +89.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | 0.460 | moderate+ |
| rot_manual_shifts | 0.369 | moderate+ |
| rot_variable_amount | 0.375 | moderate+ |

**Consistency summary:**

- **rot_manual_shifts**: won 0/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 33.4ns | 35.7ns | 93.6% | HIGH |
| rot_manual_shifts | 32.4ns | 34.9ns | 92.8% | HIGH |
| rot_variable_amount | 32.3ns | 80.1ns | 40.3% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 20.7-74.1 ns)
     20.7 |########################################
     23.4 |####################
     26.1 |
     28.7 |
     31.4 |
     34.1 |
     36.7 |
     39.4 |
     42.1 |
     44.7 |
     47.4 |
     50.1 |
     52.7 |#
     55.4 |
     58.1 |
     60.7 |####
     63.4 |####
     66.1 |
     68.7 |########
     71.4 |
  (25 below, 15 above range)

rot_manual_shifts (n=160, range 20.8-77.0 ns)
     20.8 |########################
     23.6 |########################################
     26.4 |
     29.2 |
     32.0 |
     34.8 |
     37.6 |
     40.4 |
     43.3 |
     46.1 |
     48.9 |
     51.7 |#
     54.5 |
     57.3 |
     60.1 |
     63.0 |#
     65.8 |
     68.6 |##
     71.4 |#
     74.2 |##
  (15 below, 14 above range)

rot_variable_amount (n=160, range 48.1-178.7 ns)
     48.1 |###################
     54.6 |########################################
     61.2 |
     67.7 |
     74.2 |
     80.8 |
     87.3 |
     93.8 |
    100.4 |
    106.9 |
    113.4 |
    120.0 |
    126.5 |#
    133.0 |
    139.5 |#
    146.1 |
    152.6 |#
    159.1 |###
    165.7 |#
    172.2 |###
  (25 below, 12 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=62.0% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **rot_intrinsic_const**: bridge=92.8% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=63.3% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_manual_shifts**: bridge=92.5% of algo (FFI overhead may distort results)
- **rot_variable_amount**: CV=64.7% (high variance, measurements may be unstable)
- **rot_variable_amount**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_variable_amount**: bridge=40.5% of algo (FFI overhead may distort results)
