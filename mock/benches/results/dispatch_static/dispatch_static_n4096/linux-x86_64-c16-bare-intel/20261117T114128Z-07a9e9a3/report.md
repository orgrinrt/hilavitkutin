# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (577 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_direct at 473 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### dispatch_direct dominates: 22% faster than the next best (dispatch_trait)

dispatch_direct (473 ns) leads dispatch_trait (577 ns) by 22%, a clear separation rather than a photo finish. CV 79.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_direct is fastest but the noisiest (CV 79.0%)

dispatch_direct wins on median (473 ns) yet has the highest variance (CV 79.0%), while dispatch_const_ptr is the steadiest (CV 61.2%, 577 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (105 ns) is smaller than the fastest variant's own run-to-run std-dev (374 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_trait is inconsistent: worst-20% is 3.1x its best-20%

dispatch_trait's best 20% of batches run at 472 ns but its worst 20% at 1.47 us (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### dispatch_direct's edge over baseline is significant but tiny (-0 ns, 0.07%)

dispatch_direct differs from baseline dispatch_const_ptr by -0 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_direct** at 472.6 ns median (-18.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 472.6 ns, slowest 577.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 2112808726 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2856ns | 2458ns | 2012ns | 2336ns | 5261ns | base |
| dispatch_direct | 2772ns | 2028ns | 2012ns | 2223ns | 5180ns | -2.96% |
| dispatch_trait | 3065ns | 2459ns | 2012ns | 2336ns | 6302ns | +7.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 668ns | 472ns | 1224ns | base | 6.130 |
| dispatch_direct | 647ns | 472ns | 1201ns | -3.15% | 6.329 |
| dispatch_trait | 717ns | 472ns | 1466ns | +7.25% | 5.716 |

## Performance model

- Peak throughput: **8.671 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 7.093 | 81.8% |
| dispatch_direct | 8.667 | 100.0% |
| dispatch_trait | 7.094 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2856ns | 2856ns | base |
| dispatch_direct | 2772ns | 2772ns | -2.96% |
| dispatch_trait | 3065ns | 3065ns | +7.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 577ns | base | --- | [577, 578] | --- | --- | --- | --- |
| dispatch_direct | 473ns | -0.4ns (-0.1%) | [-23, -0]ns | [473, 577] | YES | 0.0009 | 0.0004 | 8 |
| dispatch_trait | 577ns | no significant difference | [-13, +0]ns | [563, 578] | no | 0.5703 | 0.5703 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 578ns | -0.0% | -18.2% |
| 1 | 2 | 578ns | -18.2% | +0.0% |
| 1 | 3 | 473ns | -0.0% | +22.2% |
| 1 | 4 | 473ns | +22.2% | +0.0% |
| 1 | 5 | 473ns | +22.2% | +22.2% |
| 1 | 6 | 473ns | -0.1% | +107.9% |
| 1 | 7 | 578ns | -18.2% | -18.2% |
| 1 | 8 | 578ns | -18.2% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.376 | moderate+ |
| dispatch_direct | 0.059 | ok |
| dispatch_trait | -0.150 | ok |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 2/8
- **dispatch_trait**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 27.6ns | 668.2ns | 4.1% |  |
| dispatch_direct | 27.0ns | 647.1ns | 4.2% |  |
| dispatch_trait | 29.6ns | 716.6ns | 4.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 472.5-1224.2 ns)
    472.5 |##########################
    510.1 |#
    547.6 |########################################
    585.2 |#####
    622.8 |
    660.4 |
    698.0 |
    735.6 |
    773.2 |
    810.7 |
    848.3 |
    885.9 |
    923.5 |
    961.1 |
    998.7 |
   1036.3 |
   1073.8 |
   1111.4 |
   1149.0 |
   1186.6 |
  (13 below, 19 above range)

dispatch_direct (n=160, range 472.4-1201.0 ns)
    472.4 |########################################
    508.8 |
    545.2 |##########################
    581.7 |#
    618.1 |
    654.5 |
    691.0 |
    727.4 |
    763.8 |
    800.2 |
    836.7 |
    873.1 |
    909.5 |
    946.0 |
    982.4 |
   1018.8 |
   1055.3 |
   1091.7 |
   1128.1 |
   1164.5 |
  (4 below, 17 above range)

dispatch_trait (n=160, range 472.5-1465.8 ns)
    472.5 |#######################################
    522.1 |
    571.8 |########################################
    621.5 |
    671.1 |##
    720.8 |#
    770.5 |
    820.1 |
    869.8 |
    919.5 |
    969.1 |
   1018.8 |
   1068.5 |
   1118.1 |
   1167.8 |
   1217.5 |#
   1267.1 |
   1316.8 |#
   1366.5 |#
   1416.1 |
  (14 below, 18 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=52.9% (high variance, measurements may be unstable)
- **dispatch_direct**: CV=57.7% (high variance, measurements may be unstable)
- **dispatch_trait**: CV=57.1% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.1x (possible bimodal distribution)
