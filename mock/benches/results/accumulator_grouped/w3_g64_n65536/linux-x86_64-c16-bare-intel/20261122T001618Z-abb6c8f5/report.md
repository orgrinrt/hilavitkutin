# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core shows alternating (throttle bounce) (autocorr -0.66)

per_core's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 4.6x its best-20%

grouped's best 20% of batches run at 67.98 us but its worst 20% at 312.43 us (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 101059.3 ns median (-12.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.76x (fastest 101059.3 ns, slowest 177928.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 3, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112788431 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 169153ns | 116353ns | 68306ns | 106055ns | 314224ns | base |
| per_core | 125525ns | 101406ns | 66928ns | 91743ns | 205497ns | -25.79% |
| sequential | 188225ns | 178553ns | 176175ns | 178008ns | 209576ns | +11.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 168311ns | 67976ns | 312433ns | base | 0.389 |
| per_core | 124619ns | 66717ns | 203398ns | -25.96% | 0.526 |
| sequential | 187460ns | 175710ns | 208291ns | +11.38% | 0.350 |

## Performance model

- Peak throughput: **0.982 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.565 | 57.5% |
| per_core | 0.648 | 66.0% |
| sequential | 0.368 | 37.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 169153ns | 169153ns | base |
| per_core | 125525ns | 125525ns | -25.79% |
| sequential | 188225ns | 188225ns | +11.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 116050ns | base | --- | [79401, 202628] | --- | --- | --- | --- |
| per_core | 101059ns | no significant difference | [-44907, +21452]ns | [68816, 170712] | no | 0.2379 | 0.2379 | 0 |
| sequential | 177928ns | no significant difference | [-411, +97032]ns | [176272, 180595] | no | 0.1925 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 83214ns | -6.7% | +115.0% |
| 1 | 2 | 88422ns | +93.1% | +99.4% |
| 1 | 3 | 202628ns | -65.5% | -12.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.145 | ok |
| per_core | -0.663 | HIGH- (thermal bounce) |
| sequential | -0.643 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 191403.4ns | 168311.1ns | 113.7% | HIGH |
| per_core | 126528.0ns | 124618.8ns | 101.5% | HIGH |
| sequential | 190151.6ns | 187459.6ns | 101.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 67976.3-312433.4 ns)
  67976.3 |########################################
  80199.2 |##############################
  92422.0 |
  104644.9 |##########
  116867.7 |##########
  129090.6 |##########
  141313.5 |
  153536.3 |##########
  165759.2 |
  177982.0 |##########
  190204.9 |
  202427.7 |
  214650.6 |##########
  226873.4 |##########
  239096.3 |##########
  251319.1 |
  263542.0 |
  275764.8 |
  287987.7 |
  300210.5 |
  (1 below, 2 above range)

per_core (n=18, range 66716.7-203397.6 ns)
  66716.7 |########################################
  73550.8 |
  80384.8 |########
  87218.9 |
  94052.9 |########
  100887.0 |########
  107721.0 |
  114555.1 |
  121389.1 |########
  128223.1 |########
  135057.2 |
  141891.2 |########
  148725.3 |
  155559.3 |
  162393.4 |
  169227.4 |
  176061.5 |
  182895.5 |
  189729.6 |########
  196563.6 |
  (2 below, 4 above range)

sequential (n=18, range 175709.6-208291.1 ns)
  175709.6 |########################################
  177338.7 |###########
  178967.8 |#################
  180596.9 |#####
  182225.9 |
  183855.0 |
  185484.1 |#####
  187113.2 |
  188742.2 |
  190371.3 |
  192000.4 |
  193629.4 |
  195258.5 |
  196887.6 |
  198516.7 |
  200145.7 |
  201774.8 |
  203403.9 |
  205033.0 |
  206662.0 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=82.9% (high variance, measurements may be unstable)
- **grouped**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **grouped**: bridge=111.0% of algo (FFI overhead may distort results)
- **per_core**: CV=49.1% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **per_core**: bridge=103.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
