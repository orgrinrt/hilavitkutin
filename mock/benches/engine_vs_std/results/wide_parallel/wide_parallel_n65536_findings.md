# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine_par beats baseline by 41% (significant)

evs_engine_par is -417.27 us (41%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (70% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 70% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 594228.1 ns median (-41.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.74x (fastest 594228.1 ns, slowest 1036744.6 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 1032547ns | 1038195ns | 920088ns | 956581ns | 1117637ns | +1.20% |
| evs_engine_par | 633979ns | 603938ns | 576360ns | 594048ns | 678332ns | -37.87% |
| evs_std | 1020330ns | 1024840ns | 914670ns | 933408ns | 1111937ns | base |
| evs_std_par | 615043ns | 596278ns | 506868ns | 578375ns | 669587ns | -39.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1031225ns | 918975ns | 1116284ns | +1.24% | 0.064 |
| evs_engine_par | 632173ns | 574495ns | 676468ns | -37.94% | 0.104 |
| evs_std | 1018601ns | 913269ns | 1110114ns | base | 0.064 |
| evs_std_par | 613180ns | 505251ns | 667394ns | -39.80% | 0.107 |

## Performance model

- Peak throughput: 0.130 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.063 | 48.7% |
| evs_engine_par | 0.109 | 83.9% |
| evs_std | 0.064 | 49.4% |
| evs_std_par | 0.110 | 85.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 1032547ns | 1032547ns | +1.20% |
| evs_engine_par | 633979ns | 633979ns | -37.87% |
| evs_std | 1020330ns | 1020330ns | base |
| evs_std_par | 615043ns | 615043ns | -39.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 1023064ns | base | --- | [923828, 1112480] | --- | --- | --- | --- |
| evs_engine | 1036745ns | no significant difference | [-190439, +182774]ns | [922042, 1115657] | no | 1.0000 | 1.0000 | 0 |
| evs_engine_par | 602343ns | -417269.8ns (-40.8%) | [-518432, -281958]ns | [581537, 643537] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 594228ns | -397991.8ns (-38.9%) | [-447675, -347168]ns | [566101, 666573] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 924662ns | +20.3% | -32.6% | -38.3% |
| 1 | 2 | 1109537ns | -14.8% | -46.1% | -40.3% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 1378796.0ns | 1031224.7ns | 133.7% | HIGH |
| evs_engine_par | 1329339.7ns | 632173.2ns | 210.3% | HIGH |
| evs_std | 1397528.3ns | 1018601.0ns | 137.2% | HIGH |
| evs_std_par | 1404767.2ns | 613179.8ns | 229.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 918975.3-1116283.6 ns)
  918975.3 |########################################
  928840.7 |
  938706.1 |
  948571.5 |
  958437.0 |
  968302.4 |########################################
  978167.8 |
  988033.2 |
  997898.6 |
  1007764.0 |
  1017629.4 |
  1027494.9 |
  1037360.3 |
  1047225.7 |
  1057091.1 |
  1066956.5 |
  1076821.9 |
  1086687.3 |
  1096552.7 |####################
  1106418.2 |########################################
  (0 below, 1 above range)

evs_engine_par (n=8, range 574495.0-676468.0 ns)
  574495.0 |####################
  579593.7 |####################
  584692.3 |
  589790.9 |####################
  594889.6 |
  599988.2 |########################################
  605086.9 |
  610185.6 |
  615284.2 |
  620382.8 |####################
  625481.5 |
  630580.2 |
  635678.8 |
  640777.4 |####################
  645876.1 |
  650974.8 |
  656073.4 |
  661172.1 |
  666270.7 |
  671369.3 |
  (0 below, 1 above range)

evs_std (n=8, range 913269.3-1110114.2 ns)
  913269.3 |####################
  923111.5 |########################################
  932953.8 |
  942796.0 |####################
  952638.3 |
  962480.5 |
  972322.8 |
  982165.0 |
  992007.2 |
  1001849.5 |
  1011691.7 |
  1021534.0 |
  1031376.2 |
  1041218.5 |
  1051060.7 |
  1060903.0 |
  1070745.2 |
  1080587.4 |
  1090429.7 |
  1100271.9 |########################################
  (0 below, 2 above range)

evs_std_par (n=8, range 505251.1-667394.2 ns)
  505251.1 |########################################
  513358.3 |
  521465.4 |
  529572.6 |
  537679.7 |
  545786.9 |
  553894.0 |
  562001.2 |########################################
  570108.3 |########################################
  578215.5 |
  586322.6 |########################################
  594429.8 |########################################
  602536.9 |
  610644.1 |
  618751.3 |
  626858.4 |
  634965.6 |
  643072.7 |
  651179.9 |########################################
  659287.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=133.1% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=226.5% of algo (FFI overhead may distort results)
- evs_std: bridge=136.5% of algo (FFI overhead may distort results)
- evs_std_par: bridge=240.6% of algo (FFI overhead may distort results)
