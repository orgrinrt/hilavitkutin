# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 3053% faster than the next best (evs_engine)

evs_std (412 ns) leads evs_engine (13.00 us) by 3053%, a clear separation rather than a photo finish. CV 1.4%.


### evs_engine_par is an outlier: 129.2x slower than the field

evs_engine_par (53.30 us) is 129.2x the fastest (412 ns), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (412 ns median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_engine, evs_std_par, evs_engine_par} (3053% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_engine, evs_std_par, evs_engine_par} with a 3053% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 129.2x the fastest

Fastest evs_std (412 ns) to slowest evs_engine_par (53.30 us): 129.2x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 412.5 ns median
- 3 variants significantly slower than baseline
- Spread: 129.20x (fastest 412.5 ns, slowest 53295.8 ns)

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
| evs_engine | 13483ns | 13104ns | 13025ns | 13069ns | 13908ns | +2509.70% |
| evs_engine_par | 54574ns | 53692ns | 48317ns | 52547ns | 57658ns | +10462.79% |
| evs_std | 517ns | 508ns | 492ns | 503ns | 533ns | base |
| evs_std_par | 23094ns | 22667ns | 19025ns | 21414ns | 25371ns | +4369.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 13375ns | 12925ns | 13790ns | +3150.60% | 0.306 |
| evs_engine_par | 54215ns | 48050ns | 57240ns | +13076.07% | 0.076 |
| evs_std | 411ns | 400ns | 417ns | base | 9.955 |
| evs_std_par | 22730ns | 17983ns | 25215ns | +5424.25% | 0.180 |

## Performance model

- Peak throughput: 10.240 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.315 | 3.1% |
| evs_engine_par | 0.077 | 0.8% |
| evs_std | 9.930 | 97.0% |
| evs_std_par | 0.183 | 1.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 13483ns | 13483ns | +2509.70% |
| evs_engine_par | 54574ns | 54574ns | +10462.79% |
| evs_std | 517ns | 517ns | base |
| evs_std_par | 23094ns | 23094ns | +4369.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 412ns | base | --- | [408, 417] | --- | --- | --- | --- |
| evs_engine | 13004ns | +12595.8ns (+3053.5%) | [+12542, +13625]ns | [12950, 14033] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 53296ns | +52883.3ns (+12820.2%) | [+50992, +57700]ns | [51400, 58117] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 22362ns | +21950.0ns (+5321.2%) | [+19533, +25150]ns | [19942, 25550] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 417ns | +3140.7% | +13041.0% | +5209.6% |
| 1 | 2 | 408ns | +3078.8% | +12953.1% | +5694.4% |

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
| evs_engine | 21894.8ns | 13375.0ns | 163.7% | HIGH |
| evs_engine_par | 25657.3ns | 54214.6ns | 47.3% | HIGH |
| evs_std | 21893.8ns | 411.5ns | 5321.0% | HIGH |
| evs_std_par | 24613.6ns | 22730.2ns | 108.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 12925.0-13789.5 ns)
  12925.0 |########################################
  12968.2 |##########################
  13011.5 |
  13054.7 |
  13097.9 |
  13141.1 |
  13184.4 |
  13227.6 |
  13270.8 |
  13314.0 |
  13357.3 |
  13400.5 |
  13443.7 |
  13487.0 |
  13530.2 |
  13573.4 |
  13616.6 |
  13659.9 |
  13703.1 |
  13746.3 |
  (0 below, 3 above range)

evs_engine_par (n=8, range 48050.0-57239.6 ns)
  48050.0 |#############
  48509.5 |
  48969.0 |
  49428.4 |
  49887.9 |
  50347.4 |
  50806.9 |
  51266.4 |#############
  51725.8 |#############
  52185.3 |
  52644.8 |
  53104.3 |########################################
  53563.8 |
  54023.2 |
  54482.7 |
  54942.2 |
  55401.7 |
  55861.2 |
  56320.6 |
  56780.1 |
  (0 below, 2 above range)

evs_std (n=8, range 400.0-416.7 ns)
    400.0 |#############
    400.8 |
    401.7 |
    402.5 |
    403.3 |
    404.2 |
    405.0 |
    405.8 |
    406.7 |
    407.5 |########################################
    408.4 |
    409.2 |
    410.0 |
    410.9 |
    411.7 |
    412.5 |
    413.4 |
    414.2 |
    415.0 |
    415.9 |
  (0 below, 4 above range)

evs_std_par (n=8, range 17983.3-25214.6 ns)
  17983.3 |########################################
  18344.9 |
  18706.4 |
  19068.0 |
  19429.6 |
  19791.1 |########################################
  20152.7 |
  20514.2 |
  20875.8 |
  21237.4 |########################################
  21598.9 |########################################
  21960.5 |
  22322.1 |
  22683.6 |########################################
  23045.2 |
  23406.8 |
  23768.3 |########################################
  24129.9 |
  24491.4 |
  24853.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=168.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=48.7% of algo (FFI overhead may distort results)
- evs_std: bridge=5307.1% of algo (FFI overhead may distort results)
- evs_std_par: bridge=108.5% of algo (FFI overhead may distort results)
