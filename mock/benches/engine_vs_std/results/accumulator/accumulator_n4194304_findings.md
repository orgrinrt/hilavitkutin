# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 81% faster than the next best (evs_std)

evs_std_par (2.15 ms) leads evs_std (3.90 ms) by 81%, a clear separation rather than a photo finish. CV 3.7%.


### evs_std_par beats baseline by 43% (significant)

evs_std_par is -1.69 ms (43%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 4.4x slower than the field

evs_engine (9.39 ms) is 4.4x the fastest (2.15 ms), well off the pack.


### evs_engine_par shows alternating (throttle bounce) (autocorr -0.50)

evs_engine_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 4.4x the fastest

Fastest evs_std_par (2.15 ms) to slowest evs_engine (9.39 ms): 4.4x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 2147606.8 ns median (-44.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.37x (fastest 2147606.8 ns, slowest 9387104.9 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 9723466ns | 9393837ns | 9313708ns | 9390262ns | 10075808ns | +151.65% |
| evs_engine_par | 3932703ns | 3948554ns | 3357739ns | 3737008ns | 4223216ns | +1.78% |
| evs_std | 3863956ns | 3902292ns | 3501086ns | 3764349ns | 4029380ns | base |
| evs_std_par | 2193923ns | 2152682ns | 2141479ns | 2150387ns | 2239687ns | -43.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 9718332ns | 9309282ns | 10071217ns | +151.82% | 0.432 |
| evs_engine_par | 3927481ns | 3352903ns | 4217927ns | +1.77% | 1.068 |
| evs_std | 3859256ns | 3496629ns | 4024602ns | base | 1.087 |
| evs_std_par | 2188204ns | 2136684ns | 2233544ns | -43.30% | 1.917 |

## Performance model

- Peak throughput: 1.963 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.447 | 22.8% |
| evs_engine_par | 1.064 | 54.2% |
| evs_std | 1.076 | 54.8% |
| evs_std_par | 1.953 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 9723466ns | 9723466ns | +151.65% |
| evs_engine_par | 3932703ns | 3932703ns | +1.78% |
| evs_std | 3863956ns | 3863956ns | base |
| evs_std_par | 2193923ns | 2193923ns | -43.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 3897268ns | base | --- | [3624058, 4048153] | --- | --- | --- | --- |
| evs_engine | 9387105ns | +5709698.9ns (+146.5%) | [+5335676, +6614044]ns | [9383828, 10694093] | YES | 0.0117 | 0.0078 | 0 |
| evs_engine_par | 3943014ns | no significant difference | [-250603, +227221]ns | [3623564, 4246758] | no | 0.7266 | 0.7266 | 0 |
| evs_std_par | 2147607ns | -1694348.1ns (-43.5%) | [-1837454, -1435801]ns | [2144042, 2210699] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 3733962ns | +152.5% | +3.3% | -42.5% |
| 1 | 2 | 4033845ns | +132.6% | -2.3% | -46.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 1/2, lost 1/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 23089064.3ns | 9718331.8ns | 237.6% | HIGH |
| evs_engine_par | 24874013.3ns | 3927480.8ns | 633.3% | HIGH |
| evs_std | 21838662.6ns | 3859255.8ns | 565.9% | HIGH |
| evs_std_par | 23431770.4ns | 2188203.6ns | 1070.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 9309282.1-10071216.9 ns)
  9309282.1 |#############
  9347378.8 |########################################
  9385475.6 |#############
  9423572.3 |
  9461669.1 |#############
  9499765.8 |
  9537862.5 |
  9575959.3 |
  9614056.0 |
  9652152.8 |
  9690249.5 |
  9728346.3 |
  9766443.0 |
  9804539.7 |
  9842636.5 |
  9880733.2 |
  9918830.0 |
  9956926.7 |
  9995023.4 |
  10033120.2 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 3352903.0-4217927.5 ns)
  3352903.0 |####################
  3396154.2 |
  3439405.4 |
  3482656.7 |
  3525907.9 |
  3569159.1 |
  3612410.3 |####################
  3655661.6 |
  3698912.8 |####################
  3742164.0 |
  3785415.2 |
  3828666.5 |####################
  3871917.7 |
  3915168.9 |
  3958420.1 |
  4001671.4 |########################################
  4044922.6 |
  4088173.8 |
  4131425.0 |
  4174676.3 |
  (0 below, 2 above range)

evs_std (n=8, range 3496628.7-4024601.6 ns)
  3496628.7 |########################################
  3523027.3 |
  3549426.0 |
  3575824.6 |
  3602223.3 |########################################
  3628621.9 |
  3655020.6 |
  3681419.2 |
  3707817.9 |
  3734216.5 |
  3760615.2 |
  3787013.8 |########################################
  3813412.5 |
  3839811.1 |########################################
  3866209.7 |
  3892608.4 |
  3919007.0 |
  3945405.7 |########################################
  3971804.3 |
  3998203.0 |########################################
  (0 below, 2 above range)

evs_std_par (n=8, range 2136683.9-2233543.8 ns)
  2136683.9 |#############
  2141526.9 |########################################
  2146369.9 |#############
  2151212.9 |
  2156055.9 |
  2160898.9 |
  2165741.9 |
  2170584.9 |
  2175427.9 |
  2180270.9 |
  2185113.8 |#############
  2189956.8 |
  2194799.8 |
  2199642.8 |
  2204485.8 |
  2209328.8 |#############
  2214171.8 |
  2219014.8 |
  2223857.8 |
  2228700.8 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=254.1% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=628.0% of algo (FFI overhead may distort results)
- evs_std: bridge=561.7% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1111.8% of algo (FFI overhead may distort results)
