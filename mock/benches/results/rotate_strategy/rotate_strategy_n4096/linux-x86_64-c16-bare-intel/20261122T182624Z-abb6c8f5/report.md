# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 2.9x slower than the field

rot_variable_amount (862 ns) is 2.9x the fastest (293 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (rot_manual_shifts, rot_intrinsic_const) are a dead heat (<1%)

rot_manual_shifts (293 ns) and rot_intrinsic_const (293 ns) differ by 0.03%, inside the noise, even though the wider field spreads 194.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rot_manual_shifts's comparison is tie-heavy (17% tied pairs)

17% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_manual_shifts is inconsistent: worst-20% is 3.3x its best-20%

rot_manual_shifts's best 20% of batches run at 240 ns but its worst 20% at 786 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rot_manual_shifts** at 293.1 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.94x (fastest 293.1 ns, slowest 862.2 ns)

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
| Counter frequency | 2112817215 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2768ns | 2171ns | 1777ns | 2164ns | 5570ns | base |
| rot_manual_shifts | 2766ns | 2168ns | 1774ns | 2054ns | 5896ns | -0.06% |
| rot_variable_amount | 2968ns | 2741ns | 2243ns | 2558ns | 4923ns | +7.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 373ns | 240ns | 747ns | base | 10.993 |
| rot_manual_shifts | 371ns | 240ns | 786ns | -0.36% | 11.032 |
| rot_variable_amount | 934ns | 705ns | 1554ns | +150.73% | 4.384 |

## Performance model

- Peak throughput: **17.081 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.970 | 81.8% |
| rot_manual_shifts | 13.975 | 81.8% |
| rot_variable_amount | 4.751 | 27.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2768ns | 2768ns | base |
| rot_manual_shifts | 2766ns | 2766ns | -0.06% |
| rot_variable_amount | 2968ns | 2968ns | +7.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 293ns | base | --- | [293, 293] | --- | --- | --- | --- |
| rot_manual_shifts | 293ns | no significant difference | [-0, +0]ns | [293, 293] | no (adj: YES) | 0.0053 | 0.0053 | **27** (17%, HIGH) |
| rot_variable_amount | 862ns | +557.5ns (+190.1%) | [+466, +569]ns | [789, 862] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 293ns | -18.2% | +194.0% |
| 1 | 2 | 240ns | -0.0% | +194.0% |
| 1 | 3 | 293ns | +0.0% | +140.6% |
| 1 | 4 | 293ns | -18.2% | +194.1% |
| 1 | 5 | 293ns | -0.0% | +194.1% |
| 1 | 6 | 293ns | +0.0% | +194.1% |
| 1 | 7 | 293ns | +0.0% | +140.7% |
| 1 | 8 | 240ns | +0.0% | +194.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.208 | moderate- |
| rot_manual_shifts | -0.125 | ok |
| rot_variable_amount | 0.125 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 2/8, lost 0/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 30.0ns | 372.6ns | 8.0% | HIGH |
| rot_manual_shifts | 29.9ns | 371.3ns | 8.0% | HIGH |
| rot_variable_amount | 25.6ns | 934.2ns | 2.7% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 239.9-746.7 ns)
    239.9 |##########
    265.2 |
    290.6 |########################################
    315.9 |
    341.2 |
    366.6 |
    391.9 |
    417.3 |
    442.6 |
    467.9 |
    493.3 |
    518.6 |
    543.9 |
    569.3 |
    594.6 |
    620.0 |
    645.3 |
    670.6 |
    696.0 |
    721.3 |
  (9 below, 19 above range)

rot_manual_shifts (n=160, range 239.8-785.6 ns)
    239.8 |####################
    267.1 |########################################
    294.4 |
    321.7 |
    349.0 |
    376.2 |
    403.5 |
    430.8 |#
    458.1 |
    485.4 |
    512.7 |
    540.0 |
    567.3 |
    594.6 |
    621.8 |
    649.1 |
    676.4 |
    703.7 |#
    731.0 |###
    758.3 |#
  (28 below, 15 above range)

rot_variable_amount (n=160, range 705.4-1553.6 ns)
    705.4 |########################
    747.8 |
    790.2 |
    832.6 |########################################
    875.0 |##
    917.5 |
    959.9 |
   1002.3 |
   1044.7 |
   1087.1 |
   1129.5 |
   1171.9 |
   1214.3 |
   1256.7 |
   1299.1 |
   1341.6 |
   1384.0 |
   1426.4 |
   1468.8 |
   1511.2 |
  (24 below, 15 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=61.2% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **rot_manual_shifts**: CV=60.7% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **rot_variable_amount**: CV=48.6% (high variance, measurements may be unstable)
