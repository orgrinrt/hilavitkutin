# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.4x slower than the field

ctz_loop (526 ns) is 3.4x the fastest (153 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (153 ns) and ctz_intrinsic (154 ns) differ by 0.16%, inside the noise, even though the wider field spreads 243.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ctz_intrinsic shows alternating (throttle bounce) (autocorr -0.59)

ctz_intrinsic's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (153 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest ctz_debruijn (153 ns) to slowest ctz_loop (526 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader ctz_debruijn vs stability leader ctz_intrinsic (+0% speed for 1.4x steadier)

ctz_debruijn is fastest (153 ns, CV 4.2%); ctz_intrinsic gives up 0.2% median for 1.4x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 153.3 ns median
- 1 variant significantly slower than baseline
- Spread: 3.43x (fastest 153.3 ns, slowest 526.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2246ns | 2249ns | 2204ns | 2249ns | 2281ns | base |
| ctz_intrinsic | 2250ns | 2251ns | 2218ns | 2251ns | 2279ns | +0.17% |
| ctz_loop | 2623ns | 2619ns | 2550ns | 2622ns | 2702ns | +16.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 154ns | 147ns | 162ns | base | 26.634 |
| ctz_intrinsic | 154ns | 147ns | 160ns | -0.09% | 26.658 |
| ctz_loop | 528ns | 493ns | 564ns | +243.44% | 7.755 |

## Performance model

- Peak throughput: **27.851 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 26.719 | 95.9% |
| ctz_intrinsic | 26.675 | 95.8% |
| ctz_loop | 7.783 | 27.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2246ns | 2246ns | base |
| ctz_intrinsic | 2250ns | 2250ns | +0.17% |
| ctz_loop | 2623ns | 2623ns | +16.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 153ns | base | --- | [152, 155] | --- | --- | --- | --- |
| ctz_intrinsic | 154ns | no significant difference | [-0, +0]ns | [152, 155] | no | 0.8699 | 0.8699 | 11 |
| ctz_loop | 526ns | +371.5ns (+242.3%) | [+369, +374]ns | [523, 530] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 153ns | +0.8% | +238.4% |
| 1 | 2 | 153ns | +1.0% | +259.8% |
| 1 | 3 | 151ns | -0.3% | +272.8% |
| 1 | 4 | 155ns | +0.0% | +239.3% |
| 1 | 5 | 151ns | +1.0% | +220.9% |
| 1 | 6 | 152ns | +1.7% | +239.2% |
| 1 | 7 | 152ns | +0.7% | +252.7% |
| 1 | 8 | 155ns | -0.8% | +238.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.499 | moderate- |
| ctz_intrinsic | -0.588 | HIGH- (thermal bounce) |
| ctz_loop | 0.238 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 2/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.3ns | 153.8ns | 1.5% |  |
| ctz_intrinsic | 2.3ns | 153.6ns | 1.5% |  |
| ctz_loop | 2.9ns | 528.2ns | 0.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 147.1-161.9 ns)
    147.1 |###########################
    147.8 |##############################
    148.6 |###########################
    149.3 |###########################
    150.0 |###############
    150.8 |####################################
    151.5 |########################
    152.3 |#####################
    153.0 |###############
    153.8 |#####################
    154.5 |#################################
    155.2 |###############
    156.0 |##################
    156.7 |###############
    157.5 |##################
    158.2 |########################################
    158.9 |#####################
    159.7 |############
    160.4 |
    161.2 |###############
  (10 below, 7 above range)

ctz_intrinsic (n=160, range 147.1-160.4 ns)
    147.1 |
    147.8 |###############
    148.4 |######
    149.1 |########################
    149.8 |########################################
    150.4 |##################
    151.1 |########################
    151.8 |############
    152.4 |########################################
    153.1 |############
    153.8 |#################################
    154.4 |##############################
    155.1 |##################
    155.8 |########################
    156.4 |########################
    157.1 |#########
    157.8 |###############
    158.4 |###############
    159.1 |###########################
    159.8 |###############
  (17 below, 10 above range)

ctz_loop (n=160, range 492.8-564.5 ns)
    492.8 |
    496.4 |
    500.0 |######
    503.6 |##########
    507.1 |########
    510.7 |#####################
    514.3 |##########
    517.9 |#############################
    521.5 |########################################
    525.0 |################
    528.6 |###########################
    532.2 |##########
    535.8 |################
    539.4 |##########
    543.0 |########
    546.5 |#####################
    550.1 |##########
    553.7 |################
    557.3 |######
    560.9 |##
  (19 below, 11 above range)

```
