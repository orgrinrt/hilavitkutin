# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (7.80 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 4.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 71% faster than the next best (sat_saturate)

sat_wrap (4.57 us) leads sat_saturate (7.80 us) by 71%, a clear separation rather than a photo finish. CV 0.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 41% (significant)

sat_wrap is -3.23 us (41%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: sat_wrap** at 4570.0 ns median (-41.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.71x (fastest 4570.0 ns, slowest 7803.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 2112797930 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 9257ns | 9688ns | 7868ns | 9544ns | 9782ns | base |
| sat_saturate | 9379ns | 9689ns | 7901ns | 9528ns | 10411ns | +1.32% |
| sat_wrap | 6465ns | 6462ns | 6449ns | 6461ns | 6492ns | -30.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 7453ns | 6323ns | 7881ns | base | 2.198 |
| sat_saturate | 7551ns | 6359ns | 8390ns | +1.31% | 2.170 |
| sat_wrap | 4575ns | 4569ns | 4583ns | -38.61% | 3.581 |

## Performance model

- Peak throughput: **3.586 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.100 | 58.6% |
| sat_saturate | 2.100 | 58.6% |
| sat_wrap | 3.585 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 9257ns | 9257ns | base |
| sat_saturate | 9379ns | 9379ns | +1.32% |
| sat_wrap | 6465ns | 6465ns | -30.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 7804ns | base | --- | [7792, 7819] | --- | --- | --- | --- |
| sat_saturate | 7802ns | no significant difference | [-25, +12]ns | [7785, 7814] | no | 0.6928 | 0.6928 | 0 |
| sat_wrap | 4570ns | -3230.8ns (-41.4%) | [-3240, -3220]ns | [4570, 4580] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 7840ns | -0.2% | -41.7% |
| 1 | 2 | 7831ns | -18.1% | -41.6% |
| 1 | 3 | 7837ns | -1.4% | -41.6% |
| 1 | 4 | 7817ns | +0.2% | -41.4% |
| 1 | 5 | 6389ns | -0.7% | -28.5% |
| 1 | 6 | 7792ns | +0.5% | -41.4% |
| 1 | 7 | 7821ns | +0.3% | -41.6% |
| 1 | 8 | 6291ns | +24.3% | -27.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.187 | ok |
| sat_saturate | -0.358 | moderate- |
| sat_wrap | 0.095 | ok |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 22.9ns | 7453.0ns | 0.3% |  |
| sat_saturate | 23.2ns | 7550.8ns | 0.3% |  |
| sat_wrap | 23.9ns | 4575.3ns | 0.5% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 6323.2-7881.2 ns)
   6323.2 |##########
   6401.1 |###
   6479.0 |
   6556.9 |
   6634.8 |
   6712.7 |
   6790.6 |
   6868.5 |
   6946.4 |
   7024.3 |
   7102.2 |
   7180.1 |
   7258.0 |
   7335.9 |
   7413.8 |
   7491.7 |
   7569.6 |
   7647.5 |#
   7725.4 |####################
   7803.3 |########################################
  (14 below, 5 above range)

sat_saturate (n=160, range 6359.0-8390.2 ns)
   6359.0 |##########
   6460.6 |
   6562.1 |
   6663.7 |
   6765.2 |
   6866.8 |
   6968.4 |
   7069.9 |
   7171.5 |
   7273.0 |
   7374.6 |
   7476.1 |#
   7577.7 |#
   7679.3 |#######
   7780.8 |########################################
   7882.4 |##
   7983.9 |
   8085.5 |
   8187.1 |
   8288.6 |
  (16 below, 2 above range)

sat_wrap (n=160, range 4569.1-4583.3 ns)
   4569.1 |########################################
   4569.8 |####
   4570.5 |
   4571.2 |
   4571.9 |
   4572.6 |
   4573.4 |
   4574.1 |
   4574.8 |
   4575.5 |
   4576.2 |
   4576.9 |
   4577.6 |
   4578.3 |
   4579.0 |
   4579.7 |#######
   4580.5 |################
   4581.2 |#############
   4581.9 |####
   4582.6 |
  (12 below, 6 above range)

```
