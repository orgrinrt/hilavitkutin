# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 447% faster than the next best (evs_std_par)

evs_std (6.47 us) leads evs_std_par (35.35 us) by 447%, a clear separation rather than a photo finish. CV 0.4%.


### evs_engine_par is an outlier: 10.6x slower than the field

evs_engine_par (68.55 us) is 10.6x the fastest (6.47 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (6.47 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_std_par, evs_engine, evs_engine_par} (447% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_std_par, evs_engine, evs_engine_par} with a 447% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 10.6x the fastest

Fastest evs_std (6.47 us) to slowest evs_engine_par (68.55 us): 10.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 6466.6 ns median
- 3 variants significantly slower than baseline
- Spread: 10.60x (fastest 6466.6 ns, slowest 68545.8 ns)

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
| evs_engine | 56217ns | 55467ns | 54342ns | 54997ns | 57600ns | +732.97% |
| evs_engine_par | 69686ns | 69442ns | 67967ns | 68381ns | 71096ns | +932.55% |
| evs_std | 6749ns | 6625ns | 6575ns | 6600ns | 6904ns | base |
| evs_std_par | 37926ns | 36754ns | 31208ns | 34881ns | 41890ns | +461.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 55428ns | 54225ns | 56433ns | +756.03% | 1.182 |
| evs_engine_par | 68782ns | 67042ns | 69973ns | +962.28% | 0.953 |
| evs_std | 6475ns | 6450ns | 6494ns | base | 10.121 |
| evs_std_par | 37051ns | 31008ns | 41054ns | +472.22% | 1.769 |

## Performance model

- Peak throughput: 10.161 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.193 | 11.7% |
| evs_engine_par | 0.956 | 9.4% |
| evs_std | 10.134 | 99.7% |
| evs_std_par | 1.854 | 18.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 56217ns | 56217ns | +732.97% |
| evs_engine_par | 69686ns | 69686ns | +932.55% |
| evs_std | 6749ns | 6749ns | base |
| evs_std_par | 37926ns | 37926ns | +461.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 6467ns | base | --- | [6458, 6492] | --- | --- | --- | --- |
| evs_engine | 54925ns | +48450.0ns (+749.2%) | [+47783, +51050]ns | [54242, 57508] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 68546ns | +62070.8ns (+959.9%) | [+60642, +64525]ns | [67125, 70975] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 35346ns | +28887.5ns (+446.7%) | [+26908, +35983]ns | [33358, 42458] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 6479ns | +767.2% | +940.6% | +489.3% |
| 1 | 2 | 6458ns | +745.6% | +985.4% | +447.3% |

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
| evs_engine | 355846.9ns | 55428.1ns | 642.0% | HIGH |
| evs_engine_par | 362004.2ns | 68782.3ns | 526.3% | HIGH |
| evs_std | 352594.8ns | 6475.0ns | 5445.5% | HIGH |
| evs_std_par | 360582.3ns | 37051.0ns | 973.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 54225.0-56433.3 ns)
  54225.0 |########################################
  54335.4 |####################
  54445.8 |
  54556.2 |
  54666.7 |
  54777.1 |####################
  54887.5 |####################
  54997.9 |
  55108.3 |
  55218.7 |
  55329.2 |####################
  55439.6 |
  55550.0 |
  55660.4 |
  55770.8 |
  55881.2 |
  55991.7 |
  56102.1 |
  56212.5 |
  56322.9 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 67041.7-69972.9 ns)
  67041.7 |########################################
  67188.3 |
  67334.8 |
  67481.4 |
  67627.9 |####################
  67774.5 |
  67921.1 |
  68067.6 |
  68214.2 |
  68360.8 |####################
  68507.3 |####################
  68653.9 |
  68800.4 |
  68947.0 |
  69093.6 |####################
  69240.1 |
  69386.7 |
  69533.2 |
  69679.8 |
  69826.4 |
  (0 below, 2 above range)

evs_std (n=8, range 6450.0-6493.8 ns)
   6450.0 |#############
   6452.2 |
   6454.4 |
   6456.6 |########################################
   6458.8 |
   6460.9 |
   6463.1 |
   6465.3 |
   6467.5 |
   6469.7 |
   6471.9 |
   6474.1 |#############
   6476.2 |
   6478.4 |
   6480.6 |
   6482.8 |#############
   6485.0 |
   6487.2 |
   6489.4 |
   6491.6 |#############
  (0 below, 1 above range)

evs_std_par (n=8, range 31008.3-41054.2 ns)
  31008.3 |####################
  31510.6 |
  32012.9 |
  32515.2 |
  33017.5 |####################
  33519.8 |########################################
  34022.1 |
  34524.4 |
  35026.7 |
  35528.9 |
  36031.2 |
  36533.5 |####################
  37035.8 |
  37538.1 |
  38040.4 |####################
  38542.7 |
  39045.0 |
  39547.3 |
  40049.6 |
  40551.9 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=643.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=529.4% of algo (FFI overhead may distort results)
- evs_std: bridge=5448.7% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1021.5% of algo (FFI overhead may distort results)
