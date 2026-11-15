# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 25% faster than the next best (dispatch_volatile)

dispatch_direct (2.29 us) leads dispatch_volatile (2.85 us) by 25%, a clear separation rather than a photo finish. CV 19.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 5.0x slower than the field

dispatch_runtime (11.44 us) is 5.0x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 19.9%)

dispatch_direct wins on median (2.29 us) yet has the highest variance (CV 19.9%), while dispatch_volatile is the steadiest (CV 3.0%, 2.85 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest dispatch_direct (2.29 us) to slowest dispatch_runtime (11.44 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2287.2 ns median
- 2 variants significantly slower than baseline
- Spread: 5.00x (fastest 2287.2 ns, slowest 11441.9 ns)

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
| Counter frequency | 2112780910 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 3999ns | 4164ns | 3407ns | 3964ns | 4697ns | base |
| dispatch_runtime | 12943ns | 13336ns | 11211ns | 13198ns | 13911ns | +223.65% |
| dispatch_volatile | 4772ns | 4756ns | 4745ns | 4755ns | 4847ns | +19.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2193ns | 1871ns | 2566ns | base | 7.471 |
| dispatch_runtime | 11134ns | 9664ns | 12001ns | +407.69% | 1.472 |
| dispatch_volatile | 2864ns | 2853ns | 2900ns | +30.60% | 5.720 |

## Performance model

- Peak throughput: **8.755 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 7.163 | 81.8% |
| dispatch_runtime | 1.432 | 16.4% |
| dispatch_volatile | 5.741 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 3999ns | 3999ns | base |
| dispatch_runtime | 12943ns | 12943ns | +223.65% |
| dispatch_volatile | 4772ns | 4772ns | +19.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2287ns | base | --- | [2287, 2287] | --- | --- | --- | --- |
| dispatch_runtime | 11442ns | +9247.2ns (+404.3%) | [+9160, +9293]ns | [11384, 11533] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2854ns | +567.4ns (+24.8%) | [+567, +578]ns | [2854, 2854] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2287ns | +401.6% | +24.8% |
| 1 | 2 | 2287ns | +328.0% | +24.8% |
| 1 | 3 | 1872ns | +499.6% | +52.5% |
| 1 | 4 | 2287ns | +399.1% | +24.8% |
| 1 | 5 | 2287ns | +406.5% | +24.8% |
| 1 | 6 | 1872ns | +525.2% | +52.5% |
| 1 | 7 | 1871ns | +417.6% | +52.5% |
| 1 | 8 | 2287ns | +407.2% | +24.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.142 | ok |
| dispatch_runtime | -0.369 | moderate- |
| dispatch_volatile | 0.266 | moderate+ |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 22.8ns | 2193.0ns | 1.0% |  |
| dispatch_runtime | 23.2ns | 11133.8ns | 0.2% |  |
| dispatch_volatile | 25.0ns | 2864.1ns | 0.9% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 1871.4-2566.4 ns)
   1871.4 |#########################
   1906.1 |
   1940.9 |
   1975.6 |
   2010.4 |
   2045.1 |
   2079.9 |
   2114.6 |
   2149.4 |
   2184.1 |
   2218.9 |
   2253.6 |########################################
   2288.4 |###########
   2323.1 |
   2357.9 |
   2392.6 |
   2427.4 |
   2462.1 |
   2496.9 |
   2531.6 |
  (11 below, 5 above range)

dispatch_runtime (n=160, range 9664.4-12001.4 ns)
   9664.4 |##########
   9781.2 |#############
   9898.1 |#####
  10014.9 |#
  10131.8 |#
  10248.6 |#
  10365.5 |
  10482.4 |
  10599.2 |
  10716.1 |
  10832.9 |
  10949.8 |
  11066.6 |########
  11183.5 |##############
  11300.3 |###############################
  11417.2 |#######################
  11534.0 |########################################
  11650.9 |##########################
  11767.7 |########
  11884.6 |#####
  (17 below, 12 above range)

dispatch_volatile (n=160, range 2852.9-2900.1 ns)
   2852.9 |########################################
   2855.3 |
   2857.7 |
   2860.0 |
   2862.4 |#
   2864.7 |##############
   2867.1 |##
   2869.4 |
   2871.8 |
   2874.2 |
   2876.5 |
   2878.9 |
   2881.2 |
   2883.6 |
   2885.9 |
   2888.3 |
   2890.6 |
   2893.0 |
   2895.4 |
   2897.7 |
  (12 below, 1 above range)

```

## Diagnostics

- **dispatch_direct**: CV=20.7% (high variance, measurements may be unstable)
