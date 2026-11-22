# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 435% faster than the next best (evs_std_par)

evs_std (6.50 us) leads evs_std_par (34.77 us) by 435%, a clear separation rather than a photo finish. CV 0.5%.


### evs_engine_par is an outlier: 11.1x slower than the field

evs_engine_par (72.27 us) is 11.1x the fastest (6.50 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.50 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_std_par, evs_engine, evs_engine_par} (435% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_std_par, evs_engine, evs_engine_par} with a 435% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 11.1x the fastest

Fastest evs_std (6.50 us) to slowest evs_engine_par (72.27 us): 11.1x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 6495.9 ns median
- 3 variants significantly slower than baseline
- Spread: 11.13x (fastest 6495.9 ns, slowest 72270.9 ns)

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
| evs_engine | 55573ns | 54950ns | 54167ns | 54742ns | 56548ns | +706.62% |
| evs_engine_par | 72333ns | 72871ns | 68475ns | 70536ns | 74646ns | +949.89% |
| evs_std | 6890ns | 6662ns | 6575ns | 6619ns | 7171ns | base |
| evs_std_par | 35803ns | 35588ns | 32583ns | 33806ns | 38106ns | +419.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 55041ns | 53983ns | 55881ns | +746.92% | 1.191 |
| evs_engine_par | 71742ns | 67908ns | 73988ns | +1003.90% | 0.913 |
| evs_std | 6499ns | 6450ns | 6525ns | base | 10.084 |
| evs_std_par | 34977ns | 31525ns | 37292ns | +438.20% | 1.874 |

## Performance model

- Peak throughput: 10.161 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.200 | 11.8% |
| evs_engine_par | 0.907 | 8.9% |
| evs_std | 10.089 | 99.3% |
| evs_std_par | 1.885 | 18.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 55573ns | 55573ns | +706.62% |
| evs_engine_par | 72333ns | 72333ns | +949.89% |
| evs_std | 6890ns | 6890ns | base |
| evs_std_par | 35803ns | 35803ns | +419.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6496ns | base | --- | [6475, 6533] | --- | --- | --- | --- |
| evs_engine | 54612ns | +48137.5ns (+741.1%) | [+47700, +50067]ns | [54208, 56567] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 72271ns | +65783.4ns (+1012.7%) | [+61500, +67908]ns | [67950, 74383] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 34767ns | +28241.7ns (+434.8%) | [+25342, +31608]ns | [31817, 38058] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 6488ns | +744.1% | +999.3% | +418.8% |
| 1 | 2 | 6500ns | +739.7% | +1016.7% | +445.6% |

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
| evs_engine | 352815.6ns | 55040.6ns | 641.0% | HIGH |
| evs_engine_par | 362444.8ns | 71741.7ns | 505.2% | HIGH |
| evs_std | 353119.8ns | 6499.0ns | 5433.5% | HIGH |
| evs_std_par | 361095.8ns | 34977.1ns | 1032.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 53983.3-55881.3 ns)
  53983.3 |####################
  54078.2 |
  54173.1 |####################
  54268.0 |########################################
  54362.9 |
  54457.8 |
  54552.7 |
  54647.6 |
  54742.5 |
  54837.4 |####################
  54932.3 |
  55027.2 |
  55122.1 |####################
  55217.0 |
  55311.9 |
  55406.8 |
  55501.7 |
  55596.6 |
  55691.5 |
  55786.4 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 67908.3-73987.5 ns)
  67908.3 |########################################
  68212.3 |
  68516.2 |
  68820.2 |
  69124.1 |
  69428.1 |
  69732.1 |
  70036.0 |####################
  70340.0 |
  70643.9 |
  70947.9 |
  71251.9 |
  71555.8 |
  71859.8 |####################
  72163.7 |
  72467.7 |####################
  72771.7 |
  73075.6 |####################
  73379.6 |
  73683.5 |
  (0 below, 2 above range)

evs_std (n=8, range 6450.0-6525.0 ns)
   6450.0 |####################
   6453.7 |
   6457.5 |
   6461.2 |
   6465.0 |
   6468.7 |
   6472.5 |########################################
   6476.2 |
   6480.0 |
   6483.7 |
   6487.5 |
   6491.2 |####################
   6495.0 |
   6498.7 |####################
   6502.5 |
   6506.2 |####################
   6510.0 |
   6513.7 |
   6517.5 |
   6521.2 |
  (0 below, 2 above range)

evs_std_par (n=8, range 31525.0-37291.7 ns)
  31525.0 |########################################
  31813.3 |########################################
  32101.7 |
  32390.0 |########################################
  32678.3 |
  32966.7 |
  33255.0 |
  33543.3 |
  33831.7 |
  34120.0 |
  34408.3 |########################################
  34696.7 |########################################
  34985.0 |
  35273.3 |
  35561.7 |
  35850.0 |########################################
  36138.3 |
  36426.7 |
  36715.0 |
  37003.3 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=645.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=500.9% of algo (FFI overhead may distort results)
- evs_std: bridge=5424.9% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1036.3% of algo (FFI overhead may distort results)
