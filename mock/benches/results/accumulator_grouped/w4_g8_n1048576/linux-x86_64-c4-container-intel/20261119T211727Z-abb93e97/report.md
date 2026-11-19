# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 29% faster than the next best (grouped)

per_core (2.38 ms) leads grouped (3.07 ms) by 29%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core beats baseline by 22% (significant)

per_core is -688.17 us (22%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 2.0x slower than the field

sequential (4.84 ms) is 2.0x the fastest (2.38 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core is fastest but the noisiest (CV 5.8%)

per_core wins on median (2.38 ms) yet has the highest variance (CV 5.8%), while sequential is the steadiest (CV 4.7%, 4.84 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 2383604.2 ns median (-22.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.03x (fastest 2383604.2 ns, slowest 4840785.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
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
| grouped | 3122563ns | 3070517ns | 2930226ns | 3063017ns | 3308050ns | base |
| per_core | 2390202ns | 2385569ns | 2175757ns | 2361700ns | 2540176ns | -23.45% |
| sequential | 4855801ns | 4843838ns | 4586435ns | 4801593ns | 5071797ns | +55.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 3120150ns | 2927741ns | 3305517ns | base | 0.336 |
| per_core | 2387948ns | 2172602ns | 2537906ns | -23.47% | 0.439 |
| sequential | 4853142ns | 4584169ns | 5068811ns | +55.54% | 0.216 |

## Performance model

- Peak throughput: **0.483 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.342 | 70.8% |
| per_core | 0.440 | 91.1% |
| sequential | 0.217 | 44.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 3122563ns | 3122563ns | base |
| per_core | 2390202ns | 2390202ns | -23.45% |
| sequential | 4855801ns | 4855801ns | +55.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 3068222ns | base | --- | [3021039, 3220522] | --- | --- | --- | --- |
| per_core | 2383604ns | -688172.6ns (-22.4%) | [-877212, -632972]ns | [2317887, 2476884] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 4840786ns | +1658010.4ns (+54.0%) | [+1599154, +1784565]ns | [4717531, 4895393] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 3002495ns | -20.3% | +58.6% |
| 1 | 2 | 3295295ns | -26.8% | +48.1% |
| 1 | 3 | 3101811ns | -24.5% | +54.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.593 | HIGH- (thermal bounce) |
| per_core | -0.336 | moderate- |
| sequential | -0.652 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 4275857.7ns | 3120150.5ns | 137.0% | HIGH |
| per_core | 2348429.2ns | 2387948.2ns | 98.3% | HIGH |
| sequential | 4895007.9ns | 4853141.6ns | 100.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 2927741.0-3305517.0 ns)
  2927741.0 |
  2946629.8 |
  2965518.6 |#############
  2984407.4 |
  3003296.2 |##########################
  3022185.0 |########################################
  3041073.8 |#############
  3059962.6 |
  3078851.4 |#############
  3097740.2 |#############
  3116629.0 |
  3135517.8 |
  3154406.6 |
  3173295.4 |#############
  3192184.2 |#############
  3211073.0 |
  3229961.8 |#############
  3248850.6 |
  3267739.4 |#############
  3286628.2 |
  (2 below, 3 above range)

per_core (n=18, range 2172601.5-2537906.4 ns)
  2172601.5 |####################
  2190866.8 |
  2209132.0 |
  2227397.3 |
  2245662.5 |
  2263927.8 |####################
  2282193.0 |########################################
  2300458.2 |
  2318723.5 |####################
  2336988.7 |####################
  2355254.0 |####################
  2373519.2 |####################
  2391784.5 |########################################
  2410049.7 |
  2428314.9 |####################
  2446580.2 |####################
  2464845.4 |
  2483110.7 |
  2501375.9 |########################################
  2519641.2 |
  (1 below, 3 above range)

sequential (n=18, range 4584169.4-5068811.0 ns)
  4584169.4 |##########
  4608401.4 |
  4632633.5 |
  4656865.6 |##########
  4681097.7 |
  4705329.8 |##############################
  4729561.9 |
  4753793.9 |
  4778026.0 |
  4802258.1 |##########
  4826490.2 |########################################
  4850722.3 |##########
  4874954.3 |##########
  4899186.4 |##############################
  4923418.5 |
  4947650.6 |
  4971882.7 |
  4996114.7 |
  5020346.8 |
  5044578.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=138.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=98.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
