# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 21% faster than the next best (evs_engine)

evs_std (29.27 us) leads evs_engine (35.50 us) by 21%, a clear separation rather than a photo finish. CV 1.7%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (29.27 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 29268.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.21x (fastest 29268.3 ns, slowest 35502.1 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 35717ns | 35668ns | 35485ns | 35550ns | 35901ns | +21.55% |
| evs_std | 29385ns | 29404ns | 28772ns | 28957ns | 29861ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 35555ns | 35331ns | 35738ns | +21.47% | 1.843 |
| evs_std | 29270ns | 28676ns | 29748ns | base | 2.239 |

## Performance model

- Peak throughput: 2.285 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.846 | 80.8% |
| evs_std | 2.239 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 35717ns | 35717ns | +21.55% |
| evs_std | 29385ns | 29385ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 29268ns | base | --- | [28764, 29706] | --- | --- | --- | --- |
| evs_engine | 35502ns | +6274.8ns (+21.4%) | [+5650, +6883]ns | [35349, 35760] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 28805ns | +23.3% |
| 1 | 2 | 29694ns | +19.5% |

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
| evs_engine | 246050.8ns | 35554.7ns | 692.0% | HIGH |
| evs_std | 244362.0ns | 29269.9ns | 834.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 35330.9-35738.4 ns)
  35330.9 |########################################
  35351.3 |
  35371.6 |
  35392.0 |########################################
  35412.4 |
  35432.8 |
  35453.1 |
  35473.5 |
  35493.9 |
  35514.3 |
  35534.6 |
  35555.0 |
  35575.4 |
  35595.8 |####################
  35616.1 |
  35636.5 |####################
  35656.9 |
  35677.3 |
  35697.6 |
  35718.0 |
  (0 below, 2 above range)

evs_std (n=8, range 28675.9-29748.4 ns)
  28675.9 |####################
  28729.5 |####################
  28783.2 |
  28836.8 |########################################
  28890.4 |
  28944.0 |
  28997.7 |
  29051.3 |
  29104.9 |
  29158.5 |
  29212.2 |
  29265.8 |
  29319.4 |
  29373.0 |
  29426.7 |
  29480.3 |
  29533.9 |
  29587.5 |
  29641.2 |########################################
  29694.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=691.9% of algo (FFI overhead may distort results)
- evs_std: bridge=832.8% of algo (FFI overhead may distort results)
