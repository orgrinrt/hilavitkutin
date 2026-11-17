# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (105.00 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 104995.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 104995.9 ns, slowest 107275.0 ns)

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
| evs_engine | 108469ns | 108321ns | 107875ns | 108100ns | 108894ns | +1.68% |
| evs_std | 106673ns | 106329ns | 104933ns | 105947ns | 107652ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 107299ns | 107058ns | 107440ns | +1.78% | 9.772 |
| evs_std | 105419ns | 104225ns | 106254ns | base | 9.947 |

## Performance model

- Peak throughput: 10.061 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 9.775 | 97.2% |
| evs_std | 9.987 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 108469ns | 108469ns | +1.68% |
| evs_std | 106673ns | 106673ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 104996ns | base | --- | [104242, 105950] | --- | --- | --- | --- |
| evs_engine | 107275ns | +2304.2ns (+2.2%) | [+1325, +3150]ns | [107100, 107533] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 104642ns | +2.7% |
| 1 | 2 | 105450ns | +1.7% |

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
| evs_engine | 5655203.1ns | 107299.0ns | 5270.5% | HIGH |
| evs_std | 5656602.1ns | 105418.8ns | 5365.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 107058.3-107439.6 ns)
  107058.3 |####################
  107077.4 |
  107096.4 |####################
  107115.5 |
  107134.6 |
  107153.6 |
  107172.7 |
  107191.7 |####################
  107210.8 |
  107229.9 |
  107248.9 |
  107268.0 |########################################
  107287.1 |
  107306.1 |
  107325.2 |
  107344.3 |
  107363.3 |
  107382.4 |####################
  107401.4 |
  107420.5 |
  (0 below, 2 above range)

evs_std (n=8, range 104225.0-106254.2 ns)
  104225.0 |########################################
  104326.5 |
  104427.9 |
  104529.4 |
  104630.8 |
  104732.3 |
  104833.8 |####################
  104935.2 |####################
  105036.7 |####################
  105138.1 |####################
  105239.6 |
  105341.0 |
  105442.5 |
  105544.0 |
  105645.4 |
  105746.9 |
  105848.3 |
  105949.8 |####################
  106051.3 |
  106152.7 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=5264.1% of algo (FFI overhead may distort results)
- evs_std: bridge=5382.0% of algo (FFI overhead may distort results)
