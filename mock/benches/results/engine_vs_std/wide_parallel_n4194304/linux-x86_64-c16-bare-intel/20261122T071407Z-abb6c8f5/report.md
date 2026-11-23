# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (41.79 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_std_par at 16.35 ms).


### evs_std_par beats baseline by 61% (significant)

evs_std_par is -25.43 ms (61%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std is an outlier: 2.6x slower than the field

evs_std (41.79 ms) is 2.6x the fastest (16.35 ms), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_engine, evs_std} (133% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_engine, evs_std} with a 133% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 16352220.1 ns median (-60.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.56x (fastest 16352220.1 ns, slowest 41792011.0 ns)

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
| Counter frequency | 2112761676 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 41737810ns | 41684894ns | 41598638ns | 41643895ns | 41843039ns | -0.10% |
| evs_engine_par | 17957375ns | 17911692ns | 15956120ns | 16917150ns | 19237858ns | -57.02% |
| evs_std | 41779225ns | 41793698ns | 41659778ns | 41769657ns | 41816263ns | base |
| evs_std_par | 16392419ns | 16355596ns | 16292036ns | 16319183ns | 16472443ns | -60.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 41736729ns | 41597643ns | 41841955ns | -0.10% | 0.100 |
| evs_engine_par | 17954661ns | 15953604ns | 19235009ns | -57.02% | 0.234 |
| evs_std | 41777392ns | 41658338ns | 41814364ns | base | 0.100 |
| evs_std_par | 16389127ns | 16288584ns | 16469364ns | -60.77% | 0.256 |

## Performance model

- Peak throughput: 0.263 Gops/s (evs_engine_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.101 | 38.3% |
| evs_engine_par | 0.234 | 89.1% |
| evs_std | 0.100 | 38.2% |
| evs_std_par | 0.256 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 41737810ns | 41737810ns | -0.10% |
| evs_engine_par | 17957375ns | 17957375ns | -57.02% |
| evs_std | 41779225ns | 41779225ns | base |
| evs_std_par | 16392419ns | 16392419ns | -60.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 41792011ns | base | --- | [41750333, 41815036] | --- | --- | --- | --- |
| evs_engine | 41683832ns | no significant difference | [-159754, +79812]ns | [41622798, 41851997] | no | 0.7266 | 0.7266 | 0 |
| evs_engine_par | 17908905ns | -23878812.1ns (-57.1%) | [-25488590, -22051996]ns | [16326446, 19749474] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 16352220ns | -25425516.2ns (-60.8%) | [-25516392, -25295854]ns | [16291727, 16449123] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 41787717ns | -0.2% | -57.1% | -61.0% |
| 1 | 2 | 41792011ns | -0.3% | -57.7% | -60.7% |

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
| evs_engine | 67493285.6ns | 41736729.4ns | 161.7% | HIGH |
| evs_engine_par | 68866116.4ns | 17954661.0ns | 383.6% | HIGH |
| evs_std | 67590057.2ns | 41777392.1ns | 161.8% | HIGH |
| evs_std_par | 68984016.1ns | 16389127.3ns | 420.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 41597643.4-41841955.1 ns)
  41597643.4 |####################
  41609859.0 |
  41622074.6 |########################################
  41634290.1 |
  41646505.7 |
  41658721.3 |
  41670936.9 |####################
  41683152.5 |
  41695368.1 |####################
  41707583.6 |
  41719799.2 |
  41732014.8 |####################
  41744230.4 |
  41756446.0 |
  41768661.6 |
  41780877.1 |
  41793092.7 |
  41805308.3 |
  41817523.9 |
  41829739.5 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 15953604.0-19235008.8 ns)
  15953604.0 |########################################
  16117674.2 |
  16281744.5 |########################################
  16445814.7 |
  16609884.9 |
  16773955.2 |
  16938025.4 |########################################
  17102095.7 |
  17266165.9 |
  17430236.1 |########################################
  17594306.4 |
  17758376.6 |
  17922446.9 |
  18086517.1 |
  18250587.3 |########################################
  18414657.6 |
  18578727.8 |
  18742798.0 |
  18906868.3 |########################################
  19070938.5 |
  (0 below, 2 above range)

evs_std (n=8, range 41658338.3-41814363.8 ns)
  41658338.3 |####################
  41666139.6 |
  41673940.9 |
  41681742.1 |
  41689543.4 |
  41697344.7 |
  41705146.0 |
  41712947.2 |
  41720748.5 |
  41728549.8 |
  41736351.1 |
  41744152.3 |####################
  41751953.6 |
  41759754.9 |
  41767556.2 |####################
  41775357.4 |####################
  41783158.7 |
  41790960.0 |
  41798761.3 |########################################
  41806562.5 |
  (0 below, 2 above range)

evs_std_par (n=8, range 16288584.0-16469364.1 ns)
  16288584.0 |########################################
  16297623.0 |
  16306662.0 |####################
  16315701.0 |
  16324740.0 |
  16333779.0 |####################
  16342818.0 |
  16351857.0 |
  16360896.0 |####################
  16369935.1 |####################
  16378974.1 |
  16388013.1 |
  16397052.1 |
  16406091.1 |
  16415130.1 |
  16424169.1 |
  16433208.1 |
  16442247.1 |####################
  16451286.1 |
  16460325.1 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=162.0% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=384.3% of algo (FFI overhead may distort results)
- evs_std: bridge=161.8% of algo (FFI overhead may distort results)
- evs_std_par: bridge=422.1% of algo (FFI overhead may distort results)
