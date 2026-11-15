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

### rot_manual_shifts's comparison is tie-heavy (22% tied pairs)

22% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_intrinsic_const is inconsistent: worst-20% is 3.1x its best-20%

rot_intrinsic_const's best 20% of batches run at 270 ns but its worst 20% at 833 ns (3.1x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112768874 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2956ns | 2171ns | 1995ns | 2183ns | 6235ns | base |
| rot_manual_shifts | 2602ns | 2167ns | 1773ns | 2026ns | 5161ns | -11.98% |
| rot_variable_amount | 3039ns | 2740ns | 2241ns | 2646ns | 5019ns | +2.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 398ns | 270ns | 833ns | base | 10.300 |
| rot_manual_shifts | 351ns | 240ns | 693ns | -11.82% | 11.680 |
| rot_variable_amount | 954ns | 705ns | 1567ns | +139.81% | 4.295 |

## Performance model

- Peak throughput: **17.080 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.970 | 81.8% |
| rot_manual_shifts | 13.975 | 81.8% |
| rot_variable_amount | 4.751 | 27.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2956ns | 2956ns | base |
| rot_manual_shifts | 2602ns | 2602ns | -11.98% |
| rot_variable_amount | 3039ns | 3039ns | +2.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 293ns | base | --- | [293, 293] | --- | --- | --- | --- |
| rot_manual_shifts | 293ns | no significant difference | [-11, +0]ns | [272, 293] | no (adj: YES) | 0.0000 | 0.0000 | **36** (22%, HIGH) |
| rot_variable_amount | 862ns | +569.0ns (+194.1%) | [+569, +569]ns | [862, 862] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 240ns | +0.0% | +259.4% |
| 1 | 2 | 293ns | +0.0% | +194.1% |
| 1 | 3 | 293ns | -18.2% | +140.7% |
| 1 | 4 | 293ns | -18.2% | +194.1% |
| 1 | 5 | 293ns | +0.0% | +194.1% |
| 1 | 6 | 293ns | +0.0% | +194.1% |
| 1 | 7 | 293ns | -18.2% | +140.6% |
| 1 | 8 | 293ns | +0.0% | +140.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.018 | ok |
| rot_manual_shifts | -0.375 | moderate- |
| rot_variable_amount | 0.059 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 3/8, lost 0/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 32.2ns | 397.7ns | 8.1% | HIGH |
| rot_manual_shifts | 28.4ns | 350.7ns | 8.1% | HIGH |
| rot_variable_amount | 26.2ns | 953.6ns | 2.7% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 269.8-833.2 ns)
    269.8 |########################################
    298.0 |#
    326.1 |
    354.3 |
    382.5 |
    410.6 |
    438.8 |
    467.0 |
    495.2 |
    523.3 |
    551.5 |
    579.7 |
    607.8 |
    636.0 |
    664.2 |
    692.3 |
    720.5 |
    748.7 |
    776.9 |
    805.0 |
  (14 below, 19 above range)

rot_manual_shifts (n=160, range 239.8-692.7 ns)
    239.8 |#######################
    262.5 |
    285.1 |########################################
    307.7 |
    330.4 |
    353.0 |
    375.7 |
    398.3 |
    421.0 |
    443.6 |
    466.3 |
    488.9 |
    511.5 |
    534.2 |
    556.8 |
    579.5 |
    602.1 |
    624.8 |
    647.4 |
    670.1 |
  (28 below, 18 above range)

rot_variable_amount (n=160, range 705.4-1567.4 ns)
    705.4 |###############
    748.5 |
    791.6 |
    834.7 |########################################
    877.8 |#
    920.9 |
    964.0 |
   1007.1 |
   1050.2 |
   1093.3 |
   1136.4 |
   1179.5 |
   1222.6 |
   1265.7 |
   1308.8 |
   1351.9 |
   1395.0 |
   1438.1 |
   1481.2 |
   1524.3 |
  (16 below, 15 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=58.4% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **rot_manual_shifts**: CV=63.6% (high variance, measurements may be unstable)
- **rot_variable_amount**: CV=49.0% (high variance, measurements may be unstable)
