# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 260% faster than the next best (evs_engine)

evs_std (7.75 us) leads evs_engine (27.90 us) by 260%, a clear separation rather than a photo finish. CV 0.2%.


### evs_engine shows alternating (throttle bounce) (autocorr -0.50)

evs_engine's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (7.75 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 3.6x the fastest

Fastest evs_std (7.75 us) to slowest evs_engine (27.90 us): 3.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 7745.9 ns median
- 1 variant significantly slower than baseline
- Spread: 3.60x (fastest 7745.9 ns, slowest 27900.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 28358ns | 28042ns | 27967ns | 28008ns | 28719ns | +245.96% |
| evs_std | 8197ns | 8138ns | 7842ns | 7864ns | 8535ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 27967ns | 27775ns | 28079ns | +261.01% | 2.343 |
| evs_std | 7747ns | 7733ns | 7756ns | base | 8.460 |

## Performance model

- Peak throughput: 8.475 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 2.349 | 27.7% |
| evs_std | 8.461 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 28358ns | 28358ns | +245.96% |
| evs_std | 8197ns | 8197ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 7746ns | base | --- | [7733, 7750] | --- | --- | --- | --- |
| evs_engine | 27900ns | +20154.2ns (+260.2%) | [+20100, +20383]ns | [27875, 28117] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 7746ns | +261.5% |
| 1 | 2 | 7746ns | +260.2% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | 0.000 | ok |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 354033.3ns | 27966.7ns | 1265.9% | HIGH |
| evs_std | 353096.9ns | 7746.9ns | 4557.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 27775.0-28079.2 ns)
  27775.0 |####################
  27790.2 |
  27805.4 |
  27820.6 |
  27835.8 |
  27851.0 |
  27866.3 |####################
  27881.5 |########################################
  27896.7 |
  27911.9 |####################
  27927.1 |####################
  27942.3 |
  27957.5 |
  27972.7 |
  27987.9 |
  28003.2 |
  28018.4 |
  28033.6 |
  28048.8 |
  28064.0 |
  (0 below, 2 above range)

evs_std (n=8, range 7733.3-7756.2 ns)
   7733.3 |##########################
   7734.4 |
   7735.6 |
   7736.7 |
   7737.9 |
   7739.0 |
   7740.2 |
   7741.3 |##########################
   7742.5 |
   7743.6 |
   7744.8 |
   7745.9 |
   7747.1 |
   7748.2 |
   7749.4 |########################################
   7750.5 |
   7751.7 |
   7752.8 |
   7754.0 |
   7755.1 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=1264.7% of algo (FFI overhead may distort results)
- evs_std: bridge=4554.1% of algo (FFI overhead may distort results)
