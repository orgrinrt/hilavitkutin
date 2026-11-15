# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 3.4x slower than the field

ctz_loop (542 ns) is 3.4x the fastest (159 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn shows alternating (throttle bounce) (autocorr -0.68)

ctz_debruijn's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest ctz_intrinsic (159 ns) to slowest ctz_loop (542 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader ctz_intrinsic vs stability leader ctz_debruijn (+6% speed for 1.1x steadier)

ctz_intrinsic is fastest (159 ns, CV 7.5%); ctz_debruijn gives up 6.4% median for 1.1x lower variance (CV 7.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: ctz_intrinsic** at 158.6 ns median (-6.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.42x (fastest 158.6 ns, slowest 542.1 ns)

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
| ctz_debruijn | 2446ns | 2548ns | 2215ns | 2479ns | 2577ns | base |
| ctz_intrinsic | 2348ns | 2308ns | 2211ns | 2323ns | 2560ns | -4.00% |
| ctz_loop | 2718ns | 2633ns | 2569ns | 2676ns | 2993ns | +11.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 166ns | 149ns | 181ns | base | 24.613 |
| ctz_intrinsic | 161ns | 148ns | 178ns | -3.55% | 25.519 |
| ctz_loop | 544ns | 498ns | 610ns | +227.12% | 7.524 |

## Performance model

- Peak throughput: **27.728 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 24.280 | 87.6% |
| ctz_intrinsic | 25.834 | 93.2% |
| ctz_loop | 7.556 | 27.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2446ns | 2446ns | base |
| ctz_intrinsic | 2348ns | 2348ns | -4.00% |
| ctz_loop | 2718ns | 2718ns | +11.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 169ns | base | --- | [167, 171] | --- | --- | --- | --- |
| ctz_intrinsic | 159ns | -12.5ns (-7.4%) | [-15, -9]ns | [156, 161] | YES | 0.0000 | 0.0000 | 1 |
| ctz_loop | 542ns | +373.5ns (+221.4%) | [+357, +395]ns | [520, 549] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 171ns | -9.5% | +196.4% |
| 1 | 2 | 171ns | -6.3% | +256.8% |
| 1 | 3 | 152ns | +13.8% | +265.2% |
| 1 | 4 | 171ns | -10.7% | +203.5% |
| 1 | 5 | 154ns | -0.9% | +220.8% |
| 1 | 6 | 173ns | -8.1% | +237.3% |
| 1 | 7 | 154ns | +13.8% | +257.0% |
| 1 | 8 | 176ns | -14.3% | +189.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.685 | HIGH- (thermal bounce) |
| ctz_intrinsic | -0.300 | moderate- |
| ctz_loop | -0.217 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 6/8, lost 2/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.3ns | 166.4ns | 1.4% |  |
| ctz_intrinsic | 2.4ns | 160.5ns | 1.5% |  |
| ctz_loop | 3.3ns | 544.4ns | 0.6% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 148.9-180.7 ns)
    148.9 |######################
    150.5 |######################
    152.1 |######################
    153.6 |##############
    155.2 |###########
    156.8 |###########
    158.4 |###########
    160.0 |##
    161.6 |########
    163.2 |##############
    164.8 |#####
    166.4 |######################
    168.0 |######################
    169.6 |##################################
    171.1 |############################
    172.7 |##############
    174.3 |##############
    175.9 |########################################
    177.5 |####################
    179.1 |##################################
  (13 below, 14 above range)

ctz_intrinsic (n=160, range 147.7-178.4 ns)
    147.7 |###############
    149.3 |##############################
    150.8 |########################################
    152.3 |########################################
    153.8 |###############
    155.4 |##############################
    156.9 |###########################
    158.4 |##################
    160.0 |####################################
    161.5 |#####################
    163.0 |########################
    164.6 |#########
    166.1 |##################
    167.6 |###
    169.2 |##################
    170.7 |##################
    172.2 |############
    173.8 |######
    175.3 |#########
    176.8 |###############
  (15 below, 11 above range)

ctz_loop (n=160, range 497.6-609.8 ns)
    497.6 |##########################
    503.2 |################
    508.8 |########################################
    514.4 |##########################
    520.0 |##
    525.6 |##
    531.2 |##
    536.8 |################
    542.5 |########################
    548.1 |######################
    553.7 |################
    559.3 |############
    564.9 |##########
    570.5 |
    576.1 |####
    581.7 |##
    587.4 |################
    593.0 |######
    598.6 |####
    604.2 |############
  (15 below, 16 above range)

```
