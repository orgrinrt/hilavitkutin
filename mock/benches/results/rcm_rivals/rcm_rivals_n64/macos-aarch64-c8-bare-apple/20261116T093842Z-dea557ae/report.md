# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (184.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_spec at 183.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 183.59 us to slowest 184.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (925 ns) is smaller than the fastest variant's own run-to-run std-dev (1.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_spec** at 183589.6 ns median (-0.5% vs baseline)
- Spread: 1.01x (fastest 183589.6 ns, slowest 184514.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 187846ns | 187629ns | 186646ns | 187569ns | 188862ns | base |
| riv_rowm | 188842ns | 187983ns | 187147ns | 187944ns | 191036ns | +0.53% |
| riv_scr | 187963ns | 187977ns | 186226ns | 187748ns | 189152ns | +0.06% |
| riv_snake | 188316ns | 188031ns | 186256ns | 187712ns | 190252ns | +0.25% |
| riv_spec | 186900ns | 187433ns | 184222ns | 186920ns | 188207ns | -0.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 184389ns | 183188ns | 185307ns | base | 0.000 |
| riv_rowm | 184973ns | 183283ns | 187023ns | +0.32% | 0.000 |
| riv_scr | 184437ns | 182497ns | 185761ns | +0.03% | 0.000 |
| riv_snake | 184718ns | 183208ns | 186266ns | +0.18% | 0.000 |
| riv_spec | 183281ns | 180322ns | 184759ns | -0.60% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.7% |
| riv_rowm | 0.000 | 97.9% |
| riv_scr | 0.000 | 97.8% |
| riv_snake | 0.000 | 97.8% |
| riv_spec | 0.000 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 187846ns | 187846ns | base |
| riv_rowm | 188842ns | 188842ns | +0.53% |
| riv_scr | 187963ns | 187963ns | +0.06% |
| riv_snake | 188316ns | 188316ns | +0.25% |
| riv_spec | 186900ns | 186900ns | -0.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 184515ns | base | --- | [183600, 184904] | --- | --- | --- | --- |
| riv_rowm | 184260ns | no significant difference | [-475, +871]ns | [183869, 185594] | no | 1.0000 | 0.8145 | 0 |
| riv_scr | 184344ns | no significant difference | [-985, +1517]ns | [183854, 185488] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 184423ns | no significant difference | [-423, +377]ns | [183748, 185442] | no | 1.0000 | 1.0000 | 0 |
| riv_spec | 183590ns | no significant difference | [-2140, +621]ns | [183025, 184308] | no | 1.0000 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 184446ns | -0.2% | -0.0% | -0.4% | +0.3% |
| 1 | 2 | 184846ns | -0.2% | +0.3% | -0.2% | -2.6% |
| 1 | 3 | 184975ns | -0.6% | -0.5% | +0.3% | -1.0% |
| 2 | 1 | 183629ns | -0.1% | +0.4% | -0.1% | +0.4% |
| 2 | 2 | 183454ns | +0.7% | +1.3% | +0.6% | -0.0% |
| 2 | 3 | 184662ns | +0.1% | -1.0% | -0.4% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.128 | ok |
| riv_rowm | 0.065 | ok |
| riv_scr | -0.490 | moderate- |
| riv_snake | -0.443 | moderate- |
| riv_spec | -0.371 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 1/6
- **riv_scr**: won 2/6, lost 3/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 4266768.5ns | 184388.7ns | 2314.0% | HIGH |
| riv_rowm | 4264814.1ns | 184972.7ns | 2305.6% | HIGH |
| riv_scr | 4261263.4ns | 184436.6ns | 2310.4% | HIGH |
| riv_snake | 4273510.4ns | 184717.6ns | 2313.5% | HIGH |
| riv_spec | 4247013.0ns | 183281.5ns | 2317.2% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 183187.5-185306.9 ns)
  183187.5 |
  183293.5 |
  183399.4 |########################################
  183505.4 |########################################
  183611.4 |####################
  183717.4 |
  183823.3 |
  183929.3 |
  184035.3 |####################
  184141.3 |####################
  184247.2 |
  184353.2 |####################
  184459.2 |
  184565.1 |########################################
  184671.1 |
  184777.1 |########################################
  184883.1 |########################################
  184989.0 |
  185095.0 |
  185201.0 |########################################
  (1 below, 1 above range)

riv_rowm (n=18, range 183283.3-187022.9 ns)
  183283.3 |####################
  183470.3 |####################
  183657.3 |########################################
  183844.3 |########################################
  184031.2 |########################################
  184218.2 |
  184405.2 |####################
  184592.2 |########################################
  184779.2 |
  184966.1 |
  185153.1 |
  185340.1 |####################
  185527.1 |
  185714.1 |########################################
  185901.0 |
  186088.0 |
  186275.0 |
  186462.0 |####################
  186649.0 |
  186835.9 |
  (1 below, 2 above range)

riv_scr (n=18, range 182497.2-185761.1 ns)
  182497.2 |
  182660.4 |#############
  182823.6 |
  182986.8 |
  183150.0 |
  183313.2 |#############
  183476.4 |#############
  183639.6 |#############
  183802.8 |
  183966.0 |########################################
  184129.2 |
  184292.4 |########################################
  184455.5 |
  184618.7 |
  184781.9 |
  184945.1 |
  185108.3 |
  185271.5 |##########################
  185434.7 |#############
  185597.9 |#############
  (1 below, 3 above range)

riv_snake (n=18, range 183208.4-186266.0 ns)
  183208.4 |
  183361.2 |####################
  183514.1 |####################
  183667.0 |########################################
  183819.9 |####################
  183972.8 |####################
  184125.6 |
  184278.5 |####################
  184431.4 |########################################
  184584.3 |
  184737.2 |
  184890.0 |
  185042.9 |
  185195.8 |####################
  185348.7 |########################################
  185501.6 |####################
  185654.4 |
  185807.3 |
  185960.2 |
  186113.1 |########################################
  (2 below, 1 above range)

riv_spec (n=18, range 180322.2-184759.1 ns)
  180322.2 |
  180544.1 |
  180765.9 |
  180987.8 |
  181209.6 |####################
  181431.4 |####################
  181653.3 |
  181875.1 |
  182097.0 |
  182318.8 |
  182540.6 |
  182762.5 |
  182984.3 |########################################
  183206.2 |########################################
  183428.0 |####################
  183649.8 |####################
  183871.7 |####################
  184093.5 |########################################
  184315.4 |####################
  184537.2 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=2311.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=2314.0% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=2313.1% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2312.0% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2322.1% of algo (FFI overhead may distort results)
