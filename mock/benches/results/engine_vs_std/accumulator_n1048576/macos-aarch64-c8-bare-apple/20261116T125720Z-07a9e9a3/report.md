# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 15% faster than the next best (evs_std)

evs_std_par (91.32 us) leads evs_std (105.30 us) by 15%, a clear separation rather than a photo finish. CV 4.5%.


### evs_engine is an outlier: 8.0x slower than the field

evs_engine (732.42 us) is 8.0x the fastest (91.32 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 8.0x the fastest

Fastest evs_std_par (91.32 us) to slowest evs_engine (732.42 us): 8.0x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 91320.9 ns median (-13.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 8.02x (fastest 91320.9 ns, slowest 732420.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, a transform feeding the append surface |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000003 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 734715ns | 733875ns | 724983ns | 730264ns | 740485ns | +587.88% |
| evs_engine_par | 230767ns | 232646ns | 225308ns | 229153ns | 233342ns | +116.06% |
| evs_std | 106808ns | 106833ns | 105275ns | 106456ns | 107456ns | base |
| evs_std_par | 94350ns | 92838ns | 88083ns | 91325ns | 98185ns | -11.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 733216ns | 723775ns | 738854ns | +596.53% | 1.430 |
| evs_engine_par | 228746ns | 223783ns | 231560ns | +117.30% | 4.584 |
| evs_std | 105267ns | 104600ns | 105604ns | base | 9.961 |
| evs_std_par | 92519ns | 87300ns | 95900ns | -12.11% | 11.334 |

## Performance model

- Peak throughput: 12.011 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.432 | 11.9% |
| evs_engine_par | 4.549 | 37.9% |
| evs_std | 9.958 | 82.9% |
| evs_std_par | 11.482 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 734715ns | 734715ns | +587.88% |
| evs_engine_par | 230767ns | 230767ns | +116.06% |
| evs_std | 106808ns | 106808ns | base |
| evs_std_par | 94350ns | 94350ns | -11.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 105296ns | base | --- | [104642, 105533] | --- | --- | --- | --- |
| evs_engine | 732421ns | +627450.0ns (+595.9%) | [+621767, +637383]ns | [726992, 742633] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 230488ns | +125045.9ns (+118.8%) | [+119183, +126625]ns | [224758, 231442] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 91321ns | -13916.7ns (-13.2%) | [-16025, -6842]ns | [88575, 98500] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 105296ns | +597.7% | +116.9% | -11.7% |
| 1 | 2 | 104996ns | +595.0% | +119.5% | -15.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 5646339.6ns | 733215.6ns | 770.1% | HIGH |
| evs_engine_par | 5666874.0ns | 228745.8ns | 2477.4% | HIGH |
| evs_std | 5642593.8ns | 105266.7ns | 5360.3% | HIGH |
| evs_std_par | 5664589.6ns | 92518.8ns | 6122.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 723775.0-738854.2 ns)
  723775.0 |########################################
  724529.0 |
  725282.9 |
  726036.9 |
  726790.8 |########################################
  727544.8 |
  728298.8 |########################################
  729052.7 |
  729806.7 |
  730560.6 |########################################
  731314.6 |
  732068.5 |
  732822.5 |
  733576.5 |########################################
  734330.4 |
  735084.4 |########################################
  735838.3 |
  736592.3 |
  737346.3 |
  738100.2 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 223783.3-231560.4 ns)
  223783.3 |####################
  224172.2 |
  224561.0 |####################
  224949.9 |
  225338.7 |####################
  225727.6 |
  226116.4 |
  226505.3 |
  226894.1 |
  227283.0 |
  227671.9 |
  228060.7 |
  228449.6 |
  228838.4 |
  229227.3 |
  229616.1 |####################
  230005.0 |
  230393.9 |
  230782.7 |####################
  231171.6 |########################################
  (0 below, 1 above range)

evs_std (n=8, range 104600.0-105604.2 ns)
  104600.0 |########################################
  104650.2 |
  104700.4 |
  104750.6 |
  104800.8 |
  104851.0 |
  104901.3 |
  104951.5 |
  105001.7 |
  105051.9 |
  105102.1 |
  105152.3 |
  105202.5 |########################################
  105252.7 |
  105302.9 |########################################
  105353.1 |
  105403.3 |
  105453.5 |
  105503.8 |####################
  105554.0 |
  (0 below, 1 above range)

evs_std_par (n=8, range 87300.0-95900.0 ns)
  87300.0 |########################################
  87730.0 |
  88160.0 |########################################
  88590.0 |
  89020.0 |
  89450.0 |
  89880.0 |########################################
  90310.0 |
  90740.0 |########################################
  91170.0 |
  91600.0 |########################################
  92030.0 |
  92460.0 |
  92890.0 |
  93320.0 |
  93750.0 |########################################
  94180.0 |
  94610.0 |
  95040.0 |
  95470.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=770.8% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2458.5% of algo (FFI overhead may distort results)
- evs_std: bridge=5359.6% of algo (FFI overhead may distort results)
- evs_std_par: bridge=6199.8% of algo (FFI overhead may distort results)
