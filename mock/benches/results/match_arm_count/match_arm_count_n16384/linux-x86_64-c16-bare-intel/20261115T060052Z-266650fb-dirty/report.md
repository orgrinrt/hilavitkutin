# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 617% faster than the next best (match_64_arms)

match_2_arms (1.58 us) leads match_64_arms (11.31 us) by 617%, a clear separation rather than a photo finish. CV 26.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 7.2x slower than the field

match_8_arms (11.37 us) is 7.2x the fastest (1.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 26.9%)

match_2_arms wins on median (1.58 us) yet has the highest variance (CV 26.9%), while match_64_arms is the steadiest (CV 9.3%, 11.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_8_arms shows alternating (throttle bounce) (autocorr -0.67)

match_8_arms's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.58 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.2x the fastest

Fastest match_2_arms (1.58 us) to slowest match_8_arms (11.37 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1578.0 ns median
- 2 variants significantly slower than baseline
- Spread: 7.21x (fastest 1578.0 ns, slowest 11374.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3c8715d9a4b682f1 |
| Counter frequency | 2112766500 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 3320ns | 3452ns | 2831ns | 3179ns | 4233ns | base |
| match_64_arms | 12631ns | 13221ns | 10900ns | 12750ns | 14005ns | +280.44% |
| match_8_arms | 12764ns | 13280ns | 10984ns | 12884ns | 14187ns | +284.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1514ns | 1293ns | 1922ns | base | 10.821 |
| match_64_arms | 10846ns | 9339ns | 12075ns | +616.36% | 1.511 |
| match_8_arms | 10988ns | 9429ns | 12272ns | +625.73% | 1.491 |

## Performance model

- Peak throughput: **12.674 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.383 | 81.9% |
| match_64_arms | 1.449 | 11.4% |
| match_8_arms | 1.440 | 11.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3320ns | 3320ns | base |
| match_64_arms | 12631ns | 12631ns | +280.44% |
| match_8_arms | 12764ns | 12764ns | +284.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1578ns | base | --- | [1304, 1581] | --- | --- | --- | --- |
| match_64_arms | 11308ns | +9711.0ns (+615.4%) | [+9603, +9792]ns | [11212, 11381] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 11375ns | +9851.7ns (+624.3%) | [+9762, +9995]ns | [11237, 11534] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1584ns | +612.2% | +620.5% |
| 1 | 2 | 1295ns | +623.8% | +827.5% |
| 1 | 3 | 1584ns | +621.9% | +497.3% |
| 1 | 4 | 1296ns | +788.4% | +816.2% |
| 1 | 5 | 1582ns | +621.2% | +510.7% |
| 1 | 6 | 1294ns | +650.3% | +772.5% |
| 1 | 7 | 1296ns | +798.3% | +642.1% |
| 1 | 8 | 1582ns | +496.8% | +644.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.624 | HIGH- (thermal bounce) |
| match_64_arms | -0.470 | moderate- |
| match_8_arms | -0.669 | HIGH- (thermal bounce) |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 22.8ns | 1514.1ns | 1.5% |  |
| match_64_arms | 26.7ns | 10846.3ns | 0.2% |  |
| match_8_arms | 22.8ns | 10988.1ns | 0.2% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1292.7-1921.6 ns)
   1292.7 |##################################
   1324.2 |#
   1355.6 |
   1387.1 |
   1418.5 |
   1449.9 |
   1481.4 |
   1512.8 |
   1544.3 |#
   1575.7 |########################################
   1607.2 |
   1638.6 |
   1670.1 |
   1701.5 |
   1732.9 |
   1764.4 |
   1795.8 |
   1827.3 |
   1858.7 |
   1890.2 |
  (11 below, 5 above range)

match_64_arms (n=160, range 9339.1-12074.9 ns)
   9339.1 |############
   9475.9 |###################
   9612.7 |###################
   9749.5 |########
   9886.3 |#####
  10023.0 |###
  10159.8 |
  10296.6 |###
  10433.4 |
  10570.2 |#
  10707.0 |
  10843.8 |
  10980.6 |
  11117.4 |###################
  11254.2 |######################################
  11391.0 |########################################
  11527.7 |###########################
  11664.5 |##########
  11801.3 |######
  11938.1 |########
  (18 below, 13 above range)

match_8_arms (n=160, range 9429.1-12271.8 ns)
   9429.1 |####################################
   9571.2 |##################
   9713.4 |############
   9855.5 |############
   9997.6 |
  10139.8 |####
  10281.9 |
  10424.0 |####
  10566.2 |##
  10708.3 |##
  10850.5 |
  10992.6 |####
  11134.7 |################
  11276.9 |########################################
  11419.0 |##########
  11561.1 |########################
  11703.3 |############################
  11845.4 |##########################
  11987.6 |####################
  12129.7 |##########
  (14 below, 12 above range)

```

## Diagnostics

- **match_2_arms**: CV=28.0% (high variance, measurements may be unstable)
