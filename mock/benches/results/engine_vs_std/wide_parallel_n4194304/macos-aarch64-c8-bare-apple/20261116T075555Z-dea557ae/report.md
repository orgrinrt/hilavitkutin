# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 66% (significant)

evs_std_par is -7.98 ms (66%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 3.0x slower than the field

evs_engine (12.16 ms) is 3.0x the fastest (4.10 ms), well off the pack.


### evs_std_par is fastest but the noisiest (CV 13.7%)

evs_std_par wins on median (4.10 ms) yet has the highest variance (CV 13.7%), while evs_std is the steadiest (CV 0.2%, 12.08 ms).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (172% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 172% jump between them - a qualitative difference, not a gradient.


## Key findings

- Fastest: evs_std_par at 4098850.0 ns median (-66.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.97x (fastest 4098850.0 ns, slowest 12164137.5 ns)

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
| evs_engine | 12203703ns | 12167625ns | 12103925ns | 12146717ns | 12271388ns | +1.03% |
| evs_engine_par | 4389256ns | 4445925ns | 4009892ns | 4210111ns | 4618456ns | -63.66% |
| evs_std | 12079212ns | 12082175ns | 12027167ns | 12068736ns | 12100081ns | base |
| evs_std_par | 4251129ns | 4102767ns | 3619092ns | 3917019ns | 4659721ns | -64.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 12200174ns | 12100775ns | 12267648ns | +1.03% | 0.344 |
| evs_engine_par | 4385835ns | 4005975ns | 4615073ns | -63.68% | 0.956 |
| evs_std | 12075677ns | 12022825ns | 12096319ns | base | 0.347 |
| evs_std_par | 4247597ns | 3616417ns | 4656242ns | -64.83% | 0.987 |

## Performance model

- Peak throughput: 1.160 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.345 | 29.7% |
| evs_engine_par | 0.944 | 81.4% |
| evs_std | 0.347 | 29.9% |
| evs_std_par | 1.023 | 88.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 12203703ns | 12203703ns | +1.03% |
| evs_engine_par | 4389256ns | 4389256ns | -63.66% |
| evs_std | 12079212ns | 12079212ns | base |
| evs_std_par | 4251129ns | 4251129ns | -64.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 12079142ns | base | --- | [12051017, 12090450] | --- | --- | --- | --- |
| evs_engine | 12164138ns | +72012.5ns (+0.6%) | [+41808, +238717]ns | [12132083, 12289733] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 4442779ns | -7659495.8ns (-63.4%) | [-8016850, -7361200]ns | [4023542, 4689817] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 4098850ns | -7981858.3ns (-66.1%) | [-8231933, -7486708]ns | [3819083, 4536117] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 12073150ns | +0.8% | -64.8% | -64.0% |
| 1 | 2 | 12086612ns | +1.0% | -62.3% | -67.2% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 93367514.6ns | 12200173.9ns | 765.3% | HIGH |
| evs_engine_par | 93271601.0ns | 4385835.4ns | 2126.7% | HIGH |
| evs_std | 93424159.4ns | 12075677.1ns | 773.7% | HIGH |
| evs_std_par | 93543507.3ns | 4247596.9ns | 2202.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 12100775.0-12267647.9 ns)
  12100775.0 |####################
  12109118.6 |
  12117462.3 |
  12125805.9 |####################
  12134149.6 |
  12142493.2 |########################################
  12150836.9 |
  12159180.5 |
  12167524.2 |
  12175867.8 |####################
  12184211.4 |
  12192555.1 |
  12200898.7 |
  12209242.4 |
  12217586.0 |
  12225929.7 |
  12234273.3 |
  12242617.0 |
  12250960.6 |
  12259304.3 |####################
  (0 below, 2 above range)

evs_engine_par (n=8, range 4005975.0-4615072.9 ns)
  4005975.0 |########################################
  4036429.9 |
  4066884.8 |
  4097339.7 |
  4127794.6 |
  4158249.5 |####################
  4188704.4 |
  4219159.3 |
  4249614.2 |
  4280069.1 |
  4310524.0 |
  4340978.9 |
  4371433.8 |
  4401888.7 |####################
  4432343.5 |
  4462798.4 |####################
  4493253.3 |
  4523708.2 |####################
  4554163.1 |
  4584618.0 |
  (0 below, 2 above range)

evs_std (n=8, range 12022825.0-12096318.8 ns)
  12022825.0 |####################
  12026499.7 |
  12030174.4 |
  12033849.1 |
  12037523.8 |
  12041198.4 |
  12044873.1 |
  12048547.8 |####################
  12052222.5 |
  12055897.2 |
  12059571.9 |
  12063246.6 |
  12066921.2 |
  12070595.9 |####################
  12074270.6 |####################
  12077945.3 |
  12081620.0 |####################
  12085294.7 |
  12088969.4 |########################################
  12092644.1 |
  (0 below, 1 above range)

evs_std_par (n=8, range 3616416.7-4656241.7 ns)
  3616416.7 |########################################
  3668407.9 |
  3720399.2 |
  3772390.4 |########################################
  3824381.7 |########################################
  3876372.9 |
  3928364.2 |
  3980355.4 |
  4032346.7 |########################################
  4084337.9 |
  4136329.2 |########################################
  4188320.4 |
  4240311.7 |
  4292302.9 |
  4344294.2 |
  4396285.4 |########################################
  4448276.7 |
  4500267.9 |########################################
  4552259.2 |
  4604250.4 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=767.2% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2100.0% of algo (FFI overhead may distort results)
- evs_std: bridge=772.4% of algo (FFI overhead may distort results)
- evs_std_par: bridge=2282.0% of algo (FFI overhead may distort results)
