# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 25% faster than the next best (dispatch_volatile)

dispatch_direct (2.29 us) leads dispatch_volatile (2.85 us) by 25%, a clear separation rather than a photo finish. CV 25.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 5.0x slower than the field

dispatch_runtime (11.45 us) is 5.0x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 25.1%)

dispatch_direct wins on median (2.29 us) yet has the highest variance (CV 25.1%), while dispatch_runtime is the steadiest (CV 7.4%, 11.45 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest dispatch_direct (2.29 us) to slowest dispatch_runtime (11.45 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 1.7x its best-20%

dispatch_direct's best 20% of batches run at 1.87 us but its worst 20% at 3.21 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2287.2 ns median
- 2 variants significantly slower than baseline
- Spread: 5.00x (fastest 2287.2 ns, slowest 11445.2 ns)

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
| Counter frequency | 2112761094 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 4222ns | 4173ns | 3414ns | 3975ns | 5767ns | base |
| dispatch_runtime | 12922ns | 13334ns | 11165ns | 13208ns | 13822ns | +206.10% |
| dispatch_volatile | 4855ns | 4758ns | 3892ns | 4708ns | 6261ns | +15.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2322ns | 1871ns | 3208ns | base | 7.057 |
| dispatch_runtime | 11110ns | 9613ns | 11913ns | +378.51% | 1.475 |
| dispatch_volatile | 2897ns | 2336ns | 3676ns | +24.79% | 5.655 |

## Performance model

- Peak throughput: **8.755 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 7.163 | 81.8% |
| dispatch_runtime | 1.432 | 16.4% |
| dispatch_volatile | 5.740 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 4222ns | 4222ns | base |
| dispatch_runtime | 12922ns | 12922ns | +206.10% |
| dispatch_volatile | 4855ns | 4855ns | +15.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2287ns | base | --- | [2287, 2287] | --- | --- | --- | --- |
| dispatch_runtime | 11445ns | +9062.6ns (+396.2%) | [+8999, +9180]ns | [11344, 11505] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 2854ns | +567.2ns (+24.8%) | [+567, +579]ns | [2854, 2854] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 3748ns | +211.7% | +1.5% |
| 1 | 2 | 1879ns | +427.1% | +51.9% |
| 1 | 3 | 1871ns | +506.9% | +52.5% |
| 1 | 4 | 2287ns | +396.8% | +24.8% |
| 1 | 5 | 2287ns | +402.5% | +24.8% |
| 1 | 6 | 1871ns | +407.1% | +52.5% |
| 1 | 7 | 2287ns | +408.2% | +2.1% |
| 1 | 8 | 2288ns | +410.8% | +2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.149 | ok |
| dispatch_runtime | -0.387 | moderate- |
| dispatch_volatile | 0.183 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 24.0ns | 2321.7ns | 1.0% |  |
| dispatch_runtime | 23.2ns | 11109.7ns | 0.2% |  |
| dispatch_volatile | 25.7ns | 2897.2ns | 0.9% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 1871.4-3208.1 ns)
   1871.4 |######################
   1938.2 |
   2005.0 |#
   2071.9 |
   2138.7 |
   2205.6 |
   2272.4 |########################################
   2339.2 |
   2406.1 |
   2472.9 |
   2539.7 |
   2606.6 |
   2673.4 |
   2740.2 |
   2807.1 |
   2873.9 |
   2940.8 |
   3007.6 |
   3074.4 |
   3141.3 |
  (13 below, 20 above range)

dispatch_runtime (n=160, range 9612.6-11913.5 ns)
   9612.6 |######
   9727.7 |##########
   9842.7 |######
   9957.7 |#####
  10072.8 |######
  10187.8 |#
  10302.9 |
  10417.9 |
  10532.9 |
  10648.0 |
  10763.0 |
  10878.1 |
  10993.1 |
  11108.2 |###
  11223.2 |########################################
  11338.2 |##############################
  11453.3 |###################################
  11568.3 |###################################
  11683.4 |#######################
  11798.4 |##########
  (18 below, 14 above range)

dispatch_volatile (n=160, range 2336.0-3676.0 ns)
   2336.0 |###
   2403.0 |
   2470.0 |
   2537.0 |
   2604.0 |
   2671.0 |
   2738.0 |
   2805.0 |########################################
   2872.0 |
   2939.0 |
   3006.0 |
   3073.0 |
   3140.0 |
   3207.0 |
   3274.0 |
   3341.0 |
   3408.0 |
   3475.0 |
   3542.0 |
   3609.0 |
  (29 below, 22 above range)

```

## Diagnostics

- **dispatch_direct**: CV=24.7% (high variance, measurements may be unstable)
- **dispatch_volatile**: CV=20.4% (high variance, measurements may be unstable)
