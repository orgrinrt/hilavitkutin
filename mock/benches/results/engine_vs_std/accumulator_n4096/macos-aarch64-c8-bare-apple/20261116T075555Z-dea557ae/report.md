# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 3086% faster than the next best (evs_engine)

evs_std (408 ns) leads evs_engine (13.01 us) by 3086%, a clear separation rather than a photo finish. CV 1.8%.


### evs_engine_par is an outlier: 135.0x slower than the field

evs_engine_par (55.13 us) is 135.0x the fastest (408 ns), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (408 ns median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_engine, evs_std_par, evs_engine_par} (3086% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_engine, evs_std_par, evs_engine_par} with a 3086% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 135.0x the fastest

Fastest evs_std (408 ns) to slowest evs_engine_par (55.13 us): 135.0x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 408.3 ns median
- 3 variants significantly slower than baseline
- Spread: 135.02x (fastest 408.3 ns, slowest 55129.2 ns)

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
| evs_engine | 13292ns | 13108ns | 13050ns | 13072ns | 13517ns | +2472.60% |
| evs_engine_par | 55331ns | 55538ns | 50042ns | 52642ns | 58671ns | +10609.36% |
| evs_std | 517ns | 508ns | 492ns | 503ns | 533ns | base |
| evs_std_par | 27041ns | 27412ns | 20767ns | 26714ns | 28854ns | +5133.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 13183ns | 12967ns | 13396ns | +3128.54% | 0.311 |
| evs_engine_par | 54934ns | 49625ns | 58235ns | +13353.18% | 0.075 |
| evs_std | 408ns | 400ns | 415ns | base | 10.031 |
| evs_std_par | 26858ns | 20625ns | 28615ns | +6477.48% | 0.153 |

## Performance model

- Peak throughput: 10.240 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.315 | 3.1% |
| evs_engine_par | 0.074 | 0.7% |
| evs_std | 10.032 | 98.0% |
| evs_std_par | 0.151 | 1.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 13292ns | 13292ns | +2472.60% |
| evs_engine_par | 55331ns | 55331ns | +10609.36% |
| evs_std | 517ns | 517ns | base |
| evs_std_par | 27041ns | 27041ns | +5133.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 408ns | base | --- | [400, 417] | --- | --- | --- | --- |
| evs_engine | 13008ns | +12600.0ns (+3086.0%) | [+12567, +13008]ns | [12967, 13417] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 55129ns | +54720.8ns (+13402.1%) | [+49367, +58825]ns | [49767, 59233] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 27154ns | +26745.8ns (+6550.5%) | [+25208, +29283]ns | [25625, 29700] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 412ns | +3053.5% | +13608.1% | +6297.0% |
| 1 | 2 | 404ns | +3126.9% | +13034.6% | +6934.3% |

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
- evs_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22243.8ns | 13183.4ns | 168.7% | HIGH |
| evs_engine_par | 24668.8ns | 54934.4ns | 44.9% | HIGH |
| evs_std | 21903.1ns | 408.3ns | 5364.0% | HIGH |
| evs_std_par | 25732.3ns | 26858.3ns | 95.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 12966.7-13395.8 ns)
  12966.7 |########################################
  12988.2 |
  13009.6 |
  13031.1 |##########
  13052.5 |
  13074.0 |
  13095.4 |##########
  13116.9 |
  13138.4 |
  13159.8 |
  13181.3 |
  13202.7 |
  13224.2 |
  13245.6 |
  13267.1 |
  13288.6 |
  13310.0 |
  13331.5 |
  13352.9 |
  13374.4 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 49625.0-58235.4 ns)
  49625.0 |########################################
  50055.5 |
  50486.0 |
  50916.6 |
  51347.1 |
  51777.6 |
  52208.1 |
  52638.6 |
  53069.2 |####################
  53499.7 |####################
  53930.2 |
  54360.7 |
  54791.3 |
  55221.8 |
  55652.3 |
  56082.8 |####################
  56513.3 |####################
  56943.9 |
  57374.4 |
  57804.9 |
  (0 below, 2 above range)

evs_std (n=8, range 400.0-414.6 ns)
    400.0 |########################################
    400.7 |
    401.5 |
    402.2 |
    402.9 |
    403.6 |
    404.4 |
    405.1 |
    405.8 |
    406.6 |
    407.3 |
    408.0 |##########################
    408.8 |
    409.5 |
    410.2 |
    411.0 |
    411.7 |
    412.4 |
    413.1 |
    413.9 |
  (0 below, 3 above range)

evs_std_par (n=8, range 20625.0-28614.6 ns)
  20625.0 |####################
  21024.5 |
  21424.0 |
  21823.4 |
  22222.9 |
  22622.4 |
  23021.9 |
  23421.4 |
  23820.8 |
  24220.3 |
  24619.8 |
  25019.3 |
  25418.7 |####################
  25818.2 |
  26217.7 |
  26617.2 |####################
  27016.7 |########################################
  27416.1 |####################
  27815.6 |
  28215.1 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=168.5% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=45.0% of algo (FFI overhead may distort results)
- evs_std: bridge=5364.7% of algo (FFI overhead may distort results)
- evs_std_par: bridge=95.8% of algo (FFI overhead may distort results)
