# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 30% faster than the next best (grouped)

per_core (84.82 us) leads grouped (109.85 us) by 30%, a clear separation rather than a photo finish. CV 27.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.2x slower than the field

sequential (182.53 us) is 2.2x the fastest (84.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped shows alternating (throttle bounce) (autocorr -0.56)

grouped's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 4.5x its best-20%

grouped's best 20% of batches run at 47.35 us but its worst 20% at 215.26 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 84818.7 ns median (-22.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.15x (fastest 84818.7 ns, slowest 182531.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112755559 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 126196ns | 110477ns | 47609ns | 92228ns | 216442ns | base |
| per_core | 88657ns | 85052ns | 53899ns | 81381ns | 116951ns | -29.75% |
| sequential | 202085ns | 183075ns | 175996ns | 182888ns | 243926ns | +60.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 125377ns | 47348ns | 215261ns | base | 0.523 |
| per_core | 88065ns | 53676ns | 115748ns | -29.76% | 0.744 |
| sequential | 201194ns | 175690ns | 242188ns | +60.47% | 0.326 |

## Performance model

- Peak throughput: **1.384 Gops/s** (grouped; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.597 | 43.1% |
| per_core | 0.773 | 55.8% |
| sequential | 0.359 | 25.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 126196ns | 126196ns | base |
| per_core | 88657ns | 88657ns | -29.75% |
| sequential | 202085ns | 202085ns | +60.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 109853ns | base | --- | [59577, 139229] | --- | --- | --- | --- |
| per_core | 84819ns | no significant difference | [-30883, +8620]ns | [73322, 107837] | no | 0.8145 | 0.8145 | 0 |
| sequential | 182531ns | +116537.6ns (+106.1%) | [+63093, +129365]ns | [176920, 196742] | YES | 0.0026 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 94082ns | +1.8% | +105.6% |
| 1 | 2 | 161996ns | -48.1% | +16.8% |
| 1 | 3 | 47692ns | +65.7% | +275.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.558 | HIGH- (thermal bounce) |
| per_core | -0.033 | ok |
| sequential | -0.037 | ok |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 139217.8ns | 125376.9ns | 111.0% | HIGH |
| per_core | 87934.8ns | 88064.8ns | 99.9% | HIGH |
| sequential | 205947.1ns | 201193.7ns | 102.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 47347.9-215260.5 ns)
  47347.9 |########################################
  55743.6 |
  64139.2 |##########
  72534.8 |##########
  80930.5 |
  89326.1 |
  97721.7 |
  106117.3 |##############################
  114513.0 |##########
  122908.6 |##########
  131304.2 |####################
  139699.9 |
  148095.5 |##########
  156491.1 |
  164886.7 |##########
  173282.4 |
  181678.0 |
  190073.6 |
  198469.3 |
  206864.9 |
  (1 below, 2 above range)

per_core (n=18, range 53675.9-115748.0 ns)
  53675.9 |####################
  56779.5 |####################
  59883.1 |####################
  62986.7 |
  66090.3 |
  69193.9 |####################
  72297.5 |
  75401.1 |####################
  78504.7 |####################
  81608.3 |####################
  84711.9 |########################################
  87815.5 |####################
  90919.1 |
  94022.7 |
  97126.3 |
  100229.9 |####################
  103333.6 |
  106437.2 |########################################
  109540.8 |
  112644.4 |########################################
  (1 below, 2 above range)

sequential (n=18, range 175690.4-242187.5 ns)
  175690.4 |########################################
  179015.3 |#############
  182340.1 |######
  185665.0 |
  188989.8 |
  192314.7 |####################
  195639.6 |
  198964.4 |#############
  202289.3 |
  205614.1 |
  208939.0 |
  212263.8 |
  215588.7 |
  218913.5 |
  222238.4 |
  225563.2 |
  228888.1 |
  232212.9 |
  235537.8 |
  238862.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=72.9% (high variance, measurements may be unstable)
- **grouped**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **grouped**: bridge=113.6% of algo (FFI overhead may distort results)
- **per_core**: CV=26.8% (high variance, measurements may be unstable)
- **per_core**: bridge=98.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.7% of algo (FFI overhead may distort results)
