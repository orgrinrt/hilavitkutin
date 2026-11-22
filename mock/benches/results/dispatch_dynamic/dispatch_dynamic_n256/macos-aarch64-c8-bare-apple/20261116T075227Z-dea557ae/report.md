# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 333% faster than the next best (dispatch_volatile)

dispatch_direct (10 ns) leads dispatch_volatile (45 ns) by 333%, a clear separation rather than a photo finish. CV 19.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 4.4x slower than the field

dispatch_runtime (46 ns) is 4.4x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 19.1%)

dispatch_direct wins on median (10 ns) yet has the highest variance (CV 19.1%), while dispatch_runtime is the steadiest (CV 9.0%, 46 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dispatch_runtime shows alternating (throttle bounce) (autocorr -0.58)

dispatch_runtime's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (10 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.4x the fastest

Fastest dispatch_direct (10 ns) to slowest dispatch_runtime (46 ns): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 1.7x its best-20%

dispatch_direct's best 20% of batches run at 8 ns but its worst 20% at 13 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 10.4 ns median
- 2 variants significantly slower than baseline
- Spread: 4.40x (fastest 10.4 ns, slowest 45.8 ns)

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
| dispatch_direct | 2101ns | 2093ns | 2087ns | 2094ns | 2136ns | base |
| dispatch_runtime | 2129ns | 2121ns | 2103ns | 2127ns | 2161ns | +1.32% |
| dispatch_volatile | 2120ns | 2107ns | 2099ns | 2113ns | 2162ns | +0.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 11ns | 8ns | 13ns | base | 24.314 |
| dispatch_runtime | 46ns | 40ns | 52ns | +336.39% | 5.572 |
| dispatch_volatile | 45ns | 39ns | 51ns | +324.95% | 5.722 |

## Performance model

- Peak throughput: **32.265 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 24.615 | 76.3% |
| dispatch_runtime | 5.590 | 17.3% |
| dispatch_volatile | 5.689 | 17.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2101ns | 2101ns | base |
| dispatch_runtime | 2129ns | 2129ns | +1.32% |
| dispatch_volatile | 2120ns | 2120ns | +0.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 10ns | base | --- | [10, 11] | --- | --- | --- | --- |
| dispatch_runtime | 46ns | +35.4ns (+340.4%) | [+35, +36]ns | [45, 46] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 45ns | +34.4ns (+330.3%) | [+33, +35]ns | [44, 46] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 10ns | +340.4% | +318.7% |
| 1 | 2 | 11ns | +335.8% | +324.5% |
| 1 | 3 | 11ns | +320.4% | +324.1% |
| 1 | 4 | 10ns | +370.8% | +372.9% |
| 1 | 5 | 10ns | +357.4% | +345.1% |
| 1 | 6 | 10ns | +336.5% | +325.0% |
| 1 | 7 | 10ns | +362.5% | +354.0% |
| 1 | 8 | 11ns | +314.5% | +307.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.244 | moderate- |
| dispatch_runtime | -0.579 | HIGH- (thermal bounce) |
| dispatch_volatile | -0.045 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.8ns | 10.5ns | 26.9% | HIGH |
| dispatch_runtime | 2.8ns | 45.9ns | 6.1% | HIGH |
| dispatch_volatile | 2.6ns | 44.7ns | 5.9% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 7.9-13.4 ns)
      7.9 |
      8.2 |#######
      8.5 |
      8.8 |############################
      9.0 |#######################
      9.3 |
      9.6 |############################
      9.9 |########################################
     10.1 |########################################
     10.4 |
     10.7 |###################################
     10.9 |###################################
     11.2 |
     11.5 |#####################
     11.8 |
     12.0 |#######################
     12.3 |#########
     12.6 |
     12.9 |#########
     13.1 |#########
  (17 below, 11 above range)

dispatch_runtime (n=160, range 40.4-51.7 ns)
     40.4 |##########
     40.9 |##########
     41.5 |##########
     42.1 |################
     42.6 |####
     43.2 |####
     43.8 |#############################
     44.3 |##########
     44.9 |########################################
     45.5 |################
     46.0 |##################
     46.6 |###########################
     47.2 |##
     47.7 |###########################
     48.3 |############
     48.9 |################
     49.4 |##############
     50.0 |##
     50.6 |######
     51.1 |######
  (12 below, 11 above range)

dispatch_volatile (n=160, range 38.7-50.7 ns)
     38.7 |##############
     39.3 |###########
     39.9 |#################
     40.5 |###########
     41.1 |######################
     41.7 |##############
     42.3 |########################################
     42.9 |#####
     43.5 |#########################
     44.1 |############################
     44.7 |#################
     45.3 |########################################
     45.9 |####################
     46.5 |##################################
     47.1 |#########################
     47.7 |#################
     48.3 |########
     48.9 |#########################
     49.5 |#####
     50.1 |##
  (12 below, 12 above range)

```

## Diagnostics

- **dispatch_direct**: bridge=27.9% of algo (FFI overhead may distort results)
