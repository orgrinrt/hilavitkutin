# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 223% faster than the next best (dispatch_volatile)

dispatch_direct (15 ns) leads dispatch_volatile (47 ns) by 223%, a clear separation rather than a photo finish. CV 21.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 3.2x slower than the field

dispatch_runtime (47 ns) is 3.2x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 21.7%)

dispatch_direct wins on median (15 ns) yet has the highest variance (CV 21.7%), while dispatch_volatile is the steadiest (CV 10.1%, 47 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (15 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest dispatch_direct (15 ns) to slowest dispatch_runtime (47 ns): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dispatch_direct is inconsistent: worst-20% is 1.9x its best-20%

dispatch_direct's best 20% of batches run at 10 ns but its worst 20% at 19 ns (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 14.6 ns median
- 2 variants significantly slower than baseline
- Spread: 3.24x (fastest 14.6 ns, slowest 47.3 ns)

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
| dispatch_direct | 2329ns | 2409ns | 2121ns | 2365ns | 2431ns | base |
| dispatch_runtime | 2331ns | 2431ns | 2115ns | 2362ns | 2457ns | +0.10% |
| dispatch_volatile | 2361ns | 2430ns | 2140ns | 2405ns | 2449ns | +1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 15ns | 10ns | 19ns | base | 17.355 |
| dispatch_runtime | 48ns | 40ns | 57ns | +223.69% | 5.362 |
| dispatch_volatile | 47ns | 40ns | 54ns | +218.74% | 5.445 |

## Performance model

- Peak throughput: **24.734 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 17.534 | 70.9% |
| dispatch_runtime | 5.412 | 21.9% |
| dispatch_volatile | 5.435 | 22.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2329ns | 2329ns | base |
| dispatch_runtime | 2331ns | 2331ns | +0.10% |
| dispatch_volatile | 2361ns | 2361ns | +1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 15ns | base | --- | [14, 15] | --- | --- | --- | --- |
| dispatch_runtime | 47ns | +32.5ns (+222.6%) | [+31, +33]ns | [46, 48] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 47ns | +32.1ns (+219.9%) | [+31, +33]ns | [46, 48] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 18ns | +147.4% | +177.1% |
| 1 | 2 | 16ns | +215.5% | +201.5% |
| 1 | 3 | 16ns | +197.6% | +167.2% |
| 1 | 4 | 15ns | +191.1% | +228.1% |
| 1 | 5 | 12ns | +291.4% | +309.4% |
| 1 | 6 | 16ns | +212.5% | +188.5% |
| 1 | 7 | 12ns | +283.2% | +275.2% |
| 1 | 8 | 12ns | +298.0% | +279.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.248 | moderate+ |
| dispatch_runtime | -0.156 | ok |
| dispatch_volatile | -0.307 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 3.8ns | 14.8ns | 26.1% | HIGH |
| dispatch_runtime | 3.8ns | 47.7ns | 8.0% | HIGH |
| dispatch_volatile | 3.5ns | 47.0ns | 7.4% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 10.4-19.3 ns)
     10.4 |################
     10.8 |########################
     11.2 |
     11.7 |##################
     12.1 |########################
     12.6 |##################
     13.0 |#############
     13.5 |################################
     13.9 |################
     14.4 |########################################
     14.8 |#####################
     15.3 |##########
     15.7 |##################
     16.2 |#####################
     16.6 |################
     17.1 |##########################
     17.5 |################
     18.0 |##################
     18.4 |##
     18.9 |########
  (11 below, 13 above range)

dispatch_runtime (n=160, range 40.5-56.9 ns)
     40.5 |##########
     41.3 |#################################
     42.1 |#################################
     42.9 |##############################
     43.7 |################
     44.6 |#################################
     45.4 |##############################
     46.2 |################
     47.0 |#################################
     47.8 |#######################
     48.7 |##########################
     49.5 |########################################
     50.3 |##########################
     51.1 |##########################
     51.9 |################
     52.8 |####################
     53.6 |##########
     54.4 |###
     55.2 |##########
     56.0 |##########
  (15 below, 10 above range)

dispatch_volatile (n=160, range 40.5-53.6 ns)
     40.5 |#####
     41.1 |###############
     41.8 |#######
     42.5 |###########################
     43.1 |##########
     43.8 |##########
     44.4 |#################
     45.1 |######################
     45.7 |#################
     46.4 |###############
     47.1 |########################################
     47.7 |######################
     48.4 |#################
     49.0 |###################################
     49.7 |############
     50.3 |####################
     51.0 |##########
     51.7 |#######
     52.3 |######################
     53.0 |##
  (15 below, 10 above range)

```

## Diagnostics

- **dispatch_direct**: CV=21.5% (high variance, measurements may be unstable)
- **dispatch_direct**: bridge=25.3% of algo (FFI overhead may distort results)
