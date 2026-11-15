# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_intrinsic_const dominates: 22% faster than the next best (rot_manual_shifts)

rot_intrinsic_const (65 ns) leads rot_manual_shifts (79 ns) by 22%, a clear separation rather than a photo finish. CV 105.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rot_variable_amount is an outlier: 2.8x slower than the field

rot_variable_amount (181 ns) is 2.8x the fastest (65 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 105.0%)

rot_intrinsic_const wins on median (65 ns) yet has the highest variance (CV 105.0%), while rot_manual_shifts is the steadiest (CV 85.3%, 79 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (65 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### rot_manual_shifts is inconsistent: worst-20% is 3.7x its best-20%

rot_manual_shifts's best 20% of batches run at 64 ns but its worst 20% at 236 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 64.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.80x (fastest 64.6 ns, slowest 180.8 ns)

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
| rot_intrinsic_const | 2615ns | 1612ns | 1597ns | 1874ns | 5854ns | base |
| rot_manual_shifts | 2726ns | 1957ns | 1598ns | 2027ns | 5948ns | +4.24% |
| rot_variable_amount | 2809ns | 1729ns | 1717ns | 2106ns | 6010ns | +7.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 105ns | 64ns | 232ns | base | 9.784 |
| rot_manual_shifts | 109ns | 64ns | 236ns | +4.31% | 9.380 |
| rot_variable_amount | 294ns | 180ns | 625ns | +180.73% | 3.485 |

## Performance model

- Peak throughput: **15.932 Gops/s** (rot_intrinsic_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 15.851 | 99.5% |
| rot_manual_shifts | 12.995 | 81.6% |
| rot_variable_amount | 5.664 | 35.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2615ns | 2615ns | base |
| rot_manual_shifts | 2726ns | 2726ns | +4.24% |
| rot_variable_amount | 2809ns | 2809ns | +7.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 65ns | base | --- | [65, 79] | --- | --- | --- | --- |
| rot_manual_shifts | 79ns | no significant difference | [+0, +14]ns | [79, 79] | no (adj: YES) | 0.0077 | 0.0077 | 15 |
| rot_variable_amount | 181ns | +116.4ns (+180.2%) | [+116, +142]ns | [181, 220] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 64ns | +0.0% | +180.1% |
| 1 | 2 | 65ns | +22.1% | +179.5% |
| 1 | 3 | 65ns | -0.1% | +242.0% |
| 1 | 4 | 65ns | +22.1% | +179.7% |
| 1 | 5 | 65ns | +22.0% | +179.6% |
| 1 | 6 | 64ns | +22.3% | +180.0% |
| 1 | 7 | 79ns | -18.1% | +179.8% |
| 1 | 8 | 79ns | -0.1% | +128.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | 0.455 | moderate+ |
| rot_manual_shifts | -0.478 | moderate- |
| rot_variable_amount | -0.376 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 1/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 31.5ns | 104.7ns | 30.1% | HIGH |
| rot_manual_shifts | 32.8ns | 109.2ns | 30.1% | HIGH |
| rot_variable_amount | 31.6ns | 293.8ns | 10.8% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 64.3-232.5 ns)
     64.3 |########################################
     72.7 |#################
     81.1 |
     89.5 |
     97.9 |
    106.3 |
    114.7 |
    123.1 |#
    131.5 |
    140.0 |
    148.4 |
    156.8 |
    165.2 |
    173.6 |
    182.0 |#
    190.4 |#
    198.8 |###
    207.2 |##
    215.6 |##
    224.0 |
  (14 below, 14 above range)

rot_manual_shifts (n=160, range 64.4-236.1 ns)
     64.4 |#################
     73.0 |########################################
     81.6 |
     90.1 |
     98.7 |
    107.3 |
    115.9 |
    124.5 |
    133.1 |
    141.7 |
    150.3 |
    158.8 |#
    167.4 |
    176.0 |#
    184.6 |#
    193.2 |
    201.8 |##
    210.4 |#
    219.0 |#
    227.5 |#
  (8 below, 14 above range)

rot_variable_amount (n=160, range 180.4-624.8 ns)
    180.4 |########################################
    202.6 |##############
    224.8 |
    247.1 |
    269.3 |
    291.5 |
    313.7 |
    335.9 |
    358.2 |
    380.4 |
    402.6 |
    424.8 |
    447.1 |
    469.3 |
    491.5 |
    513.7 |#
    535.9 |#
    558.2 |##
    580.4 |#
    602.6 |###
  (6 below, 12 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=64.8% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **rot_intrinsic_const**: bridge=30.2% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=61.6% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_manual_shifts**: bridge=30.2% of algo (FFI overhead may distort results)
- **rot_variable_amount**: CV=61.4% (high variance, measurements may be unstable)
- **rot_variable_amount**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **rot_variable_amount**: bridge=10.9% of algo (FFI overhead may distort results)
