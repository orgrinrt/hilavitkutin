# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 2.9x slower than the field

rot_variable_amount (862 ns) is 2.9x the fastest (293 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_intrinsic_const is fastest but the noisiest (CV 78.7%)

rot_intrinsic_const wins on median (293 ns) yet has the highest variance (CV 78.7%), while rot_variable_amount is the steadiest (CV 59.5%, 862 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (rot_intrinsic_const, rot_manual_shifts) are a dead heat (<1%)

rot_intrinsic_const (293 ns) and rot_manual_shifts (293 ns) differ by 0.00%, inside the noise, even though the wider field spreads 194.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rot_intrinsic_const)

The baseline rot_intrinsic_const is the fastest (293 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### rot_manual_shifts's comparison is tie-heavy (12% tied pairs)

12% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_intrinsic_const is inconsistent: worst-20% is 3.4x its best-20%

rot_intrinsic_const's best 20% of batches run at 240 ns but its worst 20% at 810 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rot_intrinsic_const) is the fastest** at 293.1 ns median
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
| Counter frequency | 2112804954 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2781ns | 2176ns | 1780ns | 2014ns | 6081ns | base |
| rot_manual_shifts | 2776ns | 2173ns | 1779ns | 2035ns | 5995ns | -0.17% |
| rot_variable_amount | 3116ns | 2742ns | 2245ns | 2553ns | 5675ns | +12.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 373ns | 240ns | 810ns | base | 10.988 |
| rot_manual_shifts | 372ns | 240ns | 797ns | -0.29% | 11.020 |
| rot_variable_amount | 976ns | 705ns | 1771ns | +161.83% | 4.197 |

## Performance model

- Peak throughput: **17.081 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 13.975 | 81.8% |
| rot_manual_shifts | 13.975 | 81.8% |
| rot_variable_amount | 4.751 | 27.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2781ns | 2781ns | base |
| rot_manual_shifts | 2776ns | 2776ns | -0.17% |
| rot_variable_amount | 3116ns | 3116ns | +12.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 293ns | base | --- | [240, 293] | --- | --- | --- | --- |
| rot_manual_shifts | 293ns | no significant difference | [-0, +0]ns | [240, 293] | no | 0.5543 | 0.5543 | **20** (12%, HIGH) |
| rot_variable_amount | 862ns | +569.0ns (+194.1%) | [+466, +569]ns | [729, 862] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 293ns | -18.2% | +194.1% |
| 1 | 2 | 293ns | +0.0% | +140.6% |
| 1 | 3 | 240ns | +22.2% | +259.5% |
| 1 | 4 | 240ns | +0.0% | +194.1% |
| 1 | 5 | 240ns | +0.0% | +194.1% |
| 1 | 6 | 240ns | +0.0% | +259.4% |
| 1 | 7 | 240ns | +22.2% | +259.3% |
| 1 | 8 | 293ns | -18.2% | +140.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | 0.259 | moderate+ |
| rot_manual_shifts | -0.142 | ok |
| rot_variable_amount | -0.375 | moderate- |

**Consistency summary:**

- **rot_manual_shifts**: won 2/8, lost 2/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 30.0ns | 372.8ns | 8.0% | HIGH |
| rot_manual_shifts | 30.0ns | 371.7ns | 8.1% | HIGH |
| rot_variable_amount | 26.7ns | 976.0ns | 2.7% |  |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 239.8-810.3 ns)
    239.8 |#######################################
    268.3 |########################################
    296.9 |
    325.4 |##
    353.9 |
    382.4 |
    411.0 |
    439.5 |
    468.0 |
    496.5 |
    525.0 |
    553.6 |
    582.1 |
    610.6 |
    639.1 |
    667.7 |##
    696.2 |##
    724.7 |#
    753.2 |###
    781.7 |#
  (24 below, 15 above range)

rot_manual_shifts (n=160, range 239.8-797.2 ns)
    239.8 |########################################
    267.7 |#####################################
    295.5 |#
    323.4 |
    351.3 |#
    379.2 |
    407.0 |
    434.9 |
    462.8 |
    490.6 |
    518.5 |
    546.4 |
    574.3 |
    602.1 |
    630.0 |
    657.9 |##
    685.7 |###
    713.6 |###
    741.5 |##
    769.4 |##
  (27 below, 14 above range)

rot_variable_amount (n=160, range 705.4-1770.5 ns)
    705.4 |########################################
    758.6 |
    811.9 |#######################################
    865.1 |###
    918.4 |#
    971.7 |
   1024.9 |
   1078.2 |
   1131.4 |
   1184.7 |
   1237.9 |
   1291.2 |
   1344.5 |
   1397.7 |
   1451.0 |
   1504.2 |
   1557.5 |
   1610.7 |
   1664.0 |
   1717.3 |
  (7 below, 19 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=61.8% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **rot_manual_shifts**: CV=59.6% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **rot_variable_amount**: CV=52.6% (high variance, measurements may be unstable)
