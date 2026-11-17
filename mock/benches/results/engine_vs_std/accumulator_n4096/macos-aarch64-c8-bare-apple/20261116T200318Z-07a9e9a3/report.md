# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 3025% faster than the next best (evs_engine)

evs_std (417 ns) leads evs_engine (13.02 us) by 3025%, a clear separation rather than a photo finish. CV 0.0%.


### evs_engine_par is an outlier: 127.0x slower than the field

evs_engine_par (52.94 us) is 127.0x the fastest (417 ns), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (417 ns median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_engine, evs_std_par, evs_engine_par} (3025% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_engine, evs_std_par, evs_engine_par} with a 3025% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 127.0x the fastest

Fastest evs_std (417 ns) to slowest evs_engine_par (52.94 us): 127.0x. The strategy choice matters a lot for this workload.


### evs_std_par is inconsistent: worst-20% is 2.3x its best-20%

evs_std_par's best 20% of batches run at 15.41 us but its worst 20% at 35.60 us (2.3x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 416.7 ns median
- 3 variants significantly slower than baseline
- Spread: 127.05x (fastest 416.7 ns, slowest 52941.7 ns)

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
| evs_engine | 13335ns | 13121ns | 12992ns | 13103ns | 13596ns | +2435.07% |
| evs_engine_par | 53866ns | 53300ns | 51100ns | 52061ns | 55910ns | +10139.88% |
| evs_std | 526ns | 512ns | 500ns | 508ns | 546ns | base |
| evs_std_par | 28649ns | 29375ns | 15742ns | 23156ns | 35996ns | +5346.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 13224ns | 12900ns | 13473ns | +3073.50% | 0.310 |
| evs_engine_par | 53450ns | 50550ns | 55510ns | +12726.97% | 0.077 |
| evs_std | 417ns | 417ns | 417ns | base | 9.830 |
| evs_std_par | 28178ns | 15408ns | 35600ns | +6662.21% | 0.145 |

## Performance model

- Peak throughput: 9.830 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.315 | 3.2% |
| evs_engine_par | 0.077 | 0.8% |
| evs_std | 9.830 | 100.0% |
| evs_std_par | 0.144 | 1.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 13335ns | 13335ns | +2435.07% |
| evs_engine_par | 53866ns | 53866ns | +10139.88% |
| evs_std | 526ns | 526ns | base |
| evs_std_par | 28649ns | 28649ns | +5346.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 417ns | base | --- | [417, 417] | --- | --- | --- | --- |
| evs_engine | 13021ns | +12604.1ns (+3024.8%) | [+12575, +13208]ns | [12992, 13625] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 52942ns | +52525.0ns (+12605.0%) | [+50708, +55383]ns | [51125, 55800] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 28400ns | +27983.3ns (+6715.5%) | [+16967, +35350]ns | [17383, 35767] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 417ns | +3045.8% | +12823.0% | +8137.3% |
| 1 | 2 | 417ns | +3024.8% | +12229.0% | +4892.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | 0.000 | ok |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 21907.3ns | 13224.0ns | 165.7% | HIGH |
| evs_engine_par | 25452.1ns | 53450.0ns | 47.6% | HIGH |
| evs_std | 22101.0ns | 416.7ns | 5303.8% | HIGH |
| evs_std_par | 25395.8ns | 28178.1ns | 90.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 12900.0-13472.9 ns)
  12900.0 |####################
  12928.6 |
  12957.3 |
  12985.9 |########################################
  13014.6 |########################################
  13043.2 |
  13071.9 |
  13100.5 |
  13129.2 |
  13157.8 |
  13186.5 |
  13215.1 |####################
  13243.8 |
  13272.4 |
  13301.0 |
  13329.7 |
  13358.3 |
  13387.0 |
  13415.6 |
  13444.3 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 50550.0-55510.4 ns)
  50550.0 |########################################
  50798.0 |
  51046.0 |########################################
  51294.1 |
  51542.1 |########################################
  51790.1 |
  52038.1 |########################################
  52286.1 |
  52534.2 |
  52782.2 |
  53030.2 |
  53278.2 |
  53526.3 |########################################
  53774.3 |
  54022.3 |########################################
  54270.3 |
  54518.3 |
  54766.4 |
  55014.4 |
  55262.4 |
  (0 below, 2 above range)

evs_std_par (n=8, range 15408.3-35600.0 ns)
  15408.3 |########################################
  16417.9 |########################################
  17427.5 |
  18437.1 |
  19446.6 |
  20456.2 |
  21465.8 |
  22475.4 |
  23485.0 |########################################
  24494.6 |
  25504.2 |########################################
  26513.7 |
  27523.3 |
  28532.9 |
  29542.5 |
  30552.1 |########################################
  31561.7 |
  32571.2 |########################################
  33580.8 |
  34590.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=168.2% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=48.2% of algo (FFI overhead may distort results)
- evs_std: bridge=5302.6% of algo (FFI overhead may distort results)
- evs_std_par: CV=30.9% (high variance, measurements may be unstable)
- evs_std_par: bridge=90.5% of algo (FFI overhead may distort results)
