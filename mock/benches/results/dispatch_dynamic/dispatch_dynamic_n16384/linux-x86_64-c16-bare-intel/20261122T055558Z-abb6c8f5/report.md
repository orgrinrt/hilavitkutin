# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 25% faster than the next best (dispatch_volatile)

dispatch_direct (2.29 us) leads dispatch_volatile (2.85 us) by 25%, a clear separation rather than a photo finish. CV 12.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 4.3x slower than the field

dispatch_runtime (9.75 us) is 4.3x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 12.3%)

dispatch_direct wins on median (2.29 us) yet has the highest variance (CV 12.3%), while dispatch_volatile is the steadiest (CV 9.5%, 2.85 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dispatch_runtime shows alternating (throttle bounce) (autocorr -0.55)

dispatch_runtime's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.3x the fastest

Fastest dispatch_direct (2.29 us) to slowest dispatch_runtime (9.75 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2287.3 ns median
- 2 variants significantly slower than baseline
- Spread: 4.26x (fastest 2287.3 ns, slowest 9748.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x40ffee15d34db33f |
| Counter frequency | 2112860326 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 4031ns | 4167ns | 3410ns | 4117ns | 4396ns | base |
| dispatch_runtime | 12051ns | 11296ns | 10834ns | 11888ns | 13758ns | +198.95% |
| dispatch_volatile | 4576ns | 4752ns | 3890ns | 4702ns | 4888ns | +13.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2209ns | 1872ns | 2397ns | base | 7.418 |
| dispatch_runtime | 10367ns | 9286ns | 11856ns | +369.39% | 1.580 |
| dispatch_volatile | 2748ns | 2335ns | 2936ns | +24.41% | 5.963 |

## Performance model

- Peak throughput: **8.754 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 7.163 | 81.8% |
| dispatch_runtime | 1.681 | 19.2% |
| dispatch_volatile | 5.741 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 4031ns | 4031ns | base |
| dispatch_runtime | 12051ns | 12051ns | +198.95% |
| dispatch_volatile | 4576ns | 4576ns | +13.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2287ns | base | --- | [2287, 2287] | --- | --- | --- | --- |
| dispatch_runtime | 9749ns | +7510.2ns (+328.3%) | [+7405, +7759]ns | [9627, 10011] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2854ns | +566.8ns (+24.8%) | [+566, +567]ns | [2854, 2854] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 1871ns | +396.4% | +52.5% |
| 1 | 2 | 2287ns | +415.1% | +2.1% |
| 1 | 3 | 2287ns | +307.6% | +24.8% |
| 1 | 4 | 2287ns | +326.4% | +2.1% |
| 1 | 5 | 2287ns | +317.8% | +24.8% |
| 1 | 6 | 2287ns | +404.9% | +24.8% |
| 1 | 7 | 2287ns | +316.7% | +24.8% |
| 1 | 8 | 1871ns | +519.1% | +52.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.042 | ok |
| dispatch_runtime | -0.553 | HIGH- (thermal bounce) |
| dispatch_volatile | -0.376 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 22.7ns | 2208.6ns | 1.0% |  |
| dispatch_runtime | 21.7ns | 10367.0ns | 0.2% |  |
| dispatch_volatile | 24.1ns | 2747.7ns | 0.9% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 1871.7-2397.1 ns)
   1871.7 |###
   1897.9 |
   1924.2 |
   1950.5 |
   1976.7 |
   2003.0 |
   2029.3 |
   2055.6 |
   2081.8 |
   2108.1 |
   2134.4 |
   2160.6 |
   2186.9 |
   2213.2 |
   2239.5 |
   2265.7 |########################################
   2292.0 |########
   2318.3 |
   2344.5 |
   2370.8 |
  (30 below, 2 above range)

dispatch_runtime (n=160, range 9285.6-11856.1 ns)
   9285.6 |######################
   9414.2 |###################################
   9542.7 |###################################
   9671.2 |################
   9799.7 |###########
   9928.3 |####
  10056.8 |###
  10185.3 |####
  10313.8 |
  10442.3 |
  10570.9 |
  10699.4 |#
  10827.9 |
  10956.4 |
  11085.0 |
  11213.5 |
  11342.0 |#
  11470.5 |########################################
  11599.0 |###################
  11727.6 |############
  (16 below, 14 above range)

dispatch_volatile (n=160, range 2335.4-2935.8 ns)
   2335.4 |#####
   2365.4 |
   2395.4 |
   2425.5 |
   2455.5 |
   2485.5 |
   2515.5 |
   2545.5 |
   2575.6 |
   2605.6 |
   2635.6 |
   2665.6 |
   2695.6 |
   2725.7 |
   2755.7 |
   2785.7 |
   2815.7 |
   2845.8 |########################################
   2875.8 |
   2905.8 |
  (23 below, 2 above range)

```
