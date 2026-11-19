# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 22% faster than the next best (grouped)

per_core (171.78 us) leads grouped (209.94 us) by 22%, a clear separation rather than a photo finish. CV 11.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core beats baseline by 23% (significant)

per_core is -48.14 us (23%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.0x slower than the field

sequential (346.37 us) is 2.0x the fastest (171.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped shows alternating (throttle bounce) (autocorr -0.51)

grouped's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 2.0x its best-20%

grouped's best 20% of batches run at 164.10 us but its worst 20% at 327.98 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 171779.1 ns median (-18.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.02x (fastest 171779.1 ns, slowest 346370.1 ns)

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
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 237321ns | 210722ns | 164927ns | 200171ns | 329243ns | base |
| per_core | 175157ns | 172270ns | 145508ns | 170631ns | 196769ns | -26.19% |
| sequential | 348847ns | 347256ns | 302878ns | 338449ns | 387428ns | +46.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 236389ns | 164097ns | 327975ns | base | 0.277 |
| per_core | 174465ns | 145035ns | 195966ns | -26.20% | 0.376 |
| sequential | 347558ns | 302000ns | 385775ns | +47.03% | 0.189 |

## Performance model

- Peak throughput: **0.452 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.312 | 69.1% |
| per_core | 0.382 | 84.4% |
| sequential | 0.189 | 41.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 237321ns | 237321ns | base |
| per_core | 175157ns | 175157ns | -26.19% |
| sequential | 348847ns | 348847ns | +46.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 209939ns | base | --- | [178733, 235248] | --- | --- | --- | --- |
| per_core | 171779ns | -48135.5ns (-22.9%) | [-65045, -4321]ns | [165054, 188542] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 346370ns | +147844.3ns (+70.4%) | [+89220, +175303]ns | [315631, 374719] | YES | 0.0026 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 209324ns | -14.7% | +46.3% |
| 1 | 2 | 242802ns | -22.5% | +49.1% |
| 1 | 3 | 178733ns | -6.8% | +106.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.515 | HIGH- (thermal bounce) |
| per_core | -0.461 | moderate- |
| sequential | -0.117 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 356660.2ns | 236389.3ns | 150.9% | HIGH |
| per_core | 183110.9ns | 174465.1ns | 105.0% | HIGH |
| sequential | 378562.1ns | 347558.3ns | 108.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 164096.7-327975.4 ns)
  164096.7 |##########################
  172290.7 |#############
  180484.6 |#############
  188678.5 |#############
  196872.5 |
  205066.4 |########################################
  213260.3 |
  221454.3 |##########################
  229648.2 |#############
  237842.1 |##########################
  246036.1 |
  254230.0 |
  262423.9 |
  270617.9 |
  278811.8 |
  287005.7 |
  295199.7 |
  303393.6 |
  311587.5 |
  319781.5 |
  (2 below, 3 above range)

per_core (n=18, range 145035.3-195965.8 ns)
  145035.3 |
  147581.8 |
  150128.3 |
  152674.8 |
  155221.4 |##########################
  157767.9 |
  160314.4 |
  162860.9 |#############
  165407.5 |########################################
  167954.0 |#############
  170500.5 |
  173047.1 |#############
  175593.6 |#############
  178140.1 |
  180686.6 |
  183233.2 |
  185779.7 |#############
  188326.2 |########################################
  190872.7 |#############
  193419.3 |
  (2 below, 2 above range)

sequential (n=18, range 302000.2-385775.1 ns)
  302000.2 |##########################
  306189.0 |##########################
  310377.7 |
  314566.5 |
  318755.2 |#############
  322943.9 |
  327132.7 |
  331321.4 |#############
  335510.2 |#############
  339698.9 |
  343887.7 |##########################
  348076.4 |
  352265.1 |
  356453.9 |
  360642.6 |#############
  364831.4 |
  369020.1 |#############
  373208.8 |########################################
  377397.6 |
  381586.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=37.1% (high variance, measurements may be unstable)
- **grouped**: bridge=125.6% of algo (FFI overhead may distort results)
- **per_core**: bridge=101.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.1% of algo (FFI overhead may distort results)
