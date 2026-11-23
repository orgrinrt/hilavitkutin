# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 577 ns to slowest 578 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (382 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader dispatch_direct vs stability leader dispatch_const_ptr (+0% speed for 1.0x steadier)

dispatch_direct is fastest (577 ns, CV 66.1%); dispatch_const_ptr gives up 0.0% median for 1.0x lower variance (CV 63.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 2.9x its best-20%

dispatch_trait's best 20% of batches run at 472 ns but its worst 20% at 1.38 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_direct** at 577.4 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 577.4 ns, slowest 577.5 ns)

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
| Counter frequency | 2112810234 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2851ns | 2454ns | 2007ns | 2364ns | 5156ns | base |
| dispatch_direct | 2891ns | 2454ns | 2007ns | 2360ns | 5369ns | +1.40% |
| dispatch_trait | 2977ns | 2454ns | 2007ns | 2315ns | 5933ns | +4.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 668ns | 472ns | 1200ns | base | 6.131 |
| dispatch_direct | 677ns | 472ns | 1249ns | +1.37% | 6.048 |
| dispatch_trait | 697ns | 472ns | 1381ns | +4.34% | 5.876 |

## Performance model

- Peak throughput: **8.670 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 7.093 | 81.8% |
| dispatch_direct | 7.094 | 81.8% |
| dispatch_trait | 7.093 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2851ns | 2851ns | base |
| dispatch_direct | 2891ns | 2891ns | +1.40% |
| dispatch_trait | 2977ns | 2977ns | +4.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 578ns | base | --- | [578, 578] | --- | --- | --- | --- |
| dispatch_direct | 577ns | no significant difference | [-0, +0]ns | [577, 577] | no | 0.2907 | 0.1454 | 7 |
| dispatch_trait | 578ns | no significant difference | [-0, +0]ns | [523, 578] | no | 0.9358 | 0.9358 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 473ns | +22.1% | +22.2% |
| 1 | 2 | 578ns | -18.2% | -18.2% |
| 1 | 3 | 578ns | -18.2% | -18.2% |
| 1 | 4 | 578ns | -18.2% | +0.0% |
| 1 | 5 | 473ns | +22.2% | -0.0% |
| 1 | 6 | 473ns | +22.2% | +22.2% |
| 1 | 7 | 578ns | -0.0% | +39.2% |
| 1 | 8 | 578ns | -0.0% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.059 | ok |
| dispatch_direct | 0.392 | moderate+ |
| dispatch_trait | -0.170 | ok |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 3/8
- **dispatch_trait**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 27.5ns | 668.1ns | 4.1% |  |
| dispatch_direct | 28.0ns | 677.3ns | 4.1% |  |
| dispatch_trait | 28.7ns | 697.1ns | 4.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 472.5-1200.0 ns)
    472.5 |###################
    508.9 |
    545.2 |########################################
    581.6 |#
    618.0 |
    654.4 |
    690.7 |
    727.1 |
    763.5 |
    799.9 |
    836.2 |
    872.6 |
    909.0 |
    945.4 |
    981.7 |
   1018.1 |
   1054.5 |
   1090.9 |
   1127.2 |
   1163.6 |
  (11 below, 18 above range)

dispatch_direct (n=160, range 472.4-1249.4 ns)
    472.4 |#################
    511.3 |
    550.1 |########################################
    589.0 |
    627.8 |
    666.7 |
    705.5 |
    744.4 |
    783.2 |
    822.0 |
    860.9 |
    899.7 |
    938.6 |
    977.4 |
   1016.3 |
   1055.1 |
   1094.0 |
   1132.8 |
   1171.7 |
   1210.5 |
  (17 below, 18 above range)

dispatch_trait (n=160, range 472.5-1381.5 ns)
    472.5 |########################################
    517.9 |##
    563.4 |#####################################
    608.8 |#
    654.3 |
    699.7 |#
    745.2 |
    790.6 |
    836.1 |
    881.5 |#
    927.0 |
    972.4 |#
   1017.9 |#
   1063.3 |
   1108.8 |
   1154.2 |#
   1199.7 |#
   1245.1 |
   1290.6 |
   1336.0 |
  (11 below, 17 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=54.5% (high variance, measurements may be unstable)
- **dispatch_direct**: CV=56.3% (high variance, measurements may be unstable)
- **dispatch_trait**: CV=55.0% (high variance, measurements may be unstable)
