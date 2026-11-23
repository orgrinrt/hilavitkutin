# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 94% faster than the next best (evs_engine)

evs_std (2.31 ms) leads evs_engine (4.48 ms) by 94%, a clear separation rather than a photo finish. CV 11.4%.


### evs_std is fastest but the noisiest (CV 11.4%)

evs_std wins on median (2.31 ms) yet has the highest variance (CV 11.4%), while evs_engine is the steadiest (CV 5.7%, 4.48 ms).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (2.31 ms median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 2310659.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.94x (fastest 2310659.9 ns, slowest 4484305.5 ns)

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
| Counter frequency | 2112781463 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 4498486ns | 4485287ns | 4132805ns | 4349860ns | 4701377ns | +86.79% |
| evs_std | 2408345ns | 2311954ns | 2174980ns | 2222252ns | 2606256ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 4497462ns | 4131870ns | 4700344ns | +86.84% | 0.933 |
| evs_std | 2407110ns | 2173735ns | 2605008ns | base | 1.742 |

## Performance model

- Peak throughput: 1.930 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.935 | 48.5% |
| evs_std | 1.815 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 4498486ns | 4498486ns | +86.79% |
| evs_std | 2408345ns | 2408345ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2310660ns | base | --- | [2176563, 2699111] | --- | --- | --- | --- |
| evs_engine | 4484306ns | +2095590.3ns (+90.7%) | [+1853742, +2383517]ns | [4256472, 4875807] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 2560248ns | +67.3% |
| 1 | 2 | 2192490ns | +106.4% |

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
| evs_engine | 16833856.7ns | 4497462.2ns | 374.3% | HIGH |
| evs_std | 16841468.4ns | 2407109.5ns | 699.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 4131870.4-4700344.3 ns)
  4131870.4 |####################
  4160294.1 |
  4188717.8 |
  4217141.5 |
  4245565.2 |####################
  4273988.9 |
  4302412.6 |####################
  4330836.3 |
  4359260.0 |
  4387683.6 |
  4416107.3 |
  4444531.0 |
  4472954.7 |########################################
  4501378.4 |
  4529802.1 |
  4558225.8 |####################
  4586649.5 |
  4615073.2 |
  4643496.9 |
  4671920.6 |
  (0 below, 2 above range)

evs_std (n=8, range 2173735.3-2605008.4 ns)
  2173735.3 |########################################
  2195299.0 |####################
  2216862.6 |
  2238426.3 |
  2259989.9 |####################
  2281553.6 |
  2303117.2 |
  2324680.9 |####################
  2346244.5 |
  2367808.2 |
  2389371.8 |
  2410935.5 |####################
  2432499.1 |
  2454062.8 |
  2475626.4 |
  2497190.1 |
  2518753.7 |
  2540317.4 |
  2561881.0 |
  2583444.7 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=375.5% of algo (FFI overhead may distort results)
- evs_std: bridge=726.4% of algo (FFI overhead may distort results)
