# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 3.2x slower than the field

rot_variable_amount (968 ns) is 3.2x the fastest (300 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_manual_shifts is fastest but the noisiest (CV 6.5%)

rot_manual_shifts wins on median (300 ns) yet has the highest variance (CV 6.5%), while rot_variable_amount is the steadiest (CV 0.9%, 968 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (rot_manual_shifts, rot_intrinsic_const) are a dead heat (<1%)

rot_manual_shifts (300 ns) and rot_intrinsic_const (301 ns) differ by 0.13%, inside the noise, even though the wider field spreads 222.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 3.2x the fastest

Fastest rot_manual_shifts (300 ns) to slowest rot_variable_amount (968 ns): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rot_manual_shifts** at 300.2 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.22x (fastest 300.2 ns, slowest 967.7 ns)

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
| rot_intrinsic_const | 2431ns | 2426ns | 2420ns | 2427ns | 2453ns | base |
| rot_manual_shifts | 2430ns | 2426ns | 2421ns | 2426ns | 2452ns | -0.04% |
| rot_variable_amount | 3098ns | 3092ns | 3085ns | 3092ns | 3131ns | +27.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 301ns | 296ns | 307ns | base | 13.608 |
| rot_manual_shifts | 302ns | 296ns | 314ns | +0.47% | 13.544 |
| rot_variable_amount | 970ns | 964ns | 982ns | +222.25% | 4.223 |

## Performance model

- Peak throughput: **13.840 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.626 | 98.5% |
| rot_manual_shifts | 13.644 | 98.6% |
| rot_variable_amount | 4.233 | 30.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2431ns | 2431ns | base |
| rot_manual_shifts | 2430ns | 2430ns | -0.04% |
| rot_variable_amount | 3098ns | 3098ns | +27.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 301ns | base | --- | [300, 301] | --- | --- | --- | --- |
| rot_manual_shifts | 300ns | no significant difference | [-0, +1]ns | [300, 301] | no | 0.5152 | 0.5152 | 9 |
| rot_variable_amount | 968ns | +667.5ns (+222.1%) | [+667, +668]ns | [967, 969] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 300ns | +0.3% | +222.7% |
| 1 | 2 | 301ns | -0.4% | +220.9% |
| 1 | 3 | 301ns | -0.2% | +222.0% |
| 1 | 4 | 301ns | -0.3% | +221.7% |
| 1 | 5 | 299ns | +0.3% | +223.3% |
| 1 | 6 | 303ns | -0.5% | +219.3% |
| 1 | 7 | 300ns | +0.2% | +222.8% |
| 1 | 8 | 299ns | +0.3% | +223.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.473 | moderate- |
| rot_manual_shifts | -0.006 | ok |
| rot_variable_amount | 0.157 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 4/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 2.7ns | 301.0ns | 0.9% |  |
| rot_manual_shifts | 2.5ns | 302.4ns | 0.8% |  |
| rot_variable_amount | 2.5ns | 970.0ns | 0.3% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 296.1-306.9 ns)
    296.1 |################
    296.6 |##########################
    297.2 |#############
    297.7 |########################
    298.3 |########################################
    298.8 |################
    299.3 |################
    299.9 |##########################
    300.4 |##########
    301.0 |########################
    301.5 |#############
    302.0 |##########################
    302.6 |#####################
    303.1 |########
    303.7 |################
    304.2 |################
    304.7 |########
    305.3 |##################
    305.8 |##
    306.4 |##
  (13 below, 17 above range)

rot_manual_shifts (n=160, range 296.0-314.4 ns)
    296.0 |########
    296.9 |############################
    297.8 |##########################
    298.7 |########################################
    299.6 |######################################
    300.6 |######################
    301.5 |################
    302.4 |##########################
    303.3 |##############
    304.2 |############################
    305.2 |####
    306.1 |####
    307.0 |############
    307.9 |############
    308.8 |##
    309.8 |####
    310.7 |
    311.6 |
    312.5 |
    313.4 |
  (16 below, 2 above range)

rot_variable_amount (n=160, range 963.6-981.7 ns)
    963.6 |#############
    964.5 |#########################
    965.4 |########################################
    966.3 |###################
    967.2 |########
    968.1 |##########
    969.0 |###########
    969.9 |################
    970.8 |#################
    971.7 |###########
    972.6 |####
    973.5 |##
    974.4 |#####
    975.3 |########
    976.2 |
    977.1 |#
    978.0 |#####
    978.9 |#
    979.8 |
    980.8 |
  (11 below, 10 above range)

```
