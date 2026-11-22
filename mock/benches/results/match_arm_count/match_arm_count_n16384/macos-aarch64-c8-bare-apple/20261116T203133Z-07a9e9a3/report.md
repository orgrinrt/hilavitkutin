# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 83% faster than the next best (match_8_arms)

match_2_arms (1.35 us) leads match_8_arms (2.47 us) by 83%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 8.8x slower than the field

match_64_arms (11.95 us) is 8.8x the fastest (1.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_8_arms shows alternating (throttle bounce) (autocorr -0.51)

match_8_arms's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.8x the fastest

Fastest match_2_arms (1.35 us) to slowest match_64_arms (11.95 us): 8.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1350.6 ns median
- 2 variants significantly slower than baseline
- Spread: 8.85x (fastest 1350.6 ns, slowest 11946.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3c8715d9a4b682f1 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 3440ns | 3433ns | 3426ns | 3433ns | 3475ns | base |
| match_64_arms | 14060ns | 14073ns | 13333ns | 14035ns | 14861ns | +308.72% |
| match_8_arms | 4560ns | 4558ns | 4517ns | 4554ns | 4621ns | +32.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1354ns | 1344ns | 1374ns | base | 12.100 |
| match_64_arms | 11970ns | 11242ns | 12778ns | +783.96% | 1.369 |
| match_8_arms | 2471ns | 2430ns | 2526ns | +82.48% | 6.631 |

## Performance model

- Peak throughput: **12.188 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 12.131 | 99.5% |
| match_64_arms | 1.371 | 11.3% |
| match_8_arms | 6.634 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3440ns | 3440ns | base |
| match_64_arms | 14060ns | 14060ns | +308.72% |
| match_8_arms | 4560ns | 4560ns | +32.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1351ns | base | --- | [1349, 1352] | --- | --- | --- | --- |
| match_64_arms | 11946ns | +10592.5ns (+784.3%) | [+10405, +10696]ns | [11758, 12063] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 2470ns | +1117.9ns (+82.8%) | [+1113, +1122]ns | [2464, 2475] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1351ns | +794.1% | +87.3% |
| 1 | 2 | 1349ns | +775.2% | +80.0% |
| 1 | 3 | 1352ns | +843.0% | +83.5% |
| 1 | 4 | 1352ns | +794.5% | +80.5% |
| 1 | 5 | 1350ns | +751.8% | +83.8% |
| 1 | 6 | 1351ns | +843.6% | +82.3% |
| 1 | 7 | 1349ns | +732.8% | +83.2% |
| 1 | 8 | 1352ns | +751.9% | +80.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.288 | moderate- |
| match_64_arms | -0.315 | moderate- |
| match_8_arms | -0.515 | HIGH- (thermal bounce) |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 1354.1ns | 0.2% |  |
| match_64_arms | 2.9ns | 11969.5ns | 0.0% |  |
| match_8_arms | 2.9ns | 2470.9ns | 0.1% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1344.3-1374.1 ns)
   1344.3 |############
   1345.8 |####################################
   1347.3 |##################################
   1348.7 |#################
   1350.2 |#############################
   1351.7 |####################
   1353.2 |########################################
   1354.7 |##########
   1356.2 |#
   1357.7 |#####
   1359.2 |#####
   1360.7 |#
   1362.2 |#
   1363.6 |######
   1365.1 |###
   1366.6 |#
   1368.1 |###
   1369.6 |
   1371.1 |#
   1372.6 |#
  (16 below, 8 above range)

match_64_arms (n=160, range 11241.5-12777.6 ns)
  11241.5 |############
  11318.3 |####################
  11395.1 |##########
  11471.9 |##############################
  11548.8 |############
  11625.6 |####################
  11702.4 |##############
  11779.2 |############
  11856.0 |######
  11932.8 |##############
  12009.6 |################
  12086.4 |##########################
  12163.2 |##########
  12240.0 |########
  12316.8 |####
  12393.6 |
  12470.4 |##
  12547.2 |######
  12624.0 |################
  12700.8 |########################################
  (11 below, 10 above range)

match_8_arms (n=160, range 2430.1-2526.3 ns)
   2430.1 |##################
   2434.9 |#####################
   2439.7 |##################
   2444.5 |##########
   2449.3 |
   2454.1 |#####
   2458.9 |####################
   2463.7 |##############
   2468.5 |#########################
   2473.3 |##################
   2478.2 |########################################
   2483.0 |############
   2487.8 |#######
   2492.6 |
   2497.4 |
   2502.2 |
   2507.0 |#####
   2511.8 |
   2516.6 |
   2521.4 |#######
  (16 below, 20 above range)

```
