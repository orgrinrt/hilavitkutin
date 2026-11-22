# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 188% faster than the next best (dispatch_volatile)

dispatch_direct (15 ns) leads dispatch_volatile (42 ns) by 188%, a clear separation rather than a photo finish. CV 14.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 2.9x slower than the field

dispatch_runtime (42 ns) is 2.9x the fastest (15 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 14.5%)

dispatch_direct wins on median (15 ns) yet has the highest variance (CV 14.5%), while dispatch_volatile is the steadiest (CV 9.6%, 42 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (15 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 14.6 ns median
- 2 variants significantly slower than baseline
- Spread: 2.91x (fastest 14.6 ns, slowest 42.5 ns)

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
| dispatch_direct | 2106ns | 2102ns | 2097ns | 2103ns | 2123ns | base |
| dispatch_runtime | 2131ns | 2120ns | 2113ns | 2120ns | 2182ns | +1.21% |
| dispatch_volatile | 2117ns | 2114ns | 2109ns | 2114ns | 2134ns | +0.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 15ns | 12ns | 18ns | base | 17.256 |
| dispatch_runtime | 42ns | 37ns | 49ns | +186.40% | 6.025 |
| dispatch_volatile | 41ns | 36ns | 47ns | +179.60% | 6.172 |

## Performance model

- Peak throughput: **21.289 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 17.534 | 82.4% |
| dispatch_runtime | 6.024 | 28.3% |
| dispatch_volatile | 6.081 | 28.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2106ns | 2106ns | base |
| dispatch_runtime | 2131ns | 2131ns | +1.21% |
| dispatch_volatile | 2117ns | 2117ns | +0.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 15ns | base | --- | [14, 15] | --- | --- | --- | --- |
| dispatch_runtime | 42ns | +27.5ns (+188.4%) | [+27, +28]ns | [42, 43] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 42ns | +26.7ns (+182.9%) | [+26, +28]ns | [41, 42] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 15ns | +182.2% | +188.0% |
| 1 | 2 | 14ns | +203.5% | +197.9% |
| 1 | 3 | 14ns | +199.3% | +187.3% |
| 1 | 4 | 14ns | +199.3% | +196.5% |
| 1 | 5 | 15ns | +184.7% | +176.3% |
| 1 | 6 | 15ns | +162.3% | +162.3% |
| 1 | 7 | 15ns | +188.5% | +185.8% |
| 1 | 8 | 14ns | +197.9% | +193.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.477 | moderate+ |
| dispatch_runtime | -0.226 | moderate- |
| dispatch_volatile | -0.332 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 3.9ns | 14.8ns | 26.3% | HIGH |
| dispatch_runtime | 3.4ns | 42.5ns | 7.9% | HIGH |
| dispatch_volatile | 3.3ns | 41.5ns | 8.1% | HIGH |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 12.0-17.9 ns)
     12.0 |############
     12.3 |####################
     12.6 |###########################
     12.9 |
     13.2 |#########################
     13.5 |
     13.8 |##############################
     14.1 |###################################
     14.4 |#########################
     14.7 |
     15.0 |########################################
     15.3 |####################
     15.6 |####################
     15.9 |
     16.2 |#####################################
     16.5 |##########
     16.8 |
     17.1 |#################
     17.3 |###############
     17.6 |############
  (11 below, 10 above range)

dispatch_runtime (n=160, range 36.8-48.6 ns)
     36.8 |
     37.4 |################
     38.0 |##########
     38.6 |########
     39.2 |################
     39.8 |##########
     40.4 |#########################
     41.0 |##########
     41.5 |#############################
     42.1 |#####################
     42.7 |########################################
     43.3 |##############
     43.9 |########
     44.5 |###########################
     45.1 |########
     45.7 |##############
     46.2 |##
     46.8 |
     47.4 |########
     48.0 |##
  (14 below, 15 above range)

dispatch_volatile (n=160, range 35.7-47.1 ns)
     35.7 |##############
     36.3 |############
     36.9 |######
     37.4 |################
     38.0 |######
     38.6 |####
     39.1 |##########################
     39.7 |################
     40.3 |################
     40.8 |############
     41.4 |########
     42.0 |########################################
     42.5 |####################
     43.1 |##########
     43.7 |##############################
     44.2 |##########
     44.8 |########
     45.4 |##########
     45.9 |####
     46.5 |######
  (11 below, 12 above range)

```

## Diagnostics

- **dispatch_direct**: bridge=25.3% of algo (FFI overhead may distort results)
