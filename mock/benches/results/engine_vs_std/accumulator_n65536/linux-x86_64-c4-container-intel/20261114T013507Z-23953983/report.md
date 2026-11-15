# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 154% faster than the next best (evs_engine)

evs_std (27.98 us) leads evs_engine (71.08 us) by 154%, a clear separation rather than a photo finish. CV 15.2%.


### evs_engine_par is an outlier: 10.4x slower than the field

evs_engine_par (290.98 us) is 10.4x the fastest (27.98 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (27.98 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 10.4x the fastest

Fastest evs_std (27.98 us) to slowest evs_engine_par (290.98 us): 10.4x. The strategy choice matters a lot for this workload.


### evs_std_par is inconsistent: worst-20% is 10.2x its best-20%

evs_std_par's best 20% of batches run at 40.17 us but its worst 20% at 408.05 us (10.2x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 27981.3 ns median
- 3 variants significantly slower than baseline
- Spread: 10.40x (fastest 27981.3 ns, slowest 290976.2 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 74226ns | 71780ns | 68948ns | 70605ns | 78262ns | +156.44% |
| evs_engine_par | 327872ns | 291891ns | 103139ns | 270641ns | 426979ns | +1032.75% |
| evs_std | 28945ns | 29104ns | 21191ns | 26965ns | 32368ns | base |
| evs_std_par | 228342ns | 123803ns | 40538ns | 49438ns | 409471ns | +688.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 73740ns | 68600ns | 77584ns | +161.21% | 0.889 |
| evs_engine_par | 326732ns | 102421ns | 425935ns | +1057.38% | 0.201 |
| evs_std | 28230ns | 20826ns | 31572ns | base | 2.321 |
| evs_std_par | 227418ns | 40167ns | 408050ns | +705.58% | 0.288 |

## Performance model

- Peak throughput: 3.147 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.922 | 29.3% |
| evs_engine_par | 0.225 | 7.2% |
| evs_std | 2.342 | 74.4% |
| evs_std_par | 0.533 | 16.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 74226ns | 74226ns | +156.44% |
| evs_engine_par | 327872ns | 327872ns | +1032.75% |
| evs_std | 28945ns | 28945ns | base |
| evs_std_par | 228342ns | 228342ns | +688.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 27981ns | base | --- | [25829, 30814] | --- | --- | --- | --- |
| evs_engine | 71082ns | +45460.4ns (+162.5%) | [+39911, +49652]ns | [69602, 81847] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 290976ns | +262224.0ns (+937.1%) | [+227333, +441694]ns | [257023, 462520] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 122953ns | +97904.1ns (+349.9%) | [+10477, +514934]ns | [45841, 541144] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 30252ns | +132.8% | +810.1% | +57.4% |
| 1 | 2 | 26019ns | +178.5% | +1333.5% | +1441.7% |

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
| evs_engine | 387579.7ns | 73739.8ns | 525.6% | HIGH |
| evs_engine_par | 352930.4ns | 326731.6ns | 108.0% | HIGH |
| evs_std | 336873.8ns | 28230.3ns | 1193.3% | HIGH |
| evs_std_par | 348001.2ns | 227417.9ns | 153.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 68600.2-77583.8 ns)
  68600.2 |####################
  69049.4 |
  69498.6 |####################
  69947.7 |
  70396.9 |####################
  70846.1 |########################################
  71295.3 |
  71744.5 |
  72193.6 |
  72642.8 |
  73092.0 |
  73541.2 |
  73990.3 |####################
  74439.5 |
  74888.7 |
  75337.9 |
  75787.1 |
  76236.2 |
  76685.4 |
  77134.6 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 102421.3-425934.5 ns)
  102421.3 |########################################
  118597.0 |
  134772.6 |
  150948.3 |
  167123.9 |
  183299.6 |
  199475.3 |
  215650.9 |
  231826.6 |
  248002.3 |########################################
  264177.9 |########################################
  280353.6 |########################################
  296529.2 |########################################
  312704.9 |
  328880.6 |
  345056.2 |
  361231.9 |
  377407.5 |
  393583.2 |
  409758.9 |
  (0 below, 3 above range)

evs_std (n=8, range 20825.9-31572.0 ns)
  20825.9 |####################
  21363.2 |
  21900.5 |
  22437.8 |
  22975.1 |
  23512.4 |
  24049.7 |
  24587.0 |
  25124.3 |
  25661.6 |####################
  26199.0 |########################################
  26736.3 |
  27273.6 |
  27810.9 |
  28348.2 |
  28885.5 |####################
  29422.8 |####################
  29960.1 |
  30497.4 |####################
  31034.7 |
  (0 below, 1 above range)

evs_std_par (n=8, range 40167.2-408049.6 ns)
  40167.2 |########################################
  58561.3 |
  76955.4 |
  95349.6 |
  113743.7 |
  132137.8 |
  150531.9 |
  168926.0 |
  187320.2 |##########
  205714.3 |
  224108.4 |
  242502.5 |
  260896.6 |##########
  279290.8 |
  297684.9 |
  316079.0 |
  334473.1 |
  352867.2 |
  371261.4 |
  389655.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=462.2% of algo (FFI overhead may distort results)
- evs_engine_par: CV=37.9% (high variance, measurements may be unstable)
- evs_engine_par: worst_20/best_20 = 4.2x (possible bimodal distribution)
- evs_engine_par: bridge=119.2% of algo (FFI overhead may distort results)
- evs_std: bridge=1229.1% of algo (FFI overhead may distort results)
- evs_std_par: CV=98.1% (high variance, measurements may be unstable)
- evs_std_par: worst_20/best_20 = 10.2x (possible bimodal distribution)
- evs_std_par: bridge=283.8% of algo (FFI overhead may distort results)
