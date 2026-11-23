# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (4.77 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 3.22 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 33% faster than the next best (per_core)

sequential (3.22 ms) leads per_core (4.30 ms) by 33%, a clear separation rather than a photo finish. CV 14.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core beats baseline by 27% (significant)

per_core is -1.29 ms (27%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core shows alternating (throttle bounce) (autocorr -0.66)

per_core's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### per_core is inconsistent: worst-20% is 5.5x its best-20%

per_core's best 20% of batches run at 967.84 us but its worst 20% at 5.28 ms (5.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 3219438.6 ns median (-32.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.48x (fastest 3219438.6 ns, slowest 4766294.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112787067 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 4762281ns | 4767212ns | 2511895ns | 4290422ns | 6595263ns | base |
| per_core | 3489234ns | 4300666ns | 968157ns | 3132425ns | 5284985ns | -26.73% |
| sequential | 3342184ns | 3220608ns | 3199634ns | 3216908ns | 3601374ns | -29.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 4761336ns | 2511149ns | 6594236ns | base | 0.220 |
| per_core | 3485211ns | 967845ns | 5278646ns | -26.80% | 0.301 |
| sequential | 3340149ns | 3198713ns | 3597450ns | -29.85% | 0.314 |

## Performance model

- Peak throughput: **1.083 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.220 | 20.3% |
| per_core | 0.244 | 22.5% |
| sequential | 0.326 | 30.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 4762281ns | 4762281ns | base |
| per_core | 3489234ns | 3489234ns | -26.73% |
| sequential | 3342184ns | 3342184ns | -29.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 4766294ns | base | --- | [3437692, 5930182] | --- | --- | --- | --- |
| per_core | 4296089ns | -1291029.9ns (-27.1%) | [-2110965, -49791]ns | [1131110, 4747162] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| sequential | 3219439ns | -1025171.4ns (-21.5%) | [-2657497, -208262]ns | [3211220, 3243878] | YES (adj: no) | 0.0618 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 4342783ns | +13.3% | -25.8% |
| 1 | 2 | 3083975ns | -68.4% | +3.8% |
| 1 | 3 | 5930182ns | -24.2% | -45.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.460 | moderate- |
| per_core | -0.660 | HIGH- (thermal bounce) |
| sequential | -0.438 | moderate- |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 5212552.9ns | 4761336.1ns | 109.5% | HIGH |
| per_core | 3327868.7ns | 3485211.0ns | 95.5% | HIGH |
| sequential | 3280816.5ns | 3340149.0ns | 98.2% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2511149.4-6594236.0 ns)
  2511149.4 |
  2715303.8 |#############
  2919458.1 |#############
  3123612.4 |#############
  3327766.7 |##########################
  3531921.1 |#############
  3736075.4 |#############
  3940229.7 |
  4144384.0 |
  4348538.4 |
  4552692.7 |#############
  4756847.0 |#############
  4961001.3 |
  5165155.7 |
  5369310.0 |
  5573464.3 |#############
  5777618.6 |########################################
  5981773.0 |##########################
  6185927.3 |
  6390081.6 |
  (1 below, 2 above range)

per_core (n=18, range 967844.9-5278646.3 ns)
  967844.9 |########################################
  1183385.0 |
  1398925.1 |
  1614465.1 |
  1830005.2 |
  2045545.3 |
  2261085.3 |
  2476625.4 |
  2692165.5 |
  2907705.6 |
  3123245.6 |##########
  3338785.7 |
  3554325.8 |
  3769865.8 |
  3985405.9 |##########
  4200946.0 |####################
  4416486.0 |##########
  4632026.1 |##############################
  4847566.2 |##########
  5063106.2 |##########
  (2 below, 2 above range)

sequential (n=18, range 3198713.2-3597449.9 ns)
  3198713.2 |########################################
  3218650.1 |##########################
  3238586.9 |#############
  3258523.7 |
  3278460.6 |#############
  3298397.4 |######
  3318334.2 |
  3338271.1 |
  3358207.9 |
  3378144.7 |
  3398081.6 |
  3418018.4 |
  3437955.2 |
  3457892.1 |
  3477828.9 |
  3497765.7 |
  3517702.6 |
  3537639.4 |
  3557576.2 |
  3577513.1 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=35.6% (high variance, measurements may be unstable)
- **grouped**: bridge=115.1% of algo (FFI overhead may distort results)
- **per_core**: CV=53.2% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 5.5x (possible bimodal distribution)
- **per_core**: bridge=87.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.8% of algo (FFI overhead may distort results)
