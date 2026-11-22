# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 294% faster than the next best (dispatch_volatile)

dispatch_direct (2 ns) leads dispatch_volatile (7 ns) by 294%, a clear separation rather than a photo finish. CV 50.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 4.9x slower than the field

dispatch_runtime (8 ns) is 4.9x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 50.3%)

dispatch_direct wins on median (2 ns) yet has the highest variance (CV 50.3%), while dispatch_runtime is the steadiest (CV 22.5%, 8 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest dispatch_direct (2 ns) to slowest dispatch_runtime (8 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 4.1x its best-20%

dispatch_direct's best 20% of batches run at 1 ns but its worst 20% at 3 ns (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 1.7 ns median
- 2 variants significantly slower than baseline
- Spread: 4.88x (fastest 1.7 ns, slowest 8.3 ns)

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
| dispatch_direct | 2107ns | 2096ns | 2090ns | 2096ns | 2155ns | base |
| dispatch_runtime | 2111ns | 2098ns | 2093ns | 2098ns | 2169ns | +0.21% |
| dispatch_volatile | 2110ns | 2099ns | 2093ns | 2100ns | 2158ns | +0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 2ns | 1ns | 3ns | base | 34.866 |
| dispatch_runtime | 8ns | 6ns | 11ns | +357.71% | 7.617 |
| dispatch_volatile | 7ns | 5ns | 9ns | +270.68% | 9.406 |

## Performance model

- Peak throughput: **85.333 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 37.647 | 44.1% |
| dispatch_runtime | 7.711 | 9.0% |
| dispatch_volatile | 9.552 | 11.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2107ns | 2107ns | base |
| dispatch_runtime | 2111ns | 2111ns | +0.21% |
| dispatch_volatile | 2110ns | 2110ns | +0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_runtime | 8ns | +6.7ns (+394.1%) | [+6, +7]ns | [8, 9] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 7ns | +4.8ns (+282.4%) | [+5, +5]ns | [6, 7] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 2ns | +336.8% | +252.6% |
| 1 | 2 | 2ns | +436.8% | +239.5% |
| 1 | 3 | 2ns | +429.4% | +317.6% |
| 1 | 4 | 2ns | +402.9% | +305.9% |
| 1 | 5 | 2ns | +228.6% | +228.6% |
| 1 | 6 | 2ns | +285.7% | +195.2% |
| 1 | 7 | 1ns | +475.0% | +491.7% |
| 1 | 8 | 2ns | +350.0% | +250.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.287 | moderate- |
| dispatch_runtime | 0.167 | ok |
| dispatch_volatile | -0.468 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.4ns | 1.8ns | 131.9% | HIGH |
| dispatch_runtime | 5.4ns | 8.4ns | 63.9% | HIGH |
| dispatch_volatile | 2.6ns | 6.8ns | 38.9% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 0.7-3.1 ns)
      0.7 |##########
      0.9 |
      1.0 |
      1.1 |########################################
      1.2 |
      1.3 |
      1.4 |
      1.6 |
      1.7 |###############################
      1.8 |
      1.9 |
      2.0 |####################################
      2.1 |
      2.2 |
      2.4 |
      2.5 |####################
      2.6 |
      2.7 |
      2.8 |#############
      2.9 |
  (12 below, 12 above range)

dispatch_runtime (n=160, range 5.8-11.0 ns)
      5.8 |
      6.1 |######################
      6.3 |
      6.6 |####################
      6.8 |###############################
      7.1 |
      7.4 |#################
      7.6 |
      7.9 |#################
      8.1 |###############################
      8.4 |
      8.7 |########################################
      8.9 |
      9.2 |######################
      9.4 |###############################
      9.7 |
      9.9 |############################
     10.2 |########
     10.5 |
     10.7 |######################
  (18 below, 10 above range)

dispatch_volatile (n=160, range 4.6-9.1 ns)
      4.6 |################
      4.8 |#########################
      5.0 |
      5.3 |##############################
      5.5 |
      5.7 |#########################
      6.0 |
      6.2 |#####################################
      6.4 |
      6.6 |########################################
      6.9 |
      7.1 |##############################
      7.3 |################################
      7.5 |
      7.8 |###################################
      8.0 |
      8.2 |#######################
      8.5 |
      8.7 |##################
      8.9 |
  (10 below, 15 above range)

```

## Diagnostics

- **dispatch_direct**: CV=46.6% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **dispatch_direct**: bridge=147.1% of algo (FFI overhead may distort results)
- **dispatch_runtime**: CV=22.2% (high variance, measurements may be unstable)
- **dispatch_runtime**: bridge=62.7% of algo (FFI overhead may distort results)
- **dispatch_volatile**: CV=24.0% (high variance, measurements may be unstable)
- **dispatch_volatile**: bridge=37.3% of algo (FFI overhead may distort results)
