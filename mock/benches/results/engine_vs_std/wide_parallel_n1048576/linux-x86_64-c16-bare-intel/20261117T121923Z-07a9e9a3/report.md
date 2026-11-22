# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine is an outlier: 2.3x slower than the field

evs_engine (10.97 ms) is 2.3x the fastest (4.85 ms), well off the pack.


### evs_engine_par is fastest but the noisiest (CV 84.2%)

evs_engine_par wins on median (4.85 ms) yet has the highest variance (CV 84.2%), while evs_engine is the steadiest (CV 1.9%, 10.97 ms).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_engine_par} vs {evs_std_par, evs_std, evs_engine} (75% apart)

The field splits into a fast tier {evs_engine_par} and a slow tier {evs_std_par, evs_std, evs_engine} with a 75% jump between them - a qualitative difference, not a gradient.


### evs_std_par is inconsistent: worst-20% is 3.0x its best-20%

evs_std_par's best 20% of batches run at 4.44 ms but its worst 20% at 13.11 ms (3.0x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: evs_engine_par at 4849828.2 ns median (-54.8% vs baseline)
- Spread: 2.26x (fastest 4849828.2 ns, slowest 10972014.7 ns)

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
| evs_engine | 10883318ns | 10973225ns | 10377752ns | 10876691ns | 11014680ns | +1.36% |
| evs_engine_par | 7296385ns | 4852467ns | 4272732ns | 4474382ns | 10168801ns | -32.05% |
| evs_std | 10737090ns | 10720786ns | 10437335ns | 10467313ns | 11014362ns | base |
| evs_std_par | 9470597ns | 8508091ns | 4442732ns | 6287965ns | 13114538ns | -11.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 10882306ns | 10376863ns | 11013698ns | +1.37% | 0.096 |
| evs_engine_par | 7293834ns | 4270036ns | 10166159ns | -32.06% | 0.144 |
| evs_std | 10735459ns | 10435870ns | 11012765ns | base | 0.098 |
| evs_std_par | 9467200ns | 4439511ns | 13111083ns | -11.81% | 0.111 |

## Performance model

- Peak throughput: 0.246 Gops/s (evs_engine_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.096 | 38.9% |
| evs_engine_par | 0.216 | 88.0% |
| evs_std | 0.098 | 39.8% |
| evs_std_par | 0.123 | 50.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 10883318ns | 10883318ns | +1.36% |
| evs_engine_par | 7296385ns | 7296385ns | -32.05% |
| evs_std | 10737090ns | 10737090ns | base |
| evs_std_par | 9470597ns | 9470597ns | -11.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 10719155ns | base | --- | [10445405, 11015434] | --- | --- | --- | --- |
| evs_engine | 10972015ns | no significant difference | [-233161, +557245]ns | [10737430, 11033860] | no | 1.0000 | 1.0000 | 0 |
| evs_engine_par | 4849828ns | no significant difference | [-6624943, +2233366]ns | [4336752, 12708087] | no | 0.4336 | 0.2891 | 0 |
| evs_std_par | 8504569ns | no significant difference | [-6575922, +4827030]ns | [4469583, 15272434] | no | 0.4336 | 0.2891 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 10993012ns | -1.5% | -59.9% | -48.1% |
| 1 | 2 | 10460063ns | +5.3% | -3.8% | +19.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 17127084.5ns | 10882305.5ns | 157.4% | HIGH |
| evs_engine_par | 17351188.6ns | 7293833.5ns | 237.9% | HIGH |
| evs_std | 17142860.1ns | 10735458.7ns | 159.7% | HIGH |
| evs_std_par | 17429191.0ns | 9467199.7ns | 184.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 10376862.9-11013698.1 ns)
  10376862.9 |####################
  10408704.7 |
  10440546.4 |
  10472388.2 |
  10504229.9 |
  10536071.7 |
  10567913.5 |
  10599755.2 |
  10631597.0 |
  10663438.8 |
  10695280.5 |
  10727122.3 |####################
  10758964.0 |
  10790805.8 |
  10822647.6 |
  10854489.3 |
  10886331.1 |
  10918172.8 |####################
  10950014.6 |########################################
  10981856.4 |####################
  (0 below, 2 above range)

evs_engine_par (n=8, range 4270035.6-10166158.7 ns)
  4270035.6 |########################################
  4564841.8 |#############
  4859647.9 |#############
  5154454.1 |
  5449260.2 |
  5744066.4 |
  6038872.5 |
  6333678.7 |
  6628484.8 |
  6923291.0 |
  7218097.1 |#############
  7512903.3 |
  7807709.4 |
  8102515.6 |
  8397321.7 |
  8692127.9 |
  8986934.0 |
  9281740.2 |
  9576546.3 |
  9871352.5 |
  (0 below, 2 above range)

evs_std (n=8, range 10435869.8-11012764.7 ns)
  10435869.8 |########################################
  10464714.5 |########################################
  10493559.3 |
  10522404.0 |
  10551248.8 |
  10580093.5 |
  10608938.3 |
  10637783.0 |
  10666627.8 |
  10695472.5 |
  10724317.3 |
  10753162.0 |
  10782006.8 |
  10810851.5 |
  10839696.2 |
  10868541.0 |
  10897385.7 |
  10926230.5 |
  10955075.2 |########################################
  10983920.0 |
  (0 below, 2 above range)

evs_std_par (n=8, range 4439511.1-13111082.8 ns)
  4439511.1 |########################################
  4873089.7 |
  5306668.3 |
  5740246.9 |
  6173825.4 |
  6607404.0 |####################
  7040982.6 |####################
  7474561.2 |
  7908139.8 |
  8341718.4 |
  8775296.9 |
  9208875.5 |####################
  9642454.1 |####################
  10076032.7 |
  10509611.3 |
  10943189.9 |
  11376768.5 |
  11810347.0 |
  12243925.6 |
  12677504.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=156.3% of algo (FFI overhead may distort results)
- evs_engine_par: CV=56.0% (high variance, measurements may be unstable)
- evs_engine_par: bridge=357.2% of algo (FFI overhead may distort results)
- evs_std: bridge=160.3% of algo (FFI overhead may distort results)
- evs_std_par: CV=48.1% (high variance, measurements may be unstable)
- evs_std_par: bridge=204.8% of algo (FFI overhead may distort results)
