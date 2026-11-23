# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (4.61 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 3.23 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 37% faster than the next best (per_core)

sequential (3.23 ms) leads per_core (4.41 ms) by 37%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 30% (significant)

sequential is -1.39 ms (30%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core is inconsistent: worst-20% is 2.9x its best-20%

per_core's best 20% of batches run at 1.72 ms but its worst 20% at 5.04 ms (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 3229861.6 ns median (-30.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.43x (fastest 3229861.6 ns, slowest 4614187.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 1024 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112752809 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 4617560ns | 4616675ns | 3391193ns | 4441656ns | 5494600ns | base |
| per_core | 3769220ns | 4413278ns | 1719398ns | 3601565ns | 5045613ns | -18.37% |
| sequential | 3237321ns | 3231402ns | 3208420ns | 3228397ns | 3265158ns | -29.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 4614826ns | 3389252ns | 5491035ns | base | 0.227 |
| per_core | 3764896ns | 1716701ns | 5039889ns | -18.42% | 0.279 |
| sequential | 3235360ns | 3207470ns | 3262282ns | -29.89% | 0.324 |

## Performance model

- Peak throughput: **0.611 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.227 | 37.2% |
| per_core | 0.238 | 38.9% |
| sequential | 0.325 | 53.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 4617560ns | 4617560ns | base |
| per_core | 3769220ns | 3769220ns | -18.37% |
| sequential | 3237321ns | 3237321ns | -29.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 4614187ns | base | --- | [4176149, 5179899] | --- | --- | --- | --- |
| per_core | 4409851ns | no significant difference | [-1921501, +164569]ns | [2594388, 4607536] | no | 0.2379 | 0.2379 | 0 |
| sequential | 3229862ns | -1389702.9ns (-30.1%) | [-1916955, -956044]ns | [3217560, 3244472] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 4990792ns | -14.0% | -34.9% |
| 1 | 2 | 4601068ns | -2.4% | -29.9% |
| 1 | 3 | 4495135ns | -27.2% | -28.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.066 | ok |
| per_core | -0.260 | moderate- |
| sequential | -0.171 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 4597491.3ns | 4614825.7ns | 99.6% | HIGH |
| per_core | 3893872.3ns | 3764895.8ns | 103.4% | HIGH |
| sequential | 3238405.9ns | 3235360.4ns | 100.1% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 3389252.0-5491035.4 ns)
  3389252.0 |
  3494341.2 |##########
  3599430.4 |
  3704519.5 |
  3809608.7 |##########
  3914697.9 |##########
  4019787.0 |
  4124876.2 |####################
  4229965.4 |
  4335054.5 |##########
  4440143.7 |
  4545232.9 |########################################
  4650322.0 |
  4755411.2 |##########
  4860500.4 |
  4965589.5 |
  5070678.7 |##########
  5175767.9 |##########
  5280857.0 |##########
  5385946.2 |##########
  (1 below, 2 above range)

per_core (n=18, range 1716700.9-5039888.5 ns)
  1716700.9 |##########
  1882860.3 |##########
  2049019.6 |##########
  2215179.0 |##########
  2381338.4 |
  2547497.8 |##########
  2713657.2 |
  2879816.6 |
  3045975.9 |
  3212135.3 |
  3378294.7 |
  3544454.1 |##########
  3710613.5 |
  3876772.9 |##########
  4042932.2 |
  4209091.6 |
  4375251.0 |########################################
  4541410.4 |####################
  4707569.8 |##########
  4873729.2 |
  (1 below, 3 above range)

sequential (n=18, range 3207470.0-3262282.1 ns)
  3207470.0 |
  3210210.6 |
  3212951.2 |#############
  3215691.8 |########################################
  3218432.4 |
  3221173.0 |#############
  3223913.6 |#############
  3226654.2 |#############
  3229394.8 |#############
  3232135.4 |
  3234876.0 |
  3237616.6 |#############
  3240357.2 |#############
  3243097.8 |##########################
  3245838.4 |
  3248579.1 |
  3251319.7 |#############
  3254060.3 |
  3256800.9 |
  3259541.5 |
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=100.2% of algo (FFI overhead may distort results)
- **per_core**: CV=35.1% (high variance, measurements may be unstable)
- **per_core**: bridge=87.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
