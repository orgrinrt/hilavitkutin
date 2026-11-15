# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std_startup has the worst median (1.73 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine_startup at 1.63 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (99.01 us) is smaller than the fastest variant's own run-to-run std-dev (167.46 us); the ranking is inside the noise.


## Key findings

- Fastest: evs_engine_startup at 1628507.9 ns median (-5.7% vs baseline)
- Spread: 1.06x (fastest 1628507.9 ns, slowest 1727522.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010003 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 1678467ns | 1633579ns | 1460462ns | 1583682ns | 1804057ns | -10.49% |
| evs_std_startup | 1875172ns | 1732874ns | 1488416ns | 1579641ns | 2193510ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1673694ns | 1456256ns | 1798922ns | -10.47% | 2.506 |
| evs_std_startup | 1869372ns | 1480390ns | 2187667ns | base | 2.244 |

## Performance model

- Peak throughput: 2.880 Gops/s (evs_engine_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 2.576 | 89.4% |
| evs_std_startup | 2.428 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1678467ns | 1678467ns | -10.49% |
| evs_std_startup | 1875172ns | 1875172ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 1727522ns | base | --- | [1541662, 2144910] | --- | --- | --- | --- |
| evs_engine_startup | 1628508ns | no significant difference | [-688654, +343390]ns | [1517926, 1919296] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 1690430ns | -3.7% |
| 1 | 2 | 1877321ns | -14.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 48742650.9ns | 1673693.8ns | 2912.3% | HIGH |
| evs_std_startup | 47194341.2ns | 1869372.3ns | 2524.6% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1456256.1-1798921.9 ns)
  1456256.1 |####################
  1473389.4 |
  1490522.7 |
  1507656.0 |####################
  1524789.3 |
  1541922.6 |
  1559055.8 |
  1576189.1 |####################
  1593322.4 |
  1610455.7 |
  1627589.0 |########################################
  1644722.3 |
  1661855.6 |
  1678988.9 |####################
  1696122.2 |
  1713255.4 |
  1730388.7 |
  1747522.0 |
  1764655.3 |
  1781788.6 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 1480390.5-2187666.7 ns)
  1480390.5 |####################
  1515754.3 |####################
  1551118.1 |####################
  1586481.9 |####################
  1621845.7 |
  1657209.6 |
  1692573.4 |
  1727937.2 |
  1763301.0 |
  1798664.8 |
  1834028.6 |####################
  1869392.4 |
  1904756.2 |
  1940120.0 |
  1975483.9 |
  2010847.7 |
  2046211.5 |
  2081575.3 |
  2116939.1 |########################################
  2152302.9 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=3029.1% of algo (FFI overhead may distort results)
- evs_std_startup: CV=20.2% (high variance, measurements may be unstable)
- evs_std_startup: bridge=2691.8% of algo (FFI overhead may distort results)
