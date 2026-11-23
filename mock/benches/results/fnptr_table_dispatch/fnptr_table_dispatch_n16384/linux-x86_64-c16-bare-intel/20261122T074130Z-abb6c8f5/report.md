# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 12% faster than the next best (fpt_table_16)

fpt_match_16 (11.19 us) leads fpt_table_16 (12.51 us) by 12%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 shows alternating (throttle bounce) (autocorr -0.71)

fpt_table_16's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (11.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 11190.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.12x (fastest 11190.2 ns, slowest 12506.7 ns)

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
| Counter frequency | 2112768962 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 13364ns | 12746ns | 11915ns | 13189ns | 15340ns | base |
| fpt_table_16 | 14828ns | 14083ns | 13019ns | 14648ns | 17179ns | +10.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 11674ns | 10359ns | 13434ns | base | 1.403 |
| fpt_table_16 | 13142ns | 11468ns | 15272ns | +12.57% | 1.247 |

## Performance model

- Peak throughput: **1.582 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.464 | 92.6% |
| fpt_table_16 | 1.310 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 13364ns | 13364ns | base |
| fpt_table_16 | 14828ns | 14828ns | +10.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 11190ns | base | --- | [10920, 11918] | --- | --- | --- | --- |
| fpt_table_16 | 12507ns | +1570.9ns (+14.0%) | [+1389, +1692]ns | [12369, 12741] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 10607ns | +15.2% |
| 1 | 2 | 10802ns | +40.0% |
| 1 | 3 | 10800ns | +8.5% |
| 1 | 4 | 10582ns | +42.5% |
| 1 | 5 | 12608ns | -8.9% |
| 1 | 6 | 10588ns | +19.0% |
| 1 | 7 | 12916ns | +13.2% |
| 1 | 8 | 13129ns | -9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | 0.080 | ok |
| fpt_table_16 | -0.710 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fpt_table_16**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 22.9ns | 11674.3ns | 0.2% |  |
| fpt_table_16 | 21.8ns | 13142.0ns | 0.2% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 10358.6-13433.5 ns)
  10358.6 |###################################
  10512.3 |########################################
  10666.1 |#######################
  10819.8 |##################
  10973.6 |##########
  11127.3 |##############
  11281.1 |########
  11434.8 |##
  11588.6 |######
  11742.3 |##
  11896.0 |####
  12049.8 |####
  12203.5 |################
  12357.3 |########
  12511.0 |##########
  12664.8 |############
  12818.5 |#############################
  12972.3 |##################
  13126.0 |####
  13279.8 |########
  (15 below, 12 above range)

fpt_table_16 (n=160, range 11468.4-15272.3 ns)
  11468.4 |########################################
  11658.5 |############
  11848.7 |#####################################
  12038.9 |######################
  12229.1 |###################################
  12419.3 |################################
  12609.5 |####################
  12799.7 |#######
  12989.9 |#####
  13180.1 |
  13370.3 |##
  13560.5 |
  13750.7 |
  13940.9 |
  14131.1 |#####
  14321.3 |#################
  14511.5 |#########################
  14701.7 |#########################
  14891.9 |####################
  15082.1 |###########################
  (14 below, 12 above range)

```
