# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 12% faster than the next best (evs_engine_par)

evs_std_par (102.03 us) leads evs_engine_par (114.25 us) by 12%, a clear separation rather than a photo finish. CV 2.1%.


### evs_std_par beats baseline by 44% (significant)

evs_std_par is -81.17 us (44%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (60% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 60% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 102033.4 ns median (-44.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 102033.4 ns, slowest 194991.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 196039ns | 195679ns | 194533ns | 195072ns | 197140ns | +6.19% |
| evs_engine_par | 121412ns | 116371ns | 112492ns | 114822ns | 128585ns | -34.23% |
| evs_std | 184616ns | 184579ns | 183392ns | 184033ns | 185358ns | base |
| evs_std_par | 103883ns | 103529ns | 100025ns | 102933ns | 105560ns | -43.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 195044ns | 193983ns | 195681ns | +6.27% | 0.336 |
| evs_engine_par | 119767ns | 110058ns | 127648ns | -34.75% | 0.547 |
| evs_std | 183541ns | 183142ns | 183925ns | base | 0.357 |
| evs_std_par | 102391ns | 99600ns | 103717ns | -44.21% | 0.640 |

## Performance model

- Peak throughput: 0.658 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.336 | 51.1% |
| evs_engine_par | 0.574 | 87.2% |
| evs_std | 0.358 | 54.4% |
| evs_std_par | 0.642 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 196039ns | 196039ns | +6.19% |
| evs_engine_par | 121412ns | 121412ns | -34.23% |
| evs_std | 184616ns | 184616ns | base |
| evs_std_par | 103883ns | 103883ns | -43.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 183175ns | base | --- | [183150, 184183] | --- | --- | --- | --- |
| evs_engine | 194992ns | +11370.8ns (+6.2%) | [+10442, +12333]ns | [194358, 195533] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 114246ns | -69620.9ns (-38.0%) | [-72758, -52400]ns | [110392, 130783] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 102033ns | -81170.9ns (-44.3%) | [-83567, -80358]ns | [101025, 102792] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 183175ns | +6.3% | -29.4% | -44.3% |
| 1 | 2 | 183204ns | +6.7% | -38.9% | -44.3% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 1413767.7ns | 195043.8ns | 724.8% | HIGH |
| evs_engine_par | 1454044.8ns | 119766.7ns | 1214.1% | HIGH |
| evs_std | 1414316.7ns | 183540.6ns | 770.6% | HIGH |
| evs_std_par | 1423030.2ns | 102390.6ns | 1389.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 193983.3-195681.2 ns)
  193983.3 |####################
  194068.2 |
  194153.1 |
  194238.0 |
  194322.9 |####################
  194407.8 |
  194492.7 |
  194577.6 |####################
  194662.5 |####################
  194747.4 |
  194832.3 |
  194917.2 |
  195002.1 |
  195087.0 |
  195171.9 |
  195256.8 |####################
  195341.7 |
  195426.6 |
  195511.5 |########################################
  195596.4 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 110058.3-127647.9 ns)
  110058.3 |########################################
  110937.8 |
  111817.3 |
  112696.7 |########################################
  113576.2 |
  114455.7 |####################
  115335.2 |
  116214.7 |
  117094.1 |
  117973.6 |
  118853.1 |
  119732.6 |
  120612.1 |
  121491.5 |
  122371.0 |
  123250.5 |
  124130.0 |
  125009.5 |
  125888.9 |
  126768.4 |
  (0 below, 3 above range)

evs_std (n=8, range 183141.7-183925.0 ns)
  183141.7 |########################################
  183180.9 |##########
  183220.0 |##########
  183259.2 |
  183298.4 |
  183337.5 |
  183376.7 |
  183415.9 |
  183455.0 |
  183494.2 |
  183533.4 |
  183572.5 |
  183611.7 |
  183650.8 |
  183690.0 |
  183729.2 |
  183768.3 |
  183807.5 |
  183846.7 |
  183885.8 |
  (0 below, 2 above range)

evs_std_par (n=8, range 99600.0-103716.7 ns)
  99600.0 |########################################
  99805.8 |
  100011.7 |
  100217.5 |
  100423.3 |
  100629.2 |
  100835.0 |########################################
  101040.8 |
  101246.7 |
  101452.5 |
  101658.3 |########################################
  101864.2 |########################################
  102070.0 |########################################
  102275.8 |########################################
  102481.7 |
  102687.5 |########################################
  102893.3 |
  103099.2 |
  103305.0 |
  103510.8 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=724.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=1272.4% of algo (FFI overhead may distort results)
- evs_std: bridge=771.8% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1392.6% of algo (FFI overhead may distort results)
