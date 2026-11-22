# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 294% faster than the next best (dispatch_volatile)

dispatch_direct (2 ns) leads dispatch_volatile (7 ns) by 294%, a clear separation rather than a photo finish. CV 49.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 5.0x slower than the field

dispatch_runtime (9 ns) is 5.0x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 49.3%)

dispatch_direct wins on median (2 ns) yet has the highest variance (CV 49.3%), while dispatch_runtime is the steadiest (CV 19.3%, 9 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest dispatch_direct (2 ns) to slowest dispatch_runtime (9 ns): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 4.1x its best-20%

dispatch_direct's best 20% of batches run at 1 ns but its worst 20% at 3 ns (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 1.7 ns median
- 2 variants significantly slower than baseline
- Spread: 5.03x (fastest 1.7 ns, slowest 8.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_direct | 2119ns | 2129ns | 2091ns | 2119ns | 2148ns | base |
| dispatch_runtime | 2112ns | 2100ns | 2094ns | 2104ns | 2153ns | -0.35% |
| dispatch_volatile | 2121ns | 2129ns | 2094ns | 2119ns | 2151ns | +0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2ns | 1ns | 3ns | base | 34.901 |
| dispatch_runtime | 8ns | 6ns | 11ns | +362.03% | 7.554 |
| dispatch_volatile | 7ns | 5ns | 9ns | +273.65% | 9.341 |

## Performance model

- Peak throughput: **85.333 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 37.647 | 44.1% |
| dispatch_runtime | 7.485 | 8.8% |
| dispatch_volatile | 9.552 | 11.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2119ns | 2119ns | base |
| dispatch_runtime | 2112ns | 2112ns | -0.35% |
| dispatch_volatile | 2121ns | 2121ns | +0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_runtime | 9ns | +6.7ns (+394.1%) | [+6, +7]ns | [8, 9] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 7ns | +5.0ns (+294.1%) | [+5, +5]ns | [7, 7] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2ns | +341.2% | +329.4% |
| 1 | 2 | 2ns | +464.7% | +264.7% |
| 1 | 3 | 2ns | +417.6% | +341.2% |
| 1 | 4 | 2ns | +376.5% | +317.6% |
| 1 | 5 | 1ns | +417.2% | +362.1% |
| 1 | 6 | 2ns | +285.7% | +228.6% |
| 1 | 7 | 2ns | +216.0% | +158.0% |
| 1 | 8 | 2ns | +452.9% | +305.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.124 | ok |
| dispatch_runtime | -0.118 | ok |
| dispatch_volatile | -0.489 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.4ns | 1.8ns | 130.9% | HIGH |
| dispatch_runtime | 5.3ns | 8.5ns | 62.4% | HIGH |
| dispatch_volatile | 2.9ns | 6.9ns | 42.0% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 0.8-3.1 ns)
      0.8 |################
      0.9 |
      1.0 |
      1.1 |########################
      1.2 |
      1.3 |
      1.4 |
      1.6 |
      1.7 |########################################
      1.8 |
      1.9 |
      2.0 |###################
      2.1 |
      2.3 |
      2.4 |
      2.5 |#################
      2.6 |
      2.7 |
      2.8 |############
      3.0 |
  (8 below, 12 above range)

dispatch_runtime (n=160, range 6.2-10.8 ns)
      6.2 |
      6.4 |
      6.7 |###############
      6.9 |##############################
      7.1 |
      7.4 |########################
      7.6 |
      7.8 |######################
      8.0 |
      8.3 |##########################
      8.5 |
      8.7 |########################################
      9.0 |
      9.2 |############################
      9.4 |#################
      9.6 |
      9.9 |####################
     10.1 |
     10.3 |#################
     10.6 |
  (17 below, 15 above range)

dispatch_volatile (n=160, range 4.8-8.9 ns)
      4.8 |########
      5.0 |
      5.2 |#############
      5.4 |
      5.6 |######################
      5.8 |
      6.1 |######################################
      6.3 |
      6.5 |
      6.7 |###############################
      6.9 |
      7.1 |########################################
      7.3 |
      7.5 |#################
      7.7 |##########################
      7.9 |
      8.1 |####################
      8.3 |
      8.5 |
      8.7 |############
  (16 below, 11 above range)

```

## Diagnostics

- **dispatch_direct**: CV=45.7% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.1% of algo (FFI overhead may distort results)
- **dispatch_runtime**: bridge=63.2% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=21.5% (high variance, measurements may be unstable)
- **dispatch_volatile**: bridge=43.3% of algo (FFI overhead may distort results)
