# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 456% faster than the next best (evs_std_par)

evs_std (6.47 us) leads evs_std_par (36.03 us) by 456%, a clear separation rather than a photo finish. CV 1.9%.


### evs_engine_par is an outlier: 10.6x slower than the field

evs_engine_par (68.47 us) is 10.6x the fastest (6.47 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.47 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_std_par, evs_engine, evs_engine_par} (456% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_std_par, evs_engine, evs_engine_par} with a 456% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 10.6x the fastest

Fastest evs_std (6.47 us) to slowest evs_engine_par (68.47 us): 10.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 6475.0 ns median
- 3 variants significantly slower than baseline
- Spread: 10.57x (fastest 6475.0 ns, slowest 68466.6 ns)

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
| evs_engine | 55610ns | 55188ns | 54642ns | 54950ns | 56348ns | +704.37% |
| evs_engine_par | 69750ns | 69100ns | 67683ns | 68750ns | 71017ns | +908.89% |
| evs_std | 6914ns | 6658ns | 6567ns | 6597ns | 7238ns | base |
| evs_std_par | 37327ns | 36833ns | 34150ns | 36375ns | 38835ns | +439.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 54958ns | 54000ns | 55517ns | +743.62% | 1.192 |
| evs_engine_par | 69159ns | 67358ns | 70242ns | +961.61% | 0.948 |
| evs_std | 6515ns | 6450ns | 6567ns | base | 10.060 |
| evs_std_par | 36678ns | 33875ns | 38202ns | +463.02% | 1.787 |

## Performance model

- Peak throughput: 10.161 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.196 | 11.8% |
| evs_engine_par | 0.957 | 9.4% |
| evs_std | 10.121 | 99.6% |
| evs_std_par | 1.819 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 55610ns | 55610ns | +704.37% |
| evs_engine_par | 69750ns | 69750ns | +908.89% |
| evs_std | 6914ns | 6914ns | base |
| evs_std_par | 37327ns | 37327ns | +439.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6475ns | base | --- | [6458, 6475] | --- | --- | --- | --- |
| evs_engine | 54796ns | +48325.0ns (+746.3%) | [+47742, +49392]ns | [54217, 55842] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 68467ns | +61991.7ns (+957.4%) | [+61267, +64275]ns | [68108, 70733] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 36033ns | +29558.3ns (+456.5%) | [+28567, +31883]ns | [35333, 38350] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 6475ns | +741.4% | +957.4% | +482.7% |
| 1 | 2 | 6467ns | +748.6% | +962.5% | +447.0% |

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
| evs_engine | 353019.8ns | 54958.4ns | 642.3% | HIGH |
| evs_engine_par | 363501.0ns | 69159.4ns | 525.6% | HIGH |
| evs_std | 352831.2ns | 6514.6ns | 5416.0% | HIGH |
| evs_std_par | 360410.4ns | 36678.1ns | 982.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 54000.0-55516.7 ns)
  54000.0 |########################################
  54075.8 |
  54151.7 |########################################
  54227.5 |
  54303.3 |
  54379.2 |
  54455.0 |
  54530.8 |
  54606.7 |########################################
  54682.5 |########################################
  54758.3 |
  54834.2 |########################################
  54910.0 |########################################
  54985.9 |
  55061.7 |
  55137.5 |
  55213.4 |
  55289.2 |
  55365.0 |
  55440.9 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 67358.3-70241.6 ns)
  67358.3 |#############
  67502.5 |
  67646.6 |
  67790.8 |
  67935.0 |
  68079.1 |#############
  68223.3 |
  68367.5 |########################################
  68511.6 |
  68655.8 |
  68800.0 |
  68944.1 |#############
  69088.3 |
  69232.5 |
  69376.6 |
  69520.8 |
  69665.0 |
  69809.1 |
  69953.3 |
  70097.5 |
  (0 below, 2 above range)

evs_std (n=8, range 6450.0-6566.7 ns)
   6450.0 |##########
   6455.8 |##########
   6461.7 |##########
   6467.5 |
   6473.3 |########################################
   6479.2 |
   6485.0 |
   6490.8 |
   6496.7 |
   6502.5 |
   6508.3 |
   6514.2 |
   6520.0 |
   6525.8 |
   6531.7 |
   6537.5 |
   6543.3 |
   6549.2 |
   6555.0 |
   6560.8 |
  (0 below, 1 above range)

evs_std_par (n=8, range 33875.0-38202.1 ns)
  33875.0 |########################################
  34091.4 |
  34307.7 |
  34524.1 |
  34740.4 |
  34956.8 |
  35173.1 |########################################
  35389.5 |########################################
  35605.8 |
  35822.2 |########################################
  36038.5 |########################################
  36254.9 |
  36471.2 |
  36687.6 |
  36904.0 |########################################
  37120.3 |
  37336.7 |
  37553.0 |
  37769.4 |
  37985.7 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=643.2% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=531.1% of algo (FFI overhead may distort results)
- evs_std: bridge=5442.1% of algo (FFI overhead may distort results)
- evs_std_par: bridge=999.0% of algo (FFI overhead may distort results)
