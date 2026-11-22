# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 179% faster than the next best (evs_engine)

evs_std (20.55 us) leads evs_engine (57.29 us) by 179%, a clear separation rather than a photo finish. CV 10.3%.


### evs_engine_par is an outlier: 12.4x slower than the field

evs_engine_par (253.99 us) is 12.4x the fastest (20.55 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (20.55 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 12.4x the fastest

Fastest evs_std (20.55 us) to slowest evs_engine_par (253.99 us): 12.4x. The strategy choice matters a lot for this workload.


### evs_engine_par is inconsistent: worst-20% is 2.3x its best-20%

evs_engine_par's best 20% of batches run at 186.28 us but its worst 20% at 426.08 us (2.3x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 20554.8 ns median
- 3 variants significantly slower than baseline
- Spread: 12.36x (fastest 20554.8 ns, slowest 253991.0 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 57404ns | 57422ns | 56886ns | 57205ns | 57683ns | +167.69% |
| evs_engine_par | 326048ns | 254707ns | 186492ns | 236699ns | 427949ns | +1420.46% |
| evs_std | 21444ns | 20677ns | 20508ns | 20595ns | 22315ns | base |
| evs_std_par | 195036ns | 194390ns | 174860ns | 180994ns | 210612ns | +809.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 57282ns | 56755ns | 57569ns | +168.58% | 1.144 |
| evs_engine_par | 324933ns | 186281ns | 426080ns | +1423.53% | 0.202 |
| evs_std | 21328ns | 20409ns | 22189ns | base | 3.073 |
| evs_std_par | 194617ns | 174637ns | 210120ns | +812.52% | 0.337 |

## Performance model

- Peak throughput: 3.211 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.144 | 35.6% |
| evs_engine_par | 0.258 | 8.0% |
| evs_std | 3.188 | 99.3% |
| evs_std_par | 0.338 | 10.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 57404ns | 57404ns | +167.69% |
| evs_engine_par | 326048ns | 326048ns | +1420.46% |
| evs_std | 21444ns | 21444ns | base |
| evs_std_par | 195036ns | 195036ns | +809.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 20555ns | base | --- | [20418, 20674] | --- | --- | --- | --- |
| evs_engine | 57289ns | +36747.6ns (+178.8%) | [+36378, +37070]ns | [56787, 57652] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 253991ns | +230240.4ns (+1120.1%) | [+206225, +559541]ns | [226807, 579950] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 194052ns | +173552.8ns (+844.3%) | [+154128, +192226]ns | [177250, 212828] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 20463ns | +178.4% | +1077.9% | +770.3% |
| 1 | 2 | 20592ns | +179.1% | +1180.7% | +912.5% |

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
| evs_engine | 243865.4ns | 57282.2ns | 425.7% | HIGH |
| evs_engine_par | 1013192.2ns | 324932.8ns | 311.8% | HIGH |
| evs_std | 248057.5ns | 21327.6ns | 1163.1% | HIGH |
| evs_std_par | 511026.9ns | 194617.4ns | 262.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 56755.3-57568.6 ns)
  56755.3 |########################################
  56796.0 |
  56836.6 |
  56877.3 |
  56918.0 |
  56958.6 |
  56999.3 |
  57040.0 |
  57080.6 |
  57121.3 |
  57162.0 |####################
  57202.6 |
  57243.3 |####################
  57283.9 |####################
  57324.6 |
  57365.3 |
  57405.9 |
  57446.6 |####################
  57487.3 |
  57527.9 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 186281.0-426080.1 ns)
  186281.0 |####################
  198271.0 |
  210260.9 |
  222250.9 |########################################
  234240.8 |
  246230.8 |####################
  258220.7 |########################################
  270210.7 |
  282200.6 |
  294190.6 |
  306180.5 |
  318170.5 |
  330160.4 |
  342150.4 |
  354140.4 |
  366130.3 |
  378120.3 |
  390110.2 |
  402100.2 |
  414090.1 |
  (0 below, 2 above range)

evs_std (n=8, range 20409.1-22189.3 ns)
  20409.1 |##########################
  20498.1 |########################################
  20587.1 |##########################
  20676.1 |
  20765.1 |
  20854.2 |
  20943.2 |
  21032.2 |
  21121.2 |
  21210.2 |
  21299.2 |
  21388.2 |
  21477.2 |
  21566.2 |
  21655.3 |
  21744.3 |
  21833.3 |
  21922.3 |
  22011.3 |
  22100.3 |
  (0 below, 1 above range)

evs_std_par (n=8, range 174637.0-210120.1 ns)
  174637.0 |########################################
  176411.2 |########################################
  178185.3 |########################################
  179959.5 |
  181733.6 |
  183507.8 |
  185281.9 |########################################
  187056.1 |
  188830.2 |
  190604.4 |
  192378.5 |
  194152.7 |
  195926.8 |
  197701.0 |
  199475.2 |
  201249.3 |########################################
  203023.5 |########################################
  204797.6 |
  206571.8 |
  208345.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=425.1% of algo (FFI overhead may distort results)
- evs_engine_par: CV=47.4% (high variance, measurements may be unstable)
- evs_engine_par: bridge=295.6% of algo (FFI overhead may distort results)
- evs_std: bridge=1186.1% of algo (FFI overhead may distort results)
- evs_std_par: bridge=239.5% of algo (FFI overhead may distort results)
