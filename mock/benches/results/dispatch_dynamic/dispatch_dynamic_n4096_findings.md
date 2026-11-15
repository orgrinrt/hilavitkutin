# Dynamic dispatch shapes: direct vs opaque fn pointer vs runtime table

3 variants, 160 samples per variant.
Baseline: **dispatch_direct**

## Highlights

Baseline for all deltas below: **dispatch_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_runtime shows alternating (throttle bounce) (autocorr -0.59)

dispatch_runtime's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dispatch_direct)

The baseline dispatch_direct is the fastest (636 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (dispatch_direct) is the fastest** at 636.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 636.2 ns, slowest 1165.7 ns)

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
| dispatch_direct | 2923ns | 2873ns | 2770ns | 2915ns | 3100ns | base |
| dispatch_runtime | 3351ns | 3353ns | 3185ns | 3334ns | 3566ns | +14.63% |
| dispatch_volatile | 2886ns | 2879ns | 2716ns | 2862ns | 3128ns | -1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_direct | 646ns | 610ns | 689ns | base | 6.337 |
| dispatch_runtime | 1163ns | 1098ns | 1243ns | +79.93% | 3.522 |
| dispatch_volatile | 679ns | 638ns | 737ns | +5.05% | 6.032 |

## Performance model

- Peak throughput: **6.712 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_direct | 6.438 | 95.9% |
| dispatch_runtime | 3.514 | 52.4% |
| dispatch_volatile | 6.049 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_direct | 2923ns | 2923ns | base |
| dispatch_runtime | 3351ns | 3351ns | +14.63% |
| dispatch_volatile | 2886ns | 2886ns | -1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_direct | 636ns | base | --- | [635, 638] | --- | --- | --- | --- |
| dispatch_runtime | 1166ns | +498.5ns (+78.4%) | [+493, +529]ns | [1148, 1173] | YES | 0.0000 | 0.0000 | 0 |
| dispatch_volatile | 677ns | +31.7ns (+5.0%) | [+28, +45]ns | [666, 681] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_direct | dispatch_runtime | dispatch_volatile |
|---|---|---|---|---|
| 1 | 1 | 636ns | +74.8% | +5.7% |
| 1 | 2 | 617ns | +92.2% | +7.2% |
| 1 | 3 | 634ns | +87.9% | +0.5% |
| 1 | 4 | 633ns | +78.9% | +15.5% |
| 1 | 5 | 679ns | +86.3% | -2.2% |
| 1 | 6 | 648ns | +69.0% | +5.6% |
| 1 | 7 | 682ns | +71.8% | +1.7% |
| 1 | 8 | 636ns | +79.0% | +5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_direct | 0.025 | ok |
| dispatch_runtime | -0.592 | HIGH- (thermal bounce) |
| dispatch_volatile | -0.419 | moderate- |

**Consistency summary:**

- **dispatch_runtime**: won 0/8, lost 8/8
- **dispatch_volatile**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_direct | 3.0ns | 646.4ns | 0.5% |  |
| dispatch_runtime | 3.7ns | 1163.0ns | 0.3% |  |
| dispatch_volatile | 3.4ns | 679.0ns | 0.5% |  |

## Distribution (algo ns)

```
dispatch_direct (n=160, range 610.2-689.3 ns)
    610.2 |####
    614.2 |###
    618.1 |#
    622.1 |#
    626.0 |######
    630.0 |##################################
    634.0 |########################################
    637.9 |#######
    641.9 |##
    645.8 |#
    649.8 |
    653.7 |###
    657.7 |
    661.6 |
    665.6 |
    669.5 |#
    673.5 |###
    677.4 |################
    681.4 |#######################
    685.3 |######
  (15 below, 4 above range)

dispatch_runtime (n=160, range 1097.8-1243.1 ns)
   1097.8 |##############
   1105.1 |########################
   1112.4 |##########
   1119.6 |##########
   1126.9 |##########################
   1134.2 |##################
   1141.4 |######
   1148.7 |######
   1155.9 |########
   1163.2 |######################
   1170.5 |########################################
   1177.7 |################
   1185.0 |########################
   1192.3 |########################
   1199.5 |##
   1206.8 |
   1214.0 |##
   1221.3 |
   1228.6 |##
   1235.8 |##
  (14 below, 18 above range)

dispatch_volatile (n=160, range 637.8-737.4 ns)
    637.8 |####################
    642.8 |#####
    647.7 |#####
    652.7 |################
    657.7 |##############################
    662.7 |###############
    667.7 |########
    672.7 |######
    677.6 |############################
    682.6 |########################################
    687.6 |#
    692.6 |###
    697.6 |#####
    702.5 |
    707.5 |
    712.5 |
    717.5 |
    722.5 |#
    727.5 |##########
    732.4 |####################
  (17 below, 13 above range)

```
