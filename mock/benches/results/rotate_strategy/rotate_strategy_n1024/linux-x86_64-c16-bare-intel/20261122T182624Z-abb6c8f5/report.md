# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_manual_shifts dominates: 21% faster than the next best (rot_intrinsic_const)

rot_manual_shifts (65 ns) leads rot_intrinsic_const (79 ns) by 21%, a clear separation rather than a photo finish. CV 104.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rot_variable_amount is an outlier: 3.4x slower than the field

rot_variable_amount (220 ns) is 3.4x the fastest (65 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_manual_shifts is fastest but the noisiest (CV 104.8%)

rot_manual_shifts wins on median (65 ns) yet has the highest variance (CV 104.8%), while rot_variable_amount is the steadiest (CV 77.5%, 220 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.4x the fastest

Fastest rot_manual_shifts (65 ns) to slowest rot_variable_amount (220 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### rot_intrinsic_const is inconsistent: worst-20% is 3.8x its best-20%

rot_intrinsic_const's best 20% of batches run at 64 ns but its worst 20% at 246 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rot_manual_shifts** at 65.3 ns median (-17.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.37x (fastest 65.3 ns, slowest 220.4 ns)

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
| rot_intrinsic_const | 2914ns | 1956ns | 1598ns | 2259ns | 6195ns | base |
| rot_manual_shifts | 2649ns | 1633ns | 1598ns | 1901ns | 5942ns | -9.09% |
| rot_variable_amount | 2703ns | 2096ns | 1714ns | 2000ns | 5801ns | -7.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 117ns | 64ns | 246ns | base | 8.778 |
| rot_manual_shifts | 105ns | 64ns | 236ns | -9.74% | 9.726 |
| rot_variable_amount | 283ns | 180ns | 603ns | +142.62% | 3.618 |

## Performance model

- Peak throughput: **15.916 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.995 | 81.6% |
| rot_manual_shifts | 15.669 | 98.5% |
| rot_variable_amount | 4.647 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2914ns | 2914ns | base |
| rot_manual_shifts | 2649ns | 2649ns | -9.09% |
| rot_variable_amount | 2703ns | 2703ns | -7.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 79ns | base | --- | [79, 79] | --- | --- | --- | --- |
| rot_manual_shifts | 65ns | -14.0ns (-17.8%) | [-14, -0]ns | [65, 79] | YES | 0.0303 | 0.0303 | 4 |
| rot_variable_amount | 220ns | +141.5ns (+179.6%) | [+116, +142]ns | [181, 220] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 79ns | -18.3% | +179.6% |
| 1 | 2 | 65ns | +22.1% | +179.3% |
| 1 | 3 | 79ns | -18.2% | +128.8% |
| 1 | 4 | 64ns | +0.5% | +241.9% |
| 1 | 5 | 65ns | +22.1% | +179.3% |
| 1 | 6 | 218ns | -70.4% | -17.2% |
| 1 | 7 | 64ns | +1.9% | +179.5% |
| 1 | 8 | 79ns | -18.1% | +179.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.244 | moderate- |
| rot_manual_shifts | -0.415 | moderate- |
| rot_variable_amount | -0.275 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 4/8
- **rot_variable_amount**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 35.2ns | 116.6ns | 30.2% | HIGH |
| rot_manual_shifts | 31.8ns | 105.3ns | 30.2% | HIGH |
| rot_variable_amount | 30.3ns | 283.0ns | 10.7% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 64.4-246.4 ns)
     64.4 |####################################
     73.5 |########################################
     82.6 |
     91.7 |#
    100.8 |
    109.9 |
    119.0 |
    128.1 |
    137.2 |#
    146.3 |#
    155.4 |
    164.5 |
    173.6 |###
    182.7 |#
    191.8 |
    200.9 |###
    210.0 |#####
    219.1 |
    228.2 |###
    237.3 |
  (9 below, 15 above range)

rot_manual_shifts (n=160, range 64.3-235.7 ns)
     64.3 |########################################
     72.9 |##############
     81.5 |
     90.0 |#
     98.6 |
    107.2 |
    115.8 |#
    124.3 |
    132.9 |
    141.5 |
    150.0 |
    158.6 |
    167.2 |
    175.8 |
    184.3 |
    192.9 |
    201.5 |##
    210.0 |###
    218.6 |#
    227.2 |##
  (11 below, 16 above range)

rot_variable_amount (n=160, range 180.3-603.4 ns)
    180.3 |########################################
    201.4 |#############################
    222.6 |
    243.7 |
    264.9 |
    286.1 |
    307.2 |
    328.4 |
    349.5 |
    370.7 |
    391.8 |
    413.0 |
    434.1 |
    455.3 |
    476.5 |#
    497.6 |##
    518.8 |###
    539.9 |
    561.1 |###
    582.2 |#
  (9 below, 12 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=62.2% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **rot_intrinsic_const**: bridge=30.2% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=65.0% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_manual_shifts**: bridge=30.3% of algo (FFI overhead may distort results)
- **rot_variable_amount**: CV=60.3% (high variance, measurements may be unstable)
- **rot_variable_amount**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **rot_variable_amount**: bridge=10.7% of algo (FFI overhead may distort results)
