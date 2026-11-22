# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 12% faster than the next best (evs_std)

evs_std_par (93.80 us) leads evs_std (105.34 us) by 12%, a clear separation rather than a photo finish. CV 2.7%.


### evs_engine is an outlier: 7.9x slower than the field

evs_engine (736.86 us) is 7.9x the fastest (93.80 us), well off the pack.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 7.9x the fastest

Fastest evs_std_par (93.80 us) to slowest evs_engine (736.86 us): 7.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 93804.2 ns median (-11.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.86x (fastest 93804.2 ns, slowest 736862.5 ns)

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
| evs_engine | 739082ns | 738312ns | 724283ns | 733167ns | 747219ns | +589.87% |
| evs_engine_par | 243052ns | 236812ns | 231375ns | 234033ns | 252735ns | +126.87% |
| evs_std | 107133ns | 107125ns | 106375ns | 106917ns | 107485ns | base |
| evs_std_par | 94926ns | 95925ns | 90083ns | 93436ns | 97254ns | -11.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 737428ns | 722558ns | 745623ns | +600.01% | 1.422 |
| evs_engine_par | 241144ns | 230158ns | 250737ns | +128.91% | 4.348 |
| evs_std | 105345ns | 105183ns | 105431ns | base | 9.954 |
| evs_std_par | 93049ns | 88892ns | 95017ns | -11.67% | 11.269 |

## Performance model

- Peak throughput: 11.796 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.423 | 12.1% |
| evs_engine_par | 4.473 | 37.9% |
| evs_std | 9.954 | 84.4% |
| evs_std_par | 11.178 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 739082ns | 739082ns | +589.87% |
| evs_engine_par | 243052ns | 243052ns | +126.87% |
| evs_std | 107133ns | 107133ns | base |
| evs_std_par | 94926ns | 94926ns | -11.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 105342ns | base | --- | [105267, 105442] | --- | --- | --- | --- |
| evs_engine | 736862ns | +631404.2ns (+599.4%) | [+623442, +635542]ns | [728825, 740808] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 234429ns | +129095.8ns (+122.5%) | [+125658, +153958]ns | [230842, 259433] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 93804ns | -11487.5ns (-10.9%) | [-15933, -9525]ns | [89492, 95658] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 105433ns | +598.9% | +135.0% | -11.5% |
| 1 | 2 | 105275ns | +598.5% | +120.9% | -10.9% |

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
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 5643609.4ns | 737428.1ns | 765.3% | HIGH |
| evs_engine_par | 5768720.9ns | 241143.7ns | 2392.2% | HIGH |
| evs_std | 5643482.3ns | 105344.8ns | 5357.2% | HIGH |
| evs_std_par | 5668132.3ns | 93049.0ns | 6091.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 722558.3-745622.9 ns)
  722558.3 |####################
  723711.5 |
  724864.8 |
  726018.0 |
  727171.2 |
  728324.5 |####################
  729477.7 |####################
  730630.9 |
  731784.1 |
  732937.4 |
  734090.6 |
  735243.8 |####################
  736397.1 |
  737550.3 |####################
  738703.5 |
  739856.8 |########################################
  741010.0 |
  742163.2 |
  743316.4 |
  744469.7 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 230158.3-250737.5 ns)
  230158.3 |########################################
  231187.3 |
  232216.2 |########################################
  233245.2 |
  234274.1 |
  235303.1 |####################
  236332.1 |
  237361.0 |
  238390.0 |
  239418.9 |
  240447.9 |
  241476.8 |
  242505.8 |
  243534.8 |
  244563.7 |####################
  245592.7 |
  246621.6 |
  247650.6 |
  248679.6 |
  249708.5 |
  (0 below, 2 above range)

evs_std (n=8, range 105183.3-105431.2 ns)
  105183.3 |########################################
  105195.7 |
  105208.1 |
  105220.5 |
  105232.9 |
  105245.3 |
  105257.7 |########################################
  105270.1 |
  105282.5 |########################################
  105294.9 |########################################
  105307.3 |
  105319.7 |
  105332.1 |
  105344.5 |
  105356.9 |
  105369.3 |
  105381.7 |########################################
  105394.1 |
  105406.5 |
  105418.9 |########################################
  (0 below, 2 above range)

evs_std_par (n=8, range 88891.7-95016.7 ns)
  88891.7 |########################################
  89197.9 |########################################
  89504.2 |
  89810.4 |
  90116.7 |
  90422.9 |
  90729.2 |
  91035.4 |
  91341.7 |
  91647.9 |
  91954.2 |########################################
  92260.4 |
  92566.7 |
  92872.9 |
  93179.2 |
  93485.4 |########################################
  93791.7 |########################################
  94097.9 |########################################
  94404.2 |
  94710.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=765.8% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2431.1% of algo (FFI overhead may distort results)
- evs_std: bridge=5357.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=6041.0% of algo (FFI overhead may distort results)
