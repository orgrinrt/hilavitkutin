# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_direct dominates: 45% faster than the next best (dispatch_volatile)

dispatch_direct (110 ns) leads dispatch_volatile (160 ns) by 45%, a clear separation rather than a photo finish. CV 10.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_runtime is an outlier: 2.6x slower than the field

dispatch_runtime (291 ns) is 2.6x the fastest (110 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dispatch_direct is fastest but the noisiest (CV 10.3%)

dispatch_direct wins on median (110 ns) yet has the highest variance (CV 10.3%), while dispatch_volatile is the steadiest (CV 2.8%, 160 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dispatch_runtime shows alternating (throttle bounce) (autocorr -0.54)

dispatch_runtime's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (110 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 110.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.63x (fastest 110.4 ns, slowest 290.8 ns)

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
| dispatch_direct | 2216ns | 2199ns | 2190ns | 2205ns | 2273ns | base |
| dispatch_runtime | 2368ns | 2358ns | 2343ns | 2364ns | 2407ns | +6.89% |
| dispatch_volatile | 2243ns | 2228ns | 2216ns | 2236ns | 2291ns | +1.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 112ns | 106ns | 120ns | base | 9.184 |
| dispatch_runtime | 292ns | 281ns | 305ns | +161.90% | 3.507 |
| dispatch_volatile | 160ns | 154ns | 167ns | +43.47% | 6.401 |

## Performance model

- Peak throughput: **9.670 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 9.275 | 95.9% |
| dispatch_runtime | 3.521 | 36.4% |
| dispatch_volatile | 6.416 | 66.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2216ns | 2216ns | base |
| dispatch_runtime | 2368ns | 2368ns | +6.89% |
| dispatch_volatile | 2243ns | 2243ns | +1.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 110ns | base | --- | [110, 111] | --- | --- | --- | --- |
| dispatch_runtime | 291ns | +179.2ns (+162.3%) | [+178, +181]ns | [290, 292] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 160ns | +49.2ns (+44.6%) | [+48, +50]ns | [159, 160] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 113ns | +152.4% | +39.5% |
| 1 | 2 | 111ns | +165.7% | +43.9% |
| 1 | 3 | 110ns | +159.5% | +46.9% |
| 1 | 4 | 109ns | +177.6% | +47.5% |
| 1 | 5 | 112ns | +158.6% | +41.8% |
| 1 | 6 | 108ns | +179.1% | +46.2% |
| 1 | 7 | 110ns | +158.4% | +44.4% |
| 1 | 8 | 111ns | +162.9% | +47.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | -0.263 | moderate- |
| dispatch_runtime | -0.537 | HIGH- (thermal bounce) |
| dispatch_volatile | -0.010 | ok |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 2.7ns | 111.5ns | 2.4% |  |
| dispatch_runtime | 3.8ns | 292.0ns | 1.3% |  |
| dispatch_volatile | 3.3ns | 160.0ns | 2.1% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 105.9-119.8 ns)
    105.9 |######
    106.6 |#######################
    107.3 |#####################
    108.0 |#####################
    108.7 |#######################
    109.4 |########################################
    110.1 |##############
    110.8 |#########################
    111.5 |#########################
    112.2 |#####################
    112.9 |#############################
    113.6 |################
    114.3 |########
    115.0 |################
    115.7 |##
    116.4 |##
    117.1 |
    117.8 |
    118.5 |##
    119.1 |
  (12 below, 6 above range)

dispatch_runtime (n=160, range 281.3-305.3 ns)
    281.3 |####################
    282.5 |################
    283.7 |##########################
    284.9 |########################################
    286.1 |########################################
    287.3 |################
    288.5 |####################################
    289.7 |##############################
    290.9 |##########################
    292.1 |#######################
    293.3 |##########################
    294.5 |################
    295.7 |#######################
    296.9 |################
    298.1 |################
    299.3 |##########
    300.5 |################
    301.7 |####################
    302.9 |#############
    304.1 |####################
  (14 below, 9 above range)

dispatch_volatile (n=160, range 154.1-166.6 ns)
    154.1 |##########
    154.7 |#######
    155.3 |###############
    156.0 |######################
    156.6 |#########################
    157.2 |####################
    157.8 |######################
    158.5 |##########
    159.1 |########################################
    159.7 |###############
    160.3 |#########################
    161.0 |##########
    161.6 |#########################
    162.2 |############
    162.8 |#########################
    163.5 |##########
    164.1 |###############
    164.7 |
    165.4 |##########
    166.0 |#######
  (14 below, 15 above range)

```
