# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine_par is an outlier: 5.0x slower than the field

evs_engine_par (265.14 us) is 5.0x the fastest (52.76 us), well off the pack.


### evs_std is fastest but the noisiest (CV 104.1%)

evs_std wins on median (52.76 us) yet has the highest variance (CV 104.1%), while evs_engine_par is the steadiest (CV 16.4%, 265.14 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (52.76 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std, evs_engine} vs {evs_std_par, evs_engine_par} (268% apart)

The field splits into a fast tier {evs_std, evs_engine} and a slow tier {evs_std_par, evs_engine_par} with a 268% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.0x the fastest

Fastest evs_std (52.76 us) to slowest evs_engine_par (265.14 us): 5.0x. The strategy choice matters a lot for this workload.


### evs_std is inconsistent: worst-20% is 3.2x its best-20%

evs_std's best 20% of batches run at 40.56 us but its worst 20% at 130.84 us (3.2x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 52760.9 ns median
- 2 variants significantly slower than baseline
- Spread: 5.03x (fastest 52760.9 ns, slowest 265143.2 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 92729ns | 56870ns | 48274ns | 48401ns | 137089ns | +7.70% |
| evs_engine_par | 265312ns | 266378ns | 169133ns | 257214ns | 295431ns | +208.15% |
| evs_std | 86097ns | 52855ns | 40636ns | 41175ns | 131154ns | base |
| evs_std_par | 209390ns | 211287ns | 132057ns | 177758ns | 252447ns | +143.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 92547ns | 48194ns | 136802ns | +7.74% | 0.044 |
| evs_engine_par | 264326ns | 168750ns | 294767ns | +207.71% | 0.015 |
| evs_std | 85901ns | 40558ns | 130843ns | base | 0.048 |
| evs_std_par | 207673ns | 131706ns | 250152ns | +141.76% | 0.020 |

## Performance model

- Peak throughput: 0.101 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.072 | 71.4% |
| evs_engine_par | 0.015 | 15.3% |
| evs_std | 0.078 | 76.9% |
| evs_std_par | 0.020 | 19.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 92729ns | 92729ns | +7.70% |
| evs_engine_par | 265312ns | 265312ns | +208.15% |
| evs_std | 86097ns | 86097ns | base |
| evs_std_par | 209390ns | 209390ns | +143.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 52761ns | base | --- | [40642, 163008] | --- | --- | --- | --- |
| evs_engine | 56786ns | no significant difference | [-21139, +23781]ns | [48196, 157746] | no | 0.2891 | 0.2891 | 0 |
| evs_engine_par | 265143ns | +175616.0ns (+332.9%) | [+128108, +226266]ns | [245260, 302084] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 208999ns | +128949.6ns (+244.4%) | [+67790, +183841]ns | [170264, 248031] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 78965ns | -38.7% | +219.2% | +158.4% |
| 1 | 2 | 52761ns | +93.2% | +455.4% | +304.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 111691.5ns | 92546.9ns | 120.7% | HIGH |
| evs_engine_par | 269411.1ns | 264326.1ns | 101.9% | HIGH |
| evs_std | 122259.5ns | 85900.9ns | 142.3% | HIGH |
| evs_std_par | 239147.3ns | 207672.5ns | 115.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 48194.5-136802.0 ns)
  48194.5 |########################################
  52624.9 |
  57055.2 |
  61485.6 |##########
  65916.0 |
  70346.4 |
  74776.7 |
  79207.1 |
  83637.5 |
  88067.9 |
  92498.2 |
  96928.6 |
  101359.0 |
  105789.3 |
  110219.7 |
  114650.1 |
  119080.5 |
  123510.8 |
  127941.2 |
  132371.6 |
  (0 below, 3 above range)

evs_engine_par (n=8, range 168749.6-294767.0 ns)
  168749.6 |####################
  175050.5 |
  181351.3 |
  187652.2 |
  193953.1 |
  200254.0 |
  206554.8 |
  212855.7 |
  219156.6 |
  225457.5 |
  231758.3 |
  238059.2 |
  244360.1 |####################
  250660.9 |
  256961.8 |########################################
  263262.7 |####################
  269563.6 |
  275864.4 |
  282165.3 |####################
  288466.2 |
  (0 below, 2 above range)

evs_std (n=8, range 40557.7-130843.0 ns)
  40557.7 |########################################
  45072.0 |
  49586.2 |
  54100.5 |
  58614.8 |
  63129.0 |##########
  67643.3 |
  72157.5 |
  76671.8 |
  81186.1 |
  85700.3 |
  90214.6 |
  94728.9 |
  99243.1 |
  103757.4 |
  108271.6 |
  112785.9 |##########
  117300.2 |
  121814.4 |
  126328.7 |
  (0 below, 2 above range)

evs_std_par (n=8, range 131706.5-250152.1 ns)
  131706.5 |####################
  137628.8 |
  143551.1 |
  149473.3 |
  155395.6 |
  161317.9 |
  167240.2 |####################
  173162.5 |####################
  179084.8 |####################
  185007.0 |
  190929.3 |
  196851.6 |
  202773.9 |
  208696.2 |
  214618.5 |
  220540.7 |
  226463.0 |
  232385.3 |####################
  238307.6 |
  244229.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: CV=58.7% (high variance, measurements may be unstable)
- evs_engine: bridge=114.1% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=97.7% of algo (FFI overhead may distort results)
- evs_std: CV=63.9% (high variance, measurements may be unstable)
- evs_std: worst_20/best_20 = 3.2x (possible bimodal distribution)
- evs_std: bridge=133.4% of algo (FFI overhead may distort results)
- evs_std_par: CV=21.9% (high variance, measurements may be unstable)
- evs_std_par: bridge=111.3% of algo (FFI overhead may distort results)
