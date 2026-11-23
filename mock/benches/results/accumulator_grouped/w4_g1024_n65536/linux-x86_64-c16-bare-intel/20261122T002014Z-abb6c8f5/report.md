# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.4x slower than the field

sequential (178.47 us) is 2.4x the fastest (74.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core is fastest but the noisiest (CV 68.5%)

per_core wins on median (74.39 us) yet has the highest variance (CV 68.5%), while sequential is the steadiest (CV 11.9%, 178.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (per_core, grouped) are a dead heat (<1%)

per_core (74.39 us) and grouped (74.87 us) differ by 0.65%, inside the noise, even though the wider field spreads 139.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### grouped is inconsistent: worst-20% is 3.3x its best-20%

grouped's best 20% of batches run at 44.12 us but its worst 20% at 146.47 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 74385.6 ns median (-0.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.40x (fastest 74385.6 ns, slowest 178466.6 ns)

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
| grouped | 92980ns | 75201ns | 44298ns | 72604ns | 147884ns | base |
| per_core | 90011ns | 74660ns | 44716ns | 66673ns | 147667ns | -3.19% |
| sequential | 187770ns | 178851ns | 176580ns | 178519ns | 207241ns | +101.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 92332ns | 44123ns | 146467ns | base | 0.710 |
| per_core | 89380ns | 44514ns | 146230ns | -3.20% | 0.733 |
| sequential | 187018ns | 176292ns | 205804ns | +102.55% | 0.350 |

## Performance model

- Peak throughput: **1.485 Gops/s** (grouped; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.875 | 58.9% |
| per_core | 0.881 | 59.3% |
| sequential | 0.367 | 24.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 92980ns | 92980ns | base |
| per_core | 90011ns | 90011ns | -3.19% |
| sequential | 187770ns | 187770ns | +101.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 74868ns | base | --- | [65077, 112138] | --- | --- | --- | --- |
| per_core | 74386ns | no significant difference | [-22818, +15738]ns | [55557, 99676] | no | 1.0000 | 1.0000 | 0 |
| sequential | 178467ns | +104808.1ns (+140.0%) | [+74958, +115847]ns | [177390, 180114] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 87577ns | -25.9% | +104.4% |
| 1 | 2 | 92225ns | -17.3% | +93.5% |
| 1 | 3 | 44564ns | +76.5% | +300.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.220 | moderate- |
| per_core | -0.084 | ok |
| sequential | -0.055 | ok |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 95379.2ns | 92332.0ns | 103.3% | HIGH |
| per_core | 93240.7ns | 89380.5ns | 104.3% | HIGH |
| sequential | 189814.5ns | 187017.5ns | 101.5% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 44122.9-146466.7 ns)
  44122.9 |##########################
  49240.1 |
  54357.3 |
  59474.4 |#############
  64591.6 |########################################
  69708.8 |#############
  74826.0 |#############
  79943.2 |#############
  85060.4 |
  90177.6 |
  95294.8 |
  100412.0 |
  105529.2 |##########################
  110646.4 |
  115763.6 |#############
  120880.8 |#############
  125997.9 |
  131115.1 |
  136232.3 |#############
  141349.5 |
  (2 below, 2 above range)

per_core (n=18, range 44513.5-146229.8 ns)
  44513.5 |########################################
  49599.3 |####################
  54685.1 |########################################
  59770.9 |
  64856.8 |
  69942.6 |########################################
  75028.4 |####################
  80114.2 |########################################
  85200.0 |
  90285.8 |
  95371.6 |####################
  100457.5 |####################
  105543.3 |
  110629.1 |####################
  115714.9 |
  120800.7 |
  125886.5 |
  130972.3 |
  136058.2 |####################
  141144.0 |
  (2 below, 2 above range)

sequential (n=18, range 176291.5-205803.9 ns)
  176291.5 |############################
  177767.2 |########################################
  179242.8 |
  180718.4 |#####
  182194.0 |
  183669.6 |#####
  185145.2 |
  186620.9 |
  188096.5 |
  189572.1 |
  191047.7 |
  192523.3 |
  193999.0 |
  195474.6 |
  196950.2 |
  198425.8 |
  199901.4 |
  201377.0 |
  202852.7 |
  204328.3 |#####
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: CV=52.8% (high variance, measurements may be unstable)
- **grouped**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **grouped**: bridge=101.2% of algo (FFI overhead may distort results)
- **per_core**: CV=57.0% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **per_core**: bridge=101.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.6% of algo (FFI overhead may distort results)
