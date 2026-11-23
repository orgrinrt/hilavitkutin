# accumulator, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 518% faster than the next best (evs_engine_startup)

evs_std_startup (371.94 us) leads evs_engine_startup (2.30 ms) by 518%, a clear separation rather than a photo finish. CV 40.1%.


### evs_std_startup is fastest but the noisiest (CV 40.1%)

evs_std_startup wins on median (371.94 us) yet has the highest variance (CV 40.1%), while evs_engine_startup is the steadiest (CV 9.0%, 2.30 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (371.94 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 6.2x the fastest

Fastest evs_std_startup (371.94 us) to slowest evs_engine_startup (2.30 ms): 6.2x. The strategy choice matters a lot for this workload.


### evs_std_startup is inconsistent: worst-20% is 2.3x its best-20%

evs_std_startup's best 20% of batches run at 233.38 us but its worst 20% at 532.22 us (2.3x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std_startup) is the fastest at 371940.7 ns median
- 1 variant significantly slower than baseline
- Spread: 6.18x (fastest 371940.7 ns, slowest 2299138.8 ns)

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
| Counter frequency | 2112797379 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 2238016ns | 2300755ns | 1972011ns | 2101742ns | 2406722ns | +480.13% |
| evs_std_startup | 385779ns | 373430ns | 234691ns | 238655ns | 533894ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 2236667ns | 1970706ns | 2405560ns | +482.13% | 1.875 |
| evs_std_startup | 384219ns | 233382ns | 532224ns | base | 10.916 |

## Performance model

- Peak throughput: 17.972 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 1.824 | 10.2% |
| evs_std_startup | 11.277 | 62.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 2238016ns | 2238016ns | +480.13% |
| evs_std_startup | 385779ns | 385779ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 371941ns | base | --- | [235384, 537146] | --- | --- | --- | --- |
| evs_engine_startup | 2299139ns | +1894020.3ns (+509.2%) | [+1497126, +2113480]ns | [2010829, 2350346] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 525424ns | +283.6% |
| 1 | 2 | 236125ns | +891.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 22490479.2ns | 2236666.6ns | 1005.5% | HIGH |
| evs_std_startup | 24923045.1ns | 384219.4ns | 6486.7% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1970706.0-2405559.6 ns)
  1970706.0 |####################
  1992448.7 |####################
  2014191.4 |####################
  2035934.0 |
  2057676.7 |
  2079419.4 |
  2101162.1 |
  2122904.8 |
  2144647.4 |
  2166390.1 |
  2188132.8 |
  2209875.5 |
  2231618.1 |
  2253360.8 |####################
  2275103.5 |
  2296846.2 |
  2318588.9 |########################################
  2340331.5 |####################
  2362074.2 |
  2383816.9 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 233381.6-532223.6 ns)
  233381.6 |########################################
  248323.7 |
  263265.8 |
  278207.9 |
  293150.0 |
  308092.1 |
  323034.2 |
  337976.3 |
  352918.4 |
  367860.5 |
  382802.6 |
  397744.7 |
  412686.8 |
  427628.9 |
  442571.0 |
  457513.1 |
  472455.2 |
  487397.3 |
  502339.4 |####################
  517281.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=980.4% of algo (FFI overhead may distort results)
- evs_std_startup: CV=38.8% (high variance, measurements may be unstable)
- evs_std_startup: bridge=6739.8% of algo (FFI overhead may distort results)
