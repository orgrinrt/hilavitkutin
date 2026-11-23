# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 52% faster than the next best (grouped)

per_core (111.23 us) leads grouped (168.91 us) by 52%, a clear separation rather than a photo finish. CV 77.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core beats baseline by 58% (significant)

per_core is -97.43 us (58%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (67.01 us) is smaller than the fastest variant's own run-to-run std-dev (85.87 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### grouped is inconsistent: worst-20% is 5.5x its best-20%

grouped's best 20% of batches run at 82.60 us but its worst 20% at 451.77 us (5.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 111228.4 ns median (-34.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.60x (fastest 111228.4 ns, slowest 178239.8 ns)

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
| grouped | 254866ns | 169408ns | 82853ns | 179043ns | 454606ns | base |
| per_core | 132919ns | 111767ns | 51263ns | 94434ns | 231474ns | -47.85% |
| sequential | 199595ns | 178924ns | 176320ns | 178378ns | 243057ns | -21.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 253547ns | 82602ns | 451773ns | base | 0.258 |
| per_core | 131899ns | 51068ns | 229106ns | -47.98% | 0.497 |
| sequential | 198497ns | 175824ns | 240878ns | -21.71% | 0.330 |

## Performance model

- Peak throughput: **1.283 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.388 | 30.2% |
| per_core | 0.589 | 45.9% |
| sequential | 0.368 | 28.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 254866ns | 254866ns | base |
| per_core | 132919ns | 132919ns | -47.85% |
| sequential | 199595ns | 199595ns | -21.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 168907ns | base | --- | [100174, 369919] | --- | --- | --- | --- |
| per_core | 111228ns | -97428.6ns (-57.7%) | [-245825, -42273]ns | [76367, 149214] | YES | 0.0026 | 0.0013 | 0 |
| sequential | 178240ns | no significant difference | [-180068, +77010]ns | [176970, 205383] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 256239ns | -53.8% | -30.6% |
| 1 | 2 | 137372ns | -5.0% | +29.7% |
| 1 | 3 | 269353ns | -72.9% | -21.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.661 | HIGH- (thermal bounce) |
| per_core | -0.292 | moderate- |
| sequential | -0.160 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 363595.6ns | 253546.8ns | 143.4% | HIGH |
| per_core | 134578.2ns | 131899.3ns | 102.0% | HIGH |
| sequential | 200318.7ns | 198497.1ns | 100.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 82602.4-451773.5 ns)
  82602.4 |########################################
  101061.0 |####################
  119519.5 |
  137978.1 |
  156436.6 |##############################
  174895.2 |
  193353.7 |
  211812.3 |
  230270.8 |
  248729.4 |
  267187.9 |
  285646.5 |
  304105.1 |
  322563.6 |
  341022.2 |####################
  359480.7 |####################
  377939.3 |
  396397.8 |
  414856.4 |
  433314.9 |
  (1 below, 4 above range)

per_core (n=18, range 51068.3-229105.8 ns)
  51068.3 |
  59970.1 |########################################
  68872.0 |########################################
  77773.9 |########################################
  86675.8 |
  95577.7 |
  104479.5 |####################
  113381.4 |########################################
  122283.3 |####################
  131185.2 |
  140087.0 |####################
  148988.9 |####################
  157890.8 |
  166792.7 |####################
  175694.6 |
  184596.4 |
  193498.3 |
  202400.2 |
  211302.1 |
  220203.9 |
  (2 below, 3 above range)

sequential (n=18, range 175823.6-240878.3 ns)
  175823.6 |########################################
  179076.3 |
  182329.0 |
  185581.8 |
  188834.5 |
  192087.3 |
  195340.0 |
  198592.7 |
  201845.5 |###
  205098.2 |
  208350.9 |
  211603.7 |
  214856.4 |
  218109.2 |###
  221361.9 |
  224614.6 |
  227867.4 |
  231120.1 |###
  234372.8 |
  237625.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=64.4% (high variance, measurements may be unstable)
- **grouped**: worst_20/best_20 = 5.5x (possible bimodal distribution)
- **grouped**: bridge=145.9% of algo (FFI overhead may distort results)
- **per_core**: CV=65.1% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **per_core**: bridge=99.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
