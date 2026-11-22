# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 339% faster than the next best (popcount_handrolled)

popcount_std (34 ns) leads popcount_handrolled (148 ns) by 339%, a clear separation rather than a photo finish. CV 15.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 77% (significant)

popcount_std is -114 ns (77%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 4.4x slower than the field

popcount_swar (149 ns) is 4.4x the fastest (34 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 15.2%)

popcount_std wins on median (34 ns) yet has the highest variance (CV 15.2%), while popcount_handrolled is the steadiest (CV 3.3%, 148 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_std shows alternating (throttle bounce) (autocorr -0.61)

popcount_std's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest popcount_std (34 ns) to slowest popcount_swar (149 ns): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: popcount_std** at 33.8 ns median (-77.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.41x (fastest 33.8 ns, slowest 148.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2225ns | 2222ns | 2215ns | 2222ns | 2245ns | base |
| popcount_std | 2122ns | 2109ns | 2104ns | 2111ns | 2173ns | -4.64% |
| popcount_swar | 2239ns | 2222ns | 2216ns | 2223ns | 2309ns | +0.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 149ns | 143ns | 156ns | base | 6.886 |
| popcount_std | 35ns | 29ns | 43ns | -76.62% | 29.449 |
| popcount_swar | 149ns | 143ns | 158ns | +0.49% | 6.852 |

## Performance model

- Peak throughput: **35.746 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.905 | 19.3% |
| popcount_std | 30.296 | 84.8% |
| popcount_swar | 6.875 | 19.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2225ns | 2225ns | base |
| popcount_std | 2122ns | 2122ns | -4.64% |
| popcount_swar | 2239ns | 2239ns | +0.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 148ns | base | --- | [147, 150] | --- | --- | --- | --- |
| popcount_std | 34ns | -113.7ns (-76.7%) | [-114, -113]ns | [33, 35] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 149ns | no significant difference | [+0, +1]ns | [148, 150] | no | 0.0987 | 0.0987 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 149ns | -77.4% | -1.0% |
| 1 | 2 | 150ns | -76.5% | -0.4% |
| 1 | 3 | 149ns | -77.9% | -0.0% |
| 1 | 4 | 148ns | -76.6% | +0.3% |
| 1 | 5 | 148ns | -77.4% | -0.1% |
| 1 | 6 | 147ns | -77.2% | +3.3% |
| 1 | 7 | 148ns | -77.4% | +0.4% |
| 1 | 8 | 149ns | -76.8% | +2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.451 | moderate+ |
| popcount_std | -0.612 | HIGH- (thermal bounce) |
| popcount_swar | -0.386 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 2.8ns | 148.7ns | 1.9% |  |
| popcount_std | 2.2ns | 34.8ns | 6.3% | HIGH |
| popcount_swar | 2.7ns | 149.4ns | 1.8% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 142.7-156.0 ns)
    142.7 |####################################
    143.4 |######
    144.1 |####################################
    144.7 |###############
    145.4 |########################################
    146.1 |########################
    146.7 |###############
    147.4 |###########################
    148.0 |#########
    148.7 |##############################
    149.4 |####################################
    150.0 |###############
    150.7 |#####################
    151.3 |######
    152.0 |##################
    152.7 |#################################
    153.3 |###############
    154.0 |############
    154.6 |
    155.3 |#########
  (13 below, 13 above range)

popcount_std (n=160, range 28.6-42.6 ns)
     28.6 |######################
     29.3 |########################################
     30.0 |######################
     30.7 |##################################
     31.4 |#########################
     32.1 |###########
     32.8 |##################################
     33.5 |####################
     34.2 |#####
     34.9 |######################
     35.6 |####################
     36.3 |#####
     37.0 |####################
     37.7 |#################
     38.4 |#################
     39.1 |#########################
     39.8 |########
     40.5 |##
     41.2 |###########
     41.9 |###########
  (12 below, 15 above range)

popcount_swar (n=160, range 143.1-157.6 ns)
    143.1 |############################
    143.9 |###########
    144.6 |###############################
    145.3 |######################
    146.0 |########################################
    146.8 |########
    147.5 |####################
    148.2 |###############################
    148.9 |#####################################
    149.6 |#########################
    150.4 |#########################
    151.1 |########
    151.8 |#################
    152.5 |#################
    153.3 |########
    154.0 |###########
    154.7 |######################
    155.4 |#####
    156.2 |########
    156.9 |##
  (12 below, 13 above range)

```
