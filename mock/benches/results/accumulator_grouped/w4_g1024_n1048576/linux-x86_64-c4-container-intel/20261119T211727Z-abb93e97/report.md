# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.1x slower than the field

sequential (4.68 ms) is 2.1x the fastest (2.20 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped is fastest but the noisiest (CV 8.9%)

grouped wins on median (2.20 ms) yet has the highest variance (CV 8.9%), while sequential is the steadiest (CV 5.1%, 4.68 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### per_core shows alternating (throttle bounce) (autocorr -0.66)

per_core's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (grouped)

The baseline grouped is the fastest (2.20 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (grouped) is the fastest** at 2196958.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.13x (fastest 2196958.8 ns, slowest 4677941.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 1024 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 2226008ns | 2200282ns | 1918796ns | 2182262ns | 2445234ns | base |
| per_core | 2195743ns | 2241935ns | 1903568ns | 2192430ns | 2346800ns | -1.36% |
| sequential | 4641940ns | 4680026ns | 4283177ns | 4586596ns | 4904337ns | +108.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 2223734ns | 1916160ns | 2443239ns | base | 0.472 |
| per_core | 2193724ns | 1901475ns | 2344960ns | -1.35% | 0.478 |
| sequential | 4639077ns | 4281152ns | 4901726ns | +108.62% | 0.226 |

## Performance model

- Peak throughput: **0.551 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.477 | 86.6% |
| per_core | 0.468 | 84.9% |
| sequential | 0.224 | 40.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 2226008ns | 2226008ns | base |
| per_core | 2195743ns | 2195743ns | -1.36% |
| sequential | 4641940ns | 4641940ns | +108.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 2196959ns | base | --- | [2141512, 2342695] | --- | --- | --- | --- |
| per_core | 2240269ns | no significant difference | [-208308, +208572]ns | [2114144, 2321488] | no | 0.4807 | 0.4807 | 0 |
| sequential | 4677942ns | +2481586.3ns (+113.0%) | [+2248865, +2635646]ns | [4449582, 4848060] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 2330785ns | -3.6% | +85.4% |
| 1 | 2 | 2202523ns | +1.3% | +123.1% |
| 1 | 3 | 2196959ns | +2.2% | +112.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.146 | ok |
| per_core | -0.658 | HIGH- (thermal bounce) |
| sequential | -0.429 | moderate- |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2232384.5ns | 2223733.9ns | 100.4% | HIGH |
| per_core | 2132152.3ns | 2193724.0ns | 97.2% | HIGH |
| sequential | 4654462.1ns | 4639076.6ns | 100.3% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 1916160.5-2443238.8 ns)
  1916160.5 |####################
  1942514.4 |
  1968868.3 |
  1995222.3 |##########
  2021576.2 |
  2047930.1 |
  2074284.0 |
  2100637.9 |##########
  2126991.8 |
  2153345.8 |##########
  2179699.7 |########################################
  2206053.6 |##########
  2232407.5 |
  2258761.4 |
  2285115.3 |
  2311469.3 |##########
  2337823.2 |####################
  2364177.1 |
  2390531.0 |
  2416884.9 |##########
  (1 below, 3 above range)

per_core (n=18, range 1901474.8-2344959.9 ns)
  1901474.8 |
  1923649.1 |
  1945823.3 |
  1967997.6 |##########################
  1990171.8 |
  2012346.1 |
  2034520.4 |
  2056694.6 |#############
  2078868.9 |
  2101043.1 |##########################
  2123217.4 |
  2145391.6 |
  2167565.9 |#############
  2189740.1 |
  2211914.4 |#############
  2234088.6 |########################################
  2256262.9 |
  2278437.1 |#############
  2300611.4 |#############
  2322785.6 |##########################
  (1 below, 3 above range)

sequential (n=18, range 4281151.6-4901726.2 ns)
  4281151.6 |########################################
  4312180.4 |####################
  4343209.1 |
  4374237.8 |####################
  4405266.5 |
  4436295.3 |
  4467324.0 |
  4498352.7 |####################
  4529381.5 |####################
  4560410.2 |
  4591438.9 |####################
  4622467.6 |####################
  4653496.4 |
  4684525.1 |####################
  4715553.8 |####################
  4746582.5 |
  4777611.3 |####################
  4808640.0 |####################
  4839668.7 |
  4870697.5 |####################
  (1 below, 4 above range)

```

## Diagnostics

- **grouped**: bridge=104.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=96.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
