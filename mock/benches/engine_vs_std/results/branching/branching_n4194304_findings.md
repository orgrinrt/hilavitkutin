# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 31% faster than the next best (evs_engine)

evs_std (5.74 ms) leads evs_engine (7.50 ms) by 31%, a clear separation rather than a photo finish. CV 3.0%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (5.74 ms median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 5737462.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.31x (fastest 5737462.1 ns, slowest 7504154.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, two transforms joined by a third |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000002 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 7583612ns | 7510053ns | 7260031ns | 7458408ns | 7758410ns | +31.98% |
| evs_std | 5746180ns | 5742324ns | 5548112ns | 5621643ns | 5889100ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 7578239ns | 7255449ns | 7752792ns | +32.00% | 0.553 |
| evs_std | 5741220ns | 5542478ns | 5884261ns | base | 0.731 |

## Performance model

- Peak throughput: 0.757 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.559 | 73.9% |
| evs_std | 0.731 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 7583612ns | 7583612ns | +31.98% |
| evs_std | 5746180ns | 5746180ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 5737462ns | base | --- | [5572609, 5843540] | --- | --- | --- | --- |
| evs_engine | 7504154ns | +1755367.5ns (+30.6%) | [+1657040, +2196114]ns | [7377358, 7791421] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 5585509ns | +33.2% |
| 1 | 2 | 5827542ns | +31.3% |

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
| evs_engine | 24236123.7ns | 7578239.3ns | 319.8% | HIGH |
| evs_std | 22358970.1ns | 5741219.5ns | 389.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 7255449.3-7752792.0 ns)
  7255449.3 |####################
  7280316.4 |
  7305183.6 |
  7330050.7 |
  7354917.8 |####################
  7379785.0 |
  7404652.1 |
  7429519.2 |
  7454386.4 |
  7479253.5 |########################################
  7504120.6 |####################
  7528987.8 |
  7553854.9 |
  7578722.0 |
  7603589.2 |
  7628456.3 |
  7653323.4 |
  7678190.6 |
  7703057.7 |
  7727924.8 |####################
  (0 below, 2 above range)

evs_std (n=8, range 5542478.0-5884260.7 ns)
  5542478.0 |########################################
  5559567.1 |########################################
  5576656.3 |
  5593745.4 |########################################
  5610834.5 |
  5627923.7 |
  5645012.8 |
  5662101.9 |
  5679191.1 |########################################
  5696280.2 |
  5713369.3 |
  5730458.5 |
  5747547.6 |
  5764636.8 |
  5781725.9 |########################################
  5798815.0 |########################################
  5815904.2 |
  5832993.3 |########################################
  5850082.4 |
  5867171.6 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=322.1% of algo (FFI overhead may distort results)
- evs_std: bridge=388.9% of algo (FFI overhead may distort results)
