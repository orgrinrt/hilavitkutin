# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 63% (significant)

evs_std_par is -1.87 ms (63%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 2.7x slower than the field

evs_engine (3.02 ms) is 2.7x the fastest (1.10 ms), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (160% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 160% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 1099408.3 ns median (-63.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.74x (fastest 1099408.3 ns, slowest 3015570.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, four independent heavy chains |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 3030461ns | 3018508ns | 2989583ns | 3003003ns | 3061275ns | +0.99% |
| evs_engine_par | 1185321ns | 1147783ns | 995042ns | 1084192ns | 1308738ns | -60.50% |
| evs_std | 3000868ns | 2977638ns | 2967467ns | 2972575ns | 3030438ns | base |
| evs_std_par | 1139231ns | 1101908ns | 985150ns | 1048164ns | 1246052ns | -62.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 3028028ns | 2988558ns | 3058098ns | +0.98% | 0.346 |
| evs_engine_par | 1182211ns | 992100ns | 1305421ns | -60.57% | 0.887 |
| evs_std | 2998611ns | 2964858ns | 3027877ns | base | 0.350 |
| evs_std_par | 1136270ns | 982342ns | 1242921ns | -62.11% | 0.923 |

## Performance model

- Peak throughput: 1.067 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.348 | 32.6% |
| evs_engine_par | 0.916 | 85.8% |
| evs_std | 0.352 | 33.0% |
| evs_std_par | 0.954 | 89.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 3030461ns | 3030461ns | +0.99% |
| evs_engine_par | 1185321ns | 1185321ns | -60.50% |
| evs_std | 3000868ns | 3000868ns | base |
| evs_std_par | 1139231ns | 1139231ns | -62.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2975279ns | base | --- | [2969092, 3028858] | --- | --- | --- | --- |
| evs_engine | 3015571ns | no significant difference | [-14975, +97175]ns | [2996500, 3070450] | no | 0.2891 | 0.2891 | 0 |
| evs_engine_par | 1144454ns | -1825120.9ns (-61.3%) | [-1976992, -1661717]ns | [1003383, 1307425] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 1099408ns | -1871154.2ns (-62.9%) | [-1942500, -1723183]ns | [1030308, 1301042] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 2996683ns | +1.6% | -59.7% | -58.0% |
| 1 | 2 | 2975279ns | +0.7% | -61.5% | -64.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 23236910.4ns | 3028028.1ns | 767.4% | HIGH |
| evs_engine_par | 23307097.9ns | 1182211.5ns | 1971.5% | HIGH |
| evs_std | 23372965.6ns | 2998611.5ns | 779.5% | HIGH |
| evs_std_par | 23302200.0ns | 1136269.8ns | 2050.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2988558.3-3058097.9 ns)
  2988558.3 |####################
  2992035.3 |
  2995512.3 |########################################
  2998989.2 |
  3002466.2 |
  3005943.2 |####################
  3009420.2 |
  3012897.2 |
  3016374.1 |
  3019851.1 |####################
  3023328.1 |
  3026805.1 |
  3030282.1 |
  3033759.1 |
  3037236.0 |
  3040713.0 |
  3044190.0 |
  3047667.0 |
  3051144.0 |
  3054620.9 |
  (0 below, 3 above range)

evs_engine_par (n=8, range 992100.0-1305420.9 ns)
  992100.0 |########################################
  1007766.0 |
  1023432.1 |
  1039098.1 |
  1054764.2 |
  1070430.2 |
  1086096.3 |
  1101762.3 |####################
  1117428.3 |####################
  1133094.4 |
  1148760.4 |####################
  1164426.5 |
  1180092.5 |
  1195758.6 |
  1211424.6 |
  1227090.6 |
  1242756.7 |
  1258422.7 |
  1274088.8 |
  1289754.8 |####################
  (0 below, 2 above range)

evs_std (n=8, range 2964858.3-3027877.1 ns)
  2964858.3 |####################
  2968009.2 |########################################
  2971160.2 |####################
  2974311.1 |####################
  2977462.1 |
  2980613.0 |
  2983763.9 |
  2986914.9 |
  2990065.8 |
  2993216.7 |
  2996367.7 |
  2999518.6 |
  3002669.6 |
  3005820.5 |
  3008971.4 |
  3012122.4 |
  3015273.3 |
  3018424.3 |
  3021575.2 |####################
  3024726.1 |
  (0 below, 2 above range)

evs_std_par (n=8, range 982341.7-1242920.8 ns)
  982341.7 |####################
  995370.7 |
  1008399.6 |
  1021428.6 |########################################
  1034457.5 |
  1047486.5 |
  1060515.4 |
  1073544.4 |####################
  1086573.3 |
  1099602.3 |
  1112631.3 |####################
  1125660.2 |
  1138689.2 |
  1151718.1 |
  1164747.1 |
  1177776.0 |
  1190805.0 |
  1203834.0 |
  1216862.9 |####################
  1229891.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=768.6% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2025.3% of algo (FFI overhead may distort results)
- evs_std: bridge=782.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=2105.4% of algo (FFI overhead may distort results)
