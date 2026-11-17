# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 shows alternating (throttle bounce) (autocorr -0.60)

fpt_match_16's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (10.66 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader fpt_match_16 vs stability leader fpt_table_16 (+7% speed for 1.9x steadier)

fpt_match_16 is fastest (10.66 us, CV 4.6%); fpt_table_16 gives up 7.1% median for 1.9x lower variance (CV 2.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 10655.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 10655.8 ns, slowest 11408.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 12710ns | 12742ns | 11969ns | 12753ns | 13324ns | base |
| fpt_table_16 | 13511ns | 13510ns | 13078ns | 13529ns | 13889ns | +6.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 10612ns | 9879ns | 11219ns | base | 1.544 |
| fpt_table_16 | 11419ns | 10993ns | 11795ns | +7.61% | 1.435 |

## Performance model

- Peak throughput: **1.658 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.538 | 92.7% |
| fpt_table_16 | 1.436 | 86.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 12710ns | 12710ns | base |
| fpt_table_16 | 13511ns | 13511ns | +6.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 10656ns | base | --- | [10533, 10864] | --- | --- | --- | --- |
| fpt_table_16 | 11408ns | +839.4ns (+7.9%) | [+756, +929]ns | [11355, 11518] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 10536ns | +7.5% |
| 1 | 2 | 11250ns | +2.2% |
| 1 | 3 | 10061ns | +10.2% |
| 1 | 4 | 11092ns | +0.5% |
| 1 | 5 | 10585ns | +11.0% |
| 1 | 6 | 10574ns | +10.3% |
| 1 | 7 | 10924ns | +1.4% |
| 1 | 8 | 9938ns | +16.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.604 | HIGH- (thermal bounce) |
| fpt_table_16 | -0.204 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.5ns | 10611.8ns | 0.0% |  |
| fpt_table_16 | 3.3ns | 11418.9ns | 0.0% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 9879.3-11218.6 ns)
   9879.3 |########
   9946.2 |######################
  10013.2 |#############
  10080.2 |####
  10147.1 |####################
  10214.1 |####
  10281.1 |######
  10348.0 |####################
  10415.0 |######
  10482.0 |######################
  10548.9 |#############
  10615.9 |#############
  10682.9 |########
  10749.8 |######
  10816.8 |########
  10883.7 |########################################
  10950.7 |####
  11017.7 |######################
  11084.6 |#################
  11151.6 |############################
  (13 below, 15 above range)

fpt_table_16 (n=160, range 10993.1-11794.7 ns)
  10993.1 |####################
  11033.2 |########
  11073.3 |###########
  11113.3 |####
  11153.4 |######
  11193.5 |########
  11233.6 |####
  11273.7 |####################
  11313.7 |###############################
  11353.8 |########################
  11393.9 |########
  11434.0 |######
  11474.1 |###############
  11514.1 |###############
  11554.2 |######
  11594.3 |########################################
  11634.4 |#############
  11674.5 |#############
  11714.5 |#############
  11754.6 |########
  (15 below, 18 above range)

```
