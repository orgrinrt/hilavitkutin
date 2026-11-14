# branching, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 84% faster than the next best (evs_engine_startup)

evs_std_startup (1.69 ms) leads evs_engine_startup (3.10 ms) by 84%, a clear separation rather than a photo finish. CV 11.2%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (1.69 ms median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std_startup) is the fastest at 1689155.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.84x (fastest 1689155.1 ns, slowest 3100237.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010002 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 3132051ns | 3104898ns | 2413363ns | 2896324ns | 3488517ns | +83.58% |
| evs_std_startup | 1706054ns | 1693911ns | 1473024ns | 1585095ns | 1855031ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 3127374ns | 2409465ns | 3483597ns | +83.84% | 1.341 |
| evs_std_startup | 1701112ns | 1467684ns | 1850062ns | base | 2.466 |

## Performance model

- Peak throughput: 2.858 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 1.353 | 47.3% |
| evs_std_startup | 2.483 | 86.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 3132051ns | 3132051ns | +83.58% |
| evs_std_startup | 1706054ns | 1706054ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 1689155ns | base | --- | [1519333, 1808524] | --- | --- | --- | --- |
| evs_engine_startup | 3100237ns | +1494563.2ns (+88.5%) | [+971664, +1665724]ns | [2780189, 3665689] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1663929ns | +71.9% |
| 1 | 2 | 1689155ns | +95.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine_startup: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 57351082.0ns | 3127374.5ns | 1833.8% | HIGH |
| evs_std_startup | 47747654.4ns | 1701112.5ns | 2806.8% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 2409465.4-3483597.0 ns)
  2409465.4 |########################################
  2463172.0 |
  2516878.6 |
  2570585.1 |
  2624291.7 |
  2677998.3 |
  2731704.9 |########################################
  2785411.5 |
  2839118.0 |
  2892824.6 |########################################
  2946531.2 |########################################
  3000237.8 |
  3053944.4 |
  3107650.9 |
  3161357.5 |
  3215064.1 |########################################
  3268770.7 |
  3322477.3 |########################################
  3376183.8 |
  3429890.4 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 1467683.7-1850062.2 ns)
  1467683.7 |####################
  1486802.6 |
  1505921.6 |####################
  1525040.5 |
  1544159.4 |
  1563278.3 |####################
  1582397.3 |
  1601516.2 |
  1620635.1 |
  1639754.0 |####################
  1658873.0 |
  1677991.9 |
  1697110.8 |
  1716229.8 |
  1735348.7 |########################################
  1754467.6 |
  1773586.5 |
  1792705.5 |####################
  1811824.4 |
  1830943.3 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=1875.3% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=2823.6% of algo (FFI overhead may distort results)
