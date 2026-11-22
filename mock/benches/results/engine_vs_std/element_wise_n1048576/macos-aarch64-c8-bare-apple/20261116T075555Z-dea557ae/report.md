# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (105.39 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 105387.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 105387.5 ns, slowest 108029.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 109929ns | 109342ns | 108608ns | 109025ns | 110938ns | +2.51% |
| evs_std | 107232ns | 106950ns | 106350ns | 106753ns | 107812ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 108607ns | 107542ns | 109562ns | +2.59% | 9.655 |
| evs_std | 105869ns | 105125ns | 106506ns | base | 9.904 |

## Performance model

- Peak throughput: 9.975 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 9.706 | 97.3% |
| evs_std | 9.950 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 109929ns | 109929ns | +2.51% |
| evs_std | 107232ns | 107232ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 105388ns | base | --- | [105217, 106200] | --- | --- | --- | --- |
| evs_engine | 108029ns | +2775.0ns (+2.6%) | [+1508, +3750]ns | [107650, 109033] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 105979ns | +1.6% |
| 1 | 2 | 105250ns | +3.3% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 5660319.8ns | 108607.3ns | 5211.7% | HIGH |
| evs_std | 5668285.4ns | 105868.8ns | 5354.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 107541.7-109562.5 ns)
  107541.7 |#############
  107642.7 |########################################
  107743.8 |
  107844.8 |
  107945.9 |
  108046.9 |
  108147.9 |
  108249.0 |#############
  108350.0 |
  108451.1 |
  108552.1 |
  108653.1 |
  108754.2 |
  108855.2 |
  108956.3 |##########################
  109057.3 |
  109158.3 |
  109259.4 |
  109360.4 |
  109461.5 |
  (0 below, 1 above range)

evs_std (n=8, range 105125.0-106506.2 ns)
  105125.0 |####################
  105194.1 |####################
  105263.1 |########################################
  105332.2 |
  105401.2 |
  105470.3 |####################
  105539.4 |
  105608.4 |
  105677.5 |
  105746.6 |####################
  105815.6 |
  105884.7 |
  105953.8 |
  106022.8 |
  106091.9 |
  106160.9 |####################
  106230.0 |
  106299.1 |
  106368.1 |
  106437.2 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=5229.0% of algo (FFI overhead may distort results)
- evs_std: bridge=5361.5% of algo (FFI overhead may distort results)
