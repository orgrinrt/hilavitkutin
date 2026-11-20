# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 20% faster than the next best (grouped)

per_core (110.84 us) leads grouped (132.55 us) by 20%, a clear separation rather than a photo finish. CV 18.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.7x slower than the field

sequential (299.27 us) is 2.7x the fastest (110.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core is fastest but the noisiest (CV 18.0%)

per_core wins on median (110.84 us) yet has the highest variance (CV 18.0%), while sequential is the steadiest (CV 7.5%, 299.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: per_core** at 110843.4 ns median (-16.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.70x (fastest 110843.4 ns, slowest 299272.6 ns)

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
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 129885ns | 133173ns | 108100ns | 128489ns | 142871ns | base |
| per_core | 117577ns | 111313ns | 95896ns | 108262ns | 142390ns | -9.48% |
| sequential | 303146ns | 300568ns | 272685ns | 296016ns | 329072ns | +133.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 129190ns | 107646ns | 142059ns | base | 0.507 |
| per_core | 116910ns | 95068ns | 141655ns | -9.50% | 0.561 |
| sequential | 301883ns | 271570ns | 327436ns | +133.67% | 0.217 |

## Performance model

- Peak throughput: **0.689 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.494 | 71.7% |
| per_core | 0.591 | 85.8% |
| sequential | 0.219 | 31.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 129885ns | 129885ns | base |
| per_core | 117577ns | 117577ns | -9.48% |
| sequential | 303146ns | 303146ns | +133.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 132553ns | base | --- | [120160, 139424] | --- | --- | --- | --- |
| per_core | 110843ns | no significant difference | [-28924, +2552]ns | [99549, 131850] | no | 0.2379 | 0.2379 | 0 |
| sequential | 299273ns | +169591.9ns (+127.9%) | [+158347, +195660]ns | [288711, 310343] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 139424ns | +2.6% | +131.9% |
| 1 | 2 | 133539ns | -25.3% | +117.0% |
| 1 | 3 | 110327ns | -5.9% | +167.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.070 | ok |
| per_core | -0.219 | moderate- |
| sequential | -0.256 | moderate- |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 135868.3ns | 129189.5ns | 105.2% | HIGH |
| per_core | 124441.2ns | 116910.4ns | 106.4% | HIGH |
| sequential | 300681.9ns | 301883.5ns | 99.6% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 107646.3-142059.3 ns)
  107646.3 |####################
  109367.0 |########################################
  111087.6 |
  112808.3 |
  114528.9 |
  116249.6 |####################
  117970.2 |
  119690.9 |
  121411.5 |
  123132.2 |####################
  124852.8 |
  126573.5 |
  128294.1 |####################
  130014.8 |########################################
  131735.4 |
  133456.1 |####################
  135176.7 |####################
  136897.4 |####################
  138618.0 |########################################
  140338.7 |####################
  (1 below, 3 above range)

per_core (n=18, range 95067.8-141654.6 ns)
  95067.8 |########################################
  97397.1 |########################################
  99726.5 |########################################
  102055.8 |####################
  104385.2 |
  106714.5 |####################
  109043.8 |
  111373.2 |
  113702.5 |####################
  116031.9 |
  118361.2 |####################
  120690.5 |
  123019.9 |
  125349.2 |
  127678.6 |####################
  130007.9 |####################
  132337.3 |####################
  134666.6 |
  136995.9 |####################
  139325.3 |
  (1 below, 3 above range)

sequential (n=18, range 271570.4-327435.9 ns)
  271570.4 |
  274363.7 |#############
  277157.0 |
  279950.3 |#############
  282743.5 |
  285536.8 |
  288330.1 |########################################
  291123.3 |
  293916.6 |#############
  296709.9 |#############
  299503.2 |#############
  302296.4 |##########################
  305089.7 |
  307883.0 |#############
  310676.2 |#############
  313469.5 |#############
  316262.8 |
  319056.1 |
  321849.3 |
  324642.6 |
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=107.0% of algo (FFI overhead may distort results)
- **per_core**: bridge=104.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.6% of algo (FFI overhead may distort results)
