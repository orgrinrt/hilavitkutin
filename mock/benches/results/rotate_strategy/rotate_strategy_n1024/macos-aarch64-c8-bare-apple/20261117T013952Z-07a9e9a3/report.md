# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.3x slower than the field

rot_variable_amount (236 ns) is 3.3x the fastest (71 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const shows alternating (throttle bounce) (autocorr -0.52)

rot_intrinsic_const's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.3x the fastest

Fastest rot_manual_shifts (71 ns) to slowest rot_variable_amount (236 ns): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rot_manual_shifts** at 71.2 ns median (-1.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.32x (fastest 71.2 ns, slowest 236.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2362ns | 2180ns | 2147ns | 2210ns | 3032ns | base |
| rot_manual_shifts | 2216ns | 2192ns | 2149ns | 2197ns | 2338ns | -6.18% |
| rot_variable_amount | 2384ns | 2381ns | 2309ns | 2375ns | 2483ns | +0.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 73ns | 66ns | 81ns | base | 14.031 |
| rot_manual_shifts | 72ns | 66ns | 80ns | -0.92% | 14.161 |
| rot_variable_amount | 237ns | 228ns | 248ns | +224.53% | 4.324 |

## Performance model

- Peak throughput: **15.552 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 14.202 | 91.3% |
| rot_manual_shifts | 14.382 | 92.5% |
| rot_variable_amount | 4.335 | 27.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2362ns | 2362ns | base |
| rot_manual_shifts | 2216ns | 2216ns | -6.18% |
| rot_variable_amount | 2384ns | 2384ns | +0.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 72ns | base | --- | [71, 74] | --- | --- | --- | --- |
| rot_manual_shifts | 71ns | no significant difference | [-2, +0]ns | [70, 72] | no | 0.0520 | 0.0520 | 7 |
| rot_variable_amount | 236ns | +163.8ns (+227.1%) | [+162, +165]ns | [235, 238] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 74ns | -2.3% | +221.8% |
| 1 | 2 | 69ns | +8.7% | +250.5% |
| 1 | 3 | 70ns | +0.9% | +229.0% |
| 1 | 4 | 75ns | -4.8% | +227.9% |
| 1 | 5 | 70ns | +2.0% | +234.3% |
| 1 | 6 | 76ns | -6.2% | +200.8% |
| 1 | 7 | 68ns | +2.2% | +236.1% |
| 1 | 8 | 70ns | +0.0% | +244.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.521 | HIGH- (thermal bounce) |
| rot_manual_shifts | 0.205 | moderate+ |
| rot_variable_amount | -0.173 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 3/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 2.9ns | 73.0ns | 3.9% |  |
| rot_manual_shifts | 3.0ns | 72.3ns | 4.1% |  |
| rot_variable_amount | 2.9ns | 236.8ns | 1.2% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 66.2-81.5 ns)
     66.2 |###
     67.0 |##############################
     67.8 |##############################
     68.5 |####################################
     69.3 |########################
     70.1 |########################################
     70.8 |###############
     71.6 |####################################
     72.3 |###############
     73.1 |##################
     73.9 |##############################
     74.6 |#########
     75.4 |#########
     76.2 |#####################
     76.9 |#####################
     77.7 |###############
     78.4 |#########
     79.2 |############
     80.0 |############
     80.7 |###############
  (15 below, 12 above range)

rot_manual_shifts (n=160, range 65.8-80.5 ns)
     65.8 |####
     66.6 |################
     67.3 |#########################
     68.0 |############################
     68.8 |#########
     69.5 |#######################
     70.2 |########################################
     71.0 |################################
     71.7 |##
     72.4 |###########
     73.2 |##############
     73.9 |#######################
     74.6 |##
     75.4 |##############
     76.1 |##############
     76.8 |##################
     77.6 |##############
     78.3 |####
     79.0 |####
     79.8 |
  (15 below, 15 above range)

rot_variable_amount (n=160, range 227.6-247.8 ns)
    227.6 |########################################
    228.6 |####################################
    229.6 |################################
    230.6 |##############
    231.6 |##################
    232.6 |#############################
    233.7 |########################################
    234.7 |#####################
    235.7 |##################
    236.7 |################################
    237.7 |#########################
    238.7 |####################################
    239.7 |##############
    240.7 |#############################
    241.7 |#########################
    242.8 |#############################
    243.8 |#############################
    244.8 |##########
    245.8 |##################
    246.8 |#######
  (12 below, 8 above range)

```
