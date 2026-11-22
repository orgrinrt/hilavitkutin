# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (43.03 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine_par at 21.86 ms).


### evs_engine_par dominates: 11% faster than the next best (evs_std_par)

evs_engine_par (21.86 ms) leads evs_std_par (24.29 ms) by 11%, a clear separation rather than a photo finish. CV 40.5%.


### evs_engine_par beats baseline by 49% (significant)

evs_engine_par is -21.13 ms (49%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine_par is fastest but the noisiest (CV 40.5%)

evs_engine_par wins on median (21.86 ms) yet has the highest variance (CV 40.5%), while evs_engine is the steadiest (CV 0.6%, 41.50 ms).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_engine_par, evs_std_par} vs {evs_engine, evs_std} (71% apart)

The field splits into a fast tier {evs_engine_par, evs_std_par} and a slow tier {evs_engine, evs_std} with a 71% jump between them - a qualitative difference, not a gradient.


### evs_engine_par is inconsistent: worst-20% is 2.1x its best-20%

evs_engine_par's best 20% of batches run at 14.27 ms but its worst 20% at 30.55 ms (2.1x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: evs_engine_par at 21858888.6 ns median (-49.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.97x (fastest 21858888.6 ns, slowest 43029436.8 ns)

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
| evs_engine | 41579423ns | 41504031ns | 41438333ns | 41479176ns | 41689881ns | -3.31% |
| evs_engine_par | 23888450ns | 21861322ns | 14267568ns | 18208929ns | 30553311ns | -44.45% |
| evs_std | 43003346ns | 43031264ns | 41663423ns | 41729981ns | 44293350ns | base |
| evs_std_par | 24546109ns | 24292353ns | 16271946ns | 21187681ns | 29133471ns | -42.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 41578198ns | 41436951ns | 41688707ns | -3.31% | 0.101 |
| evs_engine_par | 23885921ns | 14265027ns | 30550749ns | -44.45% | 0.176 |
| evs_std | 43001712ns | 41662019ns | 44291644ns | base | 0.098 |
| evs_std_par | 24542955ns | 16268817ns | 29130197ns | -42.93% | 0.171 |

## Performance model

- Peak throughput: 0.294 Gops/s (evs_engine_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.101 | 34.4% |
| evs_engine_par | 0.192 | 65.3% |
| evs_std | 0.097 | 33.2% |
| evs_std_par | 0.173 | 58.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 41579423ns | 41579423ns | -3.31% |
| evs_engine_par | 23888450ns | 23888450ns | -44.45% |
| evs_std | 43003346ns | 43003346ns | base |
| evs_std_par | 24546109ns | 24546109ns | -42.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 43029437ns | base | --- | [41679839, 44303022] | --- | --- | --- | --- |
| evs_engine | 41502742ns | -1244535.3ns (-2.9%) | [-2757937, -169464]ns | [41444093, 41538801] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 21858889ns | -21132542.0ns (-49.1%) | [-24828734, -13304459]ns | [16833285, 28394973] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 24289205ns | -18693315.6ns (-43.4%) | [-22132459, -15873606]ns | [19673368, 28409219] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 41689636ns | -0.5% | -54.7% | -50.2% |
| 1 | 2 | 44292924ns | -6.3% | -43.9% | -37.7% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 2/2, lost 0/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 67490246.0ns | 41578198.1ns | 162.3% | HIGH |
| evs_engine_par | 68680747.6ns | 23885921.2ns | 287.5% | HIGH |
| evs_std | 68347005.4ns | 43001711.7ns | 158.9% | HIGH |
| evs_std_par | 69603097.3ns | 24542955.3ns | 283.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 41436951.0-41688707.3 ns)
  41436951.0 |########################################
  41449538.8 |
  41462126.6 |
  41474714.4 |
  41487302.3 |########################################
  41499890.1 |####################
  41512477.9 |
  41525065.7 |####################
  41537653.5 |####################
  41550241.3 |
  41562829.1 |
  41575417.0 |
  41588004.8 |
  41600592.6 |
  41613180.4 |
  41625768.2 |
  41638356.0 |
  41650943.8 |
  41663531.6 |
  41676119.5 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 14265026.7-30550748.6 ns)
  14265026.7 |########################################
  15079312.8 |
  15893598.9 |
  16707885.0 |########################################
  17522171.1 |########################################
  18336457.2 |
  19150743.3 |########################################
  19965029.3 |
  20779315.4 |
  21593601.5 |
  22407887.6 |
  23222173.7 |########################################
  24036459.8 |
  24850745.9 |########################################
  25665032.0 |
  26479318.1 |
  27293604.2 |
  28107890.3 |########################################
  28922176.4 |
  29736462.5 |
  (0 below, 1 above range)

evs_std (n=8, range 41662019.0-44291644.0 ns)
  41662019.0 |########################################
  41793500.3 |#############
  41924981.5 |
  42056462.8 |
  42187944.0 |
  42319425.3 |
  42450906.5 |
  42582387.8 |
  42713869.0 |
  42845350.3 |
  42976831.5 |
  43108312.8 |
  43239794.0 |
  43371275.3 |
  43502756.5 |
  43634237.8 |
  43765719.0 |
  43897200.3 |
  44028681.5 |
  44160162.8 |##########################
  (0 below, 2 above range)

evs_std_par (n=8, range 16268817.4-29130197.5 ns)
  16268817.4 |####################
  16911886.4 |
  17554955.4 |
  18198024.4 |
  18841093.4 |
  19484162.4 |####################
  20127231.4 |
  20770300.4 |
  21413369.4 |########################################
  22056438.4 |
  22699507.4 |
  23342576.4 |
  23985645.4 |
  24628714.4 |
  25271783.5 |
  25914852.5 |####################
  26557921.5 |####################
  27200990.5 |
  27844059.5 |####################
  28487128.5 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=162.6% of algo (FFI overhead may distort results)
- evs_engine_par: CV=37.0% (high variance, measurements may be unstable)
- evs_engine_par: bridge=314.7% of algo (FFI overhead may distort results)
- evs_std: bridge=158.0% of algo (FFI overhead may distort results)
- evs_std_par: CV=22.1% (high variance, measurements may be unstable)
- evs_std_par: bridge=284.2% of algo (FFI overhead may distort results)
