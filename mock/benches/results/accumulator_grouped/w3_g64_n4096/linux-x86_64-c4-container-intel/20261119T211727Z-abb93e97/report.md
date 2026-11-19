# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 55% faster than the next best (grouped)

sequential (26.28 us) leads grouped (40.65 us) by 55%, a clear separation rather than a photo finish. CV 30.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 33% (significant)

sequential is -13.33 us (33%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is fastest but the noisiest (CV 30.3%)

sequential wins on median (26.28 us) yet has the highest variance (CV 30.3%), while grouped is the steadiest (CV 7.7%, 40.65 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential is inconsistent: worst-20% is 1.7x its best-20%

sequential's best 20% of batches run at 23.24 us but its worst 20% at 40.22 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 26278.8 ns median (-35.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.59x (fastest 26278.8 ns, slowest 41849.2 ns)

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
| grouped | 41564ns | 41107ns | 37043ns | 40626ns | 45233ns | base |
| per_core | 44444ns | 42282ns | 36630ns | 41878ns | 52201ns | +6.93% |
| sequential | 30836ns | 26704ns | 23880ns | 26454ns | 40886ns | -25.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 41002ns | 36623ns | 44600ns | base | 0.100 |
| per_core | 43808ns | 35938ns | 51469ns | +6.84% | 0.093 |
| sequential | 30262ns | 23236ns | 40216ns | -26.19% | 0.135 |

## Performance model

- Peak throughput: **0.176 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.101 | 57.2% |
| per_core | 0.098 | 55.5% |
| sequential | 0.156 | 88.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 41564ns | 41564ns | base |
| per_core | 44444ns | 44444ns | +6.93% |
| sequential | 30836ns | 30836ns | -25.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 40653ns | base | --- | [39181, 42256] | --- | --- | --- | --- |
| per_core | 41849ns | no significant difference | [-2109, +5991]ns | [39363, 48205] | no | 0.8145 | 0.8145 | 0 |
| sequential | 26279ns | -13330.2ns (-32.8%) | [-14931, -9022]ns | [24929, 35153] | YES | 0.0026 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 42256ns | +16.7% | -12.6% |
| 1 | 2 | 40448ns | -1.2% | -35.0% |
| 1 | 3 | 38030ns | +4.6% | -36.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.005 | ok |
| per_core | -0.157 | ok |
| sequential | -0.082 | ok |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 41620.1ns | 41002.4ns | 101.5% | HIGH |
| per_core | 44034.8ns | 43808.0ns | 100.5% | HIGH |
| sequential | 30659.9ns | 30262.0ns | 101.3% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 36623.3-44599.5 ns)
  36623.3 |
  37022.1 |
  37420.9 |####################
  37819.7 |
  38218.5 |####################
  38617.3 |####################
  39016.1 |
  39415.0 |########################################
  39813.8 |####################
  40212.6 |####################
  40611.4 |####################
  41010.2 |####################
  41409.0 |########################################
  41807.8 |
  42206.7 |
  42605.5 |####################
  43004.3 |
  43403.1 |
  43801.9 |####################
  44200.7 |
  (2 below, 3 above range)

per_core (n=18, range 35938.3-51469.3 ns)
  35938.3 |########################################
  36714.9 |####################
  37491.4 |
  38268.0 |
  39044.5 |########################################
  39821.1 |########################################
  40597.6 |####################
  41374.2 |
  42150.7 |####################
  42927.3 |
  43703.8 |####################
  44480.3 |
  45256.9 |
  46033.4 |
  46810.0 |####################
  47586.5 |####################
  48363.1 |####################
  49139.6 |
  49916.2 |####################
  50692.7 |####################
  (1 below, 2 above range)

sequential (n=18, range 23235.6-40216.4 ns)
  23235.6 |##########################
  24084.6 |##########################
  24933.6 |##########################
  25782.7 |########################################
  26631.7 |
  27480.8 |##########################
  28329.8 |
  29178.8 |
  30027.9 |
  30876.9 |
  31726.0 |
  32575.0 |
  33424.0 |
  34273.1 |
  35122.1 |#############
  35971.2 |##########################
  36820.2 |#############
  37669.2 |
  38518.3 |
  39367.3 |
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=103.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=102.2% of algo (FFI overhead may distort results)
- **sequential**: CV=26.3% (high variance, measurements may be unstable)
- **sequential**: bridge=103.7% of algo (FFI overhead may distort results)
