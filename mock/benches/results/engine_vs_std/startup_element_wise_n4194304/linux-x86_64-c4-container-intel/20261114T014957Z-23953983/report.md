# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup is fastest but the noisiest (CV 18.4%)

evs_std_startup wins on median (1.66 ms) yet has the highest variance (CV 18.4%), while evs_engine_startup is the steadiest (CV 17.5%, 1.72 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (63.68 us) is smaller than the fastest variant's own run-to-run std-dev (304.89 us); the ranking is inside the noise.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (1.66 ms median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std_startup vs stability leader evs_engine_startup (+4% speed for 1.1x steadier)

evs_std_startup is fastest (1.66 ms, CV 18.4%); evs_engine_startup gives up 3.8% median for 1.1x lower variance (CV 17.5%).


## Key findings

- Baseline (evs_std_startup) is the fastest at 1655452.1 ns median
- Spread: 1.04x (fastest 1655452.1 ns, slowest 1719133.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010001 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 1838228ns | 1723496ns | 1652655ns | 1677263ns | 2005344ns | +1.81% |
| evs_std_startup | 1805575ns | 1660346ns | 1553588ns | 1600745ns | 2022194ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1833684ns | 1648794ns | 2000491ns | +1.87% | 2.287 |
| evs_std_startup | 1800082ns | 1548726ns | 2016141ns | base | 2.330 |

## Performance model

- Peak throughput: 2.708 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 2.440 | 90.1% |
| evs_std_startup | 2.534 | 93.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1838228ns | 1838228ns | +1.81% |
| evs_std_startup | 1805575ns | 1805575ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 1655452ns | base | --- | [1580896, 2124286] | --- | --- | --- | --- |
| evs_engine_startup | 1719134ns | no significant difference | [-473742, +294967]ns | [1650544, 1878045] | no | 0.7266 | 0.7266 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1853682ns | -5.0% |
| 1 | 2 | 1655452ns | +1.7% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 44350010.5ns | 1833683.8ns | 2418.6% | HIGH |
| evs_std_startup | 46221030.2ns | 1800081.7ns | 2567.7% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1648793.5-2000491.4 ns)
  1648793.5 |########################################
  1666378.4 |
  1683963.3 |
  1701548.2 |#############
  1719133.1 |#############
  1736718.0 |
  1754302.9 |
  1771887.8 |
  1789472.7 |#############
  1807057.6 |
  1824642.5 |
  1842227.4 |
  1859812.3 |
  1877397.2 |#############
  1894982.1 |
  1912567.0 |
  1930151.9 |
  1947736.8 |
  1965321.7 |
  1982906.6 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 1548726.4-2016140.6 ns)
  1548726.4 |####################
  1572097.1 |########################################
  1595467.8 |
  1618838.5 |####################
  1642209.2 |
  1665579.9 |####################
  1688950.7 |
  1712321.4 |
  1735692.1 |
  1759062.8 |
  1782433.5 |####################
  1805804.2 |
  1829174.9 |
  1852545.6 |
  1875916.3 |
  1899287.1 |
  1922657.8 |
  1946028.5 |
  1969399.2 |
  1992769.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=2551.8% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=2783.5% of algo (FFI overhead may distort results)
