# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine shows alternating (throttle bounce) (autocorr -0.50)

evs_engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (370.27 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 370267.2 ns median
- Spread: 1.07x (fastest 370267.2 ns, slowest 396884.5 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 423798ns | 397457ns | 383876ns | 385357ns | 462609ns | +12.53% |
| evs_std | 376612ns | 370775ns | 363427ns | 365806ns | 388012ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 423183ns | 383328ns | 461997ns | +12.54% | 2.478 |
| evs_std | 376042ns | 362684ns | 387527ns | base | 2.788 |

## Performance model

- Peak throughput: 2.891 Gops/s (evs_std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.642 | 91.4% |
| evs_std | 2.832 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 423798ns | 423798ns | +12.53% |
| evs_std | 376612ns | 376612ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 370267ns | base | --- | [364042, 389081] | --- | --- | --- | --- |
| evs_engine | 396884ns | no significant difference | [-5334, +142391]ns | [383689, 509046] | no | 0.2891 | 0.2891 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 364442ns | +27.6% |
| 1 | 2 | 381642ns | +0.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 4140999.4ns | 423183.1ns | 978.5% | HIGH |
| evs_std | 4095890.0ns | 376041.8ns | 1089.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 383328.0-461997.4 ns)
  383328.0 |########################################
  387261.5 |
  391194.9 |
  395128.4 |
  399061.9 |
  402995.3 |
  406928.8 |##########
  410862.3 |
  414795.8 |
  418729.2 |##########
  422662.7 |
  426596.2 |
  430529.6 |
  434463.1 |
  438396.6 |
  442330.0 |
  446263.5 |
  450197.0 |
  454130.4 |
  458063.9 |
  (0 below, 2 above range)

evs_std (n=8, range 362684.0-387527.5 ns)
  362684.0 |####################
  363926.2 |########################################
  365168.3 |
  366410.5 |####################
  367652.7 |
  368894.9 |
  370137.0 |
  371379.2 |
  372621.4 |
  373863.6 |########################################
  375105.7 |
  376347.9 |
  377590.1 |
  378832.2 |
  380074.4 |
  381316.6 |
  382558.8 |
  383800.9 |
  385043.1 |
  386285.3 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=1042.8% of algo (FFI overhead may distort results)
- evs_std: bridge=1106.1% of algo (FFI overhead may distort results)
