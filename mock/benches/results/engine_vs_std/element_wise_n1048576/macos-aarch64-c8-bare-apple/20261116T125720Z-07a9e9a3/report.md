# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (105.75 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 105750.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 105750.0 ns, slowest 107841.6 ns)

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
| evs_engine | 109938ns | 109800ns | 108733ns | 109464ns | 110594ns | +2.14% |
| evs_std | 107634ns | 107571ns | 107217ns | 107331ns | 107967ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 108137ns | 107517ns | 108733ns | +2.11% | 9.697 |
| evs_std | 105905ns | 105458ns | 106237ns | base | 9.901 |

## Performance model

- Peak throughput: 9.943 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 9.723 | 97.8% |
| evs_std | 9.916 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 109938ns | 109938ns | +2.14% |
| evs_std | 107634ns | 107634ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 105750ns | base | --- | [105517, 106475] | --- | --- | --- | --- |
| evs_engine | 107842ns | +2125.0ns (+2.0%) | [+1792, +2983]ns | [107533, 109017] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 105833ns | +1.9% |
| 1 | 2 | 105546ns | +2.4% |

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
| evs_engine | 5645375.0ns | 108137.5ns | 5220.6% | HIGH |
| evs_std | 5645500.0ns | 105905.2ns | 5330.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 107516.7-108733.3 ns)
  107516.7 |########################################
  107577.5 |#############
  107638.4 |
  107699.2 |
  107760.0 |
  107820.9 |
  107881.7 |
  107942.5 |
  108003.3 |
  108064.2 |#############
  108125.0 |
  108185.8 |
  108246.7 |
  108307.5 |
  108368.3 |
  108429.2 |
  108490.0 |
  108550.8 |#############
  108611.7 |
  108672.5 |
  (0 below, 2 above range)

evs_std (n=8, range 105458.3-106237.5 ns)
  105458.3 |####################
  105497.3 |####################
  105536.2 |####################
  105575.2 |
  105614.1 |
  105653.1 |
  105692.1 |
  105731.0 |########################################
  105770.0 |
  105808.9 |
  105847.9 |
  105886.8 |####################
  105925.8 |
  105964.8 |
  106003.7 |
  106042.7 |
  106081.6 |
  106120.6 |
  106159.6 |
  106198.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=5234.4% of algo (FFI overhead may distort results)
- evs_std: bridge=5337.4% of algo (FFI overhead may distort results)
