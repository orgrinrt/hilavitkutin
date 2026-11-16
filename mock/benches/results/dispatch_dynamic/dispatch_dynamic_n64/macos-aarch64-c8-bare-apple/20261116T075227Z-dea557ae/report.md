# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 257% faster than the next best (dispatch_volatile)

dispatch_direct (2 ns) leads dispatch_volatile (8 ns) by 257%, a clear separation rather than a photo finish. CV 38.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 4.0x slower than the field

dispatch_runtime (8 ns) is 4.0x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 38.8%)

dispatch_direct wins on median (2 ns) yet has the highest variance (CV 38.8%), while dispatch_volatile is the steadiest (CV 21.5%, 8 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.0x the fastest

Fastest dispatch_direct (2 ns) to slowest dispatch_runtime (8 ns): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 3.4x its best-20%

dispatch_direct's best 20% of batches run at 1 ns but its worst 20% at 3 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 2.1 ns median
- 2 variants significantly slower than baseline
- Spread: 3.95x (fastest 2.1 ns, slowest 8.3 ns)

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
| dispatch_direct | 2092ns | 2083ns | 2075ns | 2085ns | 2127ns | base |
| dispatch_runtime | 2099ns | 2090ns | 2082ns | 2092ns | 2134ns | +0.33% |
| dispatch_volatile | 2097ns | 2088ns | 2083ns | 2089ns | 2137ns | +0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2ns | 1ns | 3ns | base | 30.305 |
| dispatch_runtime | 9ns | 6ns | 11ns | +305.36% | 7.476 |
| dispatch_volatile | 8ns | 5ns | 10ns | +260.88% | 8.398 |

## Performance model

- Peak throughput: **65.223 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 30.476 | 46.7% |
| dispatch_runtime | 7.711 | 11.8% |
| dispatch_volatile | 8.533 | 13.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2092ns | 2092ns | base |
| dispatch_runtime | 2099ns | 2099ns | +0.33% |
| dispatch_volatile | 2097ns | 2097ns | +0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_runtime | 8ns | +6.5ns (+307.1%) | [+6, +7]ns | [8, 9] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 8ns | +5.4ns (+257.1%) | [+5, +6]ns | [7, 8] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2ns | +307.1% | +276.2% |
| 1 | 2 | 2ns | +388.2% | +388.2% |
| 1 | 3 | 2ns | +319.0% | +238.1% |
| 1 | 4 | 2ns | +376.5% | +364.7% |
| 1 | 5 | 2ns | +364.7% | +341.2% |
| 1 | 6 | 2ns | +260.9% | +208.7% |
| 1 | 7 | 2ns | +260.9% | +252.2% |
| 1 | 8 | 2ns | +319.0% | +219.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.062 | ok |
| dispatch_runtime | -0.026 | ok |
| dispatch_volatile | -0.430 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.5ns | 2.1ns | 119.7% | HIGH |
| dispatch_runtime | 3.7ns | 8.6ns | 42.7% | HIGH |
| dispatch_volatile | 2.4ns | 7.6ns | 32.1% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 1.0-3.3 ns)
      1.0 |
      1.1 |###############
      1.2 |
      1.3 |
      1.4 |
      1.6 |
      1.7 |####################################
      1.8 |
      1.9 |
      2.0 |########################################
      2.1 |
      2.3 |
      2.4 |
      2.5 |####################
      2.6 |
      2.7 |
      2.8 |################
      3.0 |
      3.1 |
      3.2 |#############
  (15 below, 9 above range)

dispatch_runtime (n=160, range 6.0-11.2 ns)
      6.0 |############
      6.3 |
      6.5 |######################
      6.8 |
      7.0 |################
      7.3 |##################
      7.5 |
      7.8 |################################
      8.1 |########################################
      8.3 |
      8.6 |##############################
      8.8 |
      9.1 |########
      9.4 |################
      9.6 |
      9.9 |########################
     10.1 |##################
     10.4 |
     10.7 |##########################
     10.9 |
  (16 below, 13 above range)

dispatch_volatile (n=160, range 5.4-10.0 ns)
      5.4 |
      5.7 |#########################
      5.9 |
      6.1 |################################
      6.3 |
      6.6 |##############################
      6.8 |
      7.0 |########################################
      7.2 |
      7.5 |########################################
      7.7 |########################################
      7.9 |
      8.1 |###################################
      8.4 |
      8.6 |##############################
      8.8 |
      9.0 |#########################
      9.3 |
      9.5 |######################
      9.7 |
  (18 below, 14 above range)

```

## Diagnostics

- **dispatch_direct**: CV=38.5% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **dispatch_direct**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=21.6% (high variance, measurements may be unstable)
- **dispatch_runtime**: bridge=39.8% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=21.2% (high variance, measurements may be unstable)
- **dispatch_volatile**: bridge=33.3% of algo (FFI overhead may distort results)
