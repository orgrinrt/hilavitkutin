# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 516% faster than the next best (evs_engine)

evs_std (6.38 us) leads evs_engine (39.28 us) by 516%, a clear separation rather than a photo finish. CV 12.0%.


### evs_engine_par is an outlier: 17.3x slower than the field

evs_engine_par (110.43 us) is 17.3x the fastest (6.38 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.38 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_engine, evs_std_par, evs_engine_par} (516% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_engine, evs_std_par, evs_engine_par} with a 516% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 17.3x the fastest

Fastest evs_std (6.38 us) to slowest evs_engine_par (110.43 us): 17.3x. The strategy choice matters a lot for this workload.


### evs_std_par is inconsistent: worst-20% is 3.4x its best-20%

evs_std_par's best 20% of batches run at 26.29 us but its worst 20% at 90.13 us (3.4x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 6380.9 ns median
- 3 variants significantly slower than baseline
- Spread: 17.31x (fastest 6380.9 ns, slowest 110428.8 ns)

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
| evs_engine | 38817ns | 39598ns | 35088ns | 37559ns | 40693ns | +464.45% |
| evs_engine_par | 136938ns | 110796ns | 88488ns | 103661ns | 174008ns | +1891.26% |
| evs_std | 6877ns | 6787ns | 5809ns | 6301ns | 7576ns | base |
| evs_std_par | 62324ns | 47166ns | 26570ns | 36415ns | 90695ns | +806.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 38502ns | 34823ns | 40333ns | +493.89% | 0.106 |
| evs_engine_par | 136443ns | 88222ns | 173326ns | +2004.63% | 0.030 |
| evs_std | 6483ns | 5528ns | 7132ns | base | 0.632 |
| evs_std_par | 61870ns | 26289ns | 90126ns | +854.33% | 0.066 |

## Performance model

- Peak throughput: 0.741 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.104 | 14.1% |
| evs_engine_par | 0.037 | 5.0% |
| evs_std | 0.642 | 86.6% |
| evs_std_par | 0.088 | 11.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 38817ns | 38817ns | +464.45% |
| evs_engine_par | 136938ns | 136938ns | +1891.26% |
| evs_std | 6877ns | 6877ns | base |
| evs_std_par | 62324ns | 62324ns | +806.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6381ns | base | --- | [5533, 7091] | --- | --- | --- | --- |
| evs_engine | 39279ns | +31919.5ns (+500.2%) | [+29031, +33739]ns | [35939, 39557] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 110429ns | +104116.6ns (+1631.7%) | [+89493, +167557]ns | [96401, 173702] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 46796ns | +40268.9ns (+631.1%) | [+22256, +93651]ns | [27784, 99782] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 5832ns | +574.0% | +2356.7% | +576.8% |
| 1 | 2 | 7000ns | +437.4% | +1360.3% | +790.6% |

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
| evs_engine | 59225.6ns | 38501.7ns | 153.8% | HIGH |
| evs_engine_par | 54241.0ns | 136443.2ns | 39.8% | HIGH |
| evs_std | 78304.7ns | 6483.0ns | 1207.8% | HIGH |
| evs_std_par | 63804.8ns | 61869.5ns | 103.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 34822.9-40332.9 ns)
  34822.9 |#############
  35098.4 |
  35373.9 |
  35649.4 |
  35924.9 |#############
  36200.4 |
  36475.9 |#############
  36751.4 |
  37026.9 |
  37302.4 |
  37577.9 |
  37853.4 |
  38128.9 |
  38404.4 |
  38679.9 |
  38955.4 |
  39230.9 |########################################
  39506.4 |#############
  39781.9 |
  40057.4 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 88221.9-173326.5 ns)
  88221.9 |####################
  92477.1 |####################
  96732.4 |
  100987.6 |
  105242.8 |########################################
  109498.0 |####################
  113753.3 |
  118008.5 |
  122263.7 |
  126519.0 |
  130774.2 |
  135029.4 |
  139284.6 |
  143539.9 |
  147795.1 |
  152050.3 |
  156305.6 |
  160560.8 |
  164816.0 |
  169071.2 |####################
  (0 below, 2 above range)

evs_std (n=8, range 5528.0-7131.8 ns)
   5528.0 |########################################
   5608.2 |
   5688.4 |
   5768.6 |
   5848.8 |
   5928.9 |
   6009.1 |
   6089.3 |########################################
   6169.5 |
   6249.7 |
   6329.9 |
   6410.1 |
   6490.3 |
   6570.5 |####################
   6650.6 |
   6730.8 |
   6811.0 |
   6891.2 |####################
   6971.4 |
   7051.6 |####################
  (0 below, 1 above range)

evs_std_par (n=8, range 26289.2-90125.7 ns)
  26289.2 |########################################
  29481.0 |
  32672.8 |
  35864.7 |####################
  39056.5 |
  42248.3 |####################
  45440.1 |
  48632.0 |####################
  51823.8 |
  55015.6 |
  58207.4 |
  61399.3 |
  64591.1 |
  67782.9 |
  70974.8 |
  74166.6 |
  77358.4 |
  80550.2 |####################
  83742.1 |
  86933.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=152.3% of algo (FFI overhead may distort results)
- evs_engine_par: CV=35.5% (high variance, measurements may be unstable)
- evs_engine_par: bridge=47.9% of algo (FFI overhead may distort results)
- evs_std: bridge=1215.4% of algo (FFI overhead may distort results)
- evs_std_par: CV=56.1% (high variance, measurements may be unstable)
- evs_std_par: worst_20/best_20 = 3.4x (possible bimodal distribution)
- evs_std_par: bridge=135.8% of algo (FFI overhead may distort results)
