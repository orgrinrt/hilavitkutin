# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 257% faster than the next best (dispatch_volatile)

dispatch_direct (2 ns) leads dispatch_volatile (8 ns) by 257%, a clear separation rather than a photo finish. CV 41.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 4.6x slower than the field

dispatch_runtime (10 ns) is 4.6x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 41.6%)

dispatch_direct wins on median (2 ns) yet has the highest variance (CV 41.6%), while dispatch_runtime is the steadiest (CV 21.2%, 10 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest dispatch_direct (2 ns) to slowest dispatch_runtime (10 ns): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 3.8x its best-20%

dispatch_direct's best 20% of batches run at 1 ns but its worst 20% at 3 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2.1 ns median
- 2 variants significantly slower than baseline
- Spread: 4.57x (fastest 2.1 ns, slowest 9.6 ns)

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
| dispatch_direct | 2251ns | 2239ns | 2080ns | 2253ns | 2416ns | base |
| dispatch_runtime | 2361ns | 2410ns | 2144ns | 2408ns | 2437ns | +4.89% |
| dispatch_volatile | 2272ns | 2276ns | 2083ns | 2284ns | 2429ns | +0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2ns | 1ns | 3ns | base | 31.296 |
| dispatch_runtime | 9ns | 7ns | 12ns | +363.08% | 6.758 |
| dispatch_volatile | 8ns | 5ns | 10ns | +269.10% | 8.479 |

## Performance model

- Peak throughput: **74.203 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 30.476 | 41.1% |
| dispatch_runtime | 6.667 | 9.0% |
| dispatch_volatile | 8.533 | 11.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2251ns | 2251ns | base |
| dispatch_runtime | 2361ns | 2361ns | +4.89% |
| dispatch_volatile | 2272ns | 2272ns | +0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_runtime | 10ns | +7.5ns (+357.1%) | [+7, +8]ns | [9, 10] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 8ns | +5.4ns (+257.1%) | [+5, +6]ns | [7, 8] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2ns | +315.8% | +315.8% |
| 1 | 2 | 2ns | +404.8% | +276.2% |
| 1 | 3 | 2ns | +366.7% | +247.6% |
| 1 | 4 | 2ns | +394.7% | +226.3% |
| 1 | 5 | 2ns | +417.6% | +317.6% |
| 1 | 6 | 2ns | +307.1% | +228.6% |
| 1 | 7 | 2ns | +317.4% | +226.1% |
| 1 | 8 | 2ns | +376.2% | +266.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.189 | ok |
| dispatch_runtime | -0.159 | ok |
| dispatch_volatile | 0.288 | moderate+ |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.5ns | 2.0ns | 120.1% | HIGH |
| dispatch_runtime | 5.2ns | 9.5ns | 55.1% | HIGH |
| dispatch_volatile | 3.1ns | 7.5ns | 40.8% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 0.9-3.3 ns)
      0.9 |
      1.0 |
      1.1 |##########################
      1.2 |
      1.3 |
      1.5 |
      1.6 |###################################
      1.7 |
      1.8 |
      1.9 |
      2.1 |########################################
      2.2 |
      2.3 |
      2.4 |######################
      2.5 |
      2.7 |
      2.8 |###########################
      2.9 |
      3.0 |
      3.1 |
  (21 below, 18 above range)

dispatch_runtime (n=160, range 6.7-12.3 ns)
      6.7 |########
      6.9 |###############
      7.2 |############################
      7.5 |
      7.8 |####################
      8.1 |######################
      8.4 |
      8.7 |##########################
      8.9 |#################
      9.2 |
      9.5 |###################################
      9.8 |########################################
     10.1 |
     10.4 |#################
     10.6 |####################
     10.9 |#################
     11.2 |
     11.5 |####################
     11.8 |
     12.1 |###########
  (10 below, 14 above range)

dispatch_volatile (n=160, range 5.4-9.9 ns)
      5.4 |###########
      5.6 |########################
      5.8 |
      6.0 |############################
      6.3 |
      6.5 |###################################
      6.7 |
      6.9 |########################################
      7.2 |
      7.4 |#################################
      7.6 |
      7.9 |#####################################
      8.1 |########################
      8.3 |
      8.5 |
      8.8 |#################################
      9.0 |######################
      9.2 |
      9.4 |#############
      9.7 |
  (11 below, 12 above range)

```

## Diagnostics

- **dispatch_direct**: CV=42.7% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_direct**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=21.5% (high variance, measurements may be unstable)
- **dispatch_runtime**: bridge=52.1% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=21.8% (high variance, measurements may be unstable)
- **dispatch_volatile**: bridge=41.3% of algo (FFI overhead may distort results)
