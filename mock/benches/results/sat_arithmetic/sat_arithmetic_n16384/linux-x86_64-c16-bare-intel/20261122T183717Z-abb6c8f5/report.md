# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (7.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 4.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 70% faster than the next best (sat_saturate)

sat_wrap (4.57 us) leads sat_saturate (7.76 us) by 70%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -3.24 us (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: sat_wrap** at 4569.5 ns median (-41.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.70x (fastest 4569.5 ns, slowest 7770.5 ns)

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
| Counter frequency | 2112769626 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 9091ns | 9666ns | 7902ns | 9171ns | 10040ns | base |
| sat_saturate | 9042ns | 9655ns | 7912ns | 9182ns | 9755ns | -0.54% |
| sat_wrap | 6031ns | 6456ns | 5283ns | 6123ns | 6503ns | -33.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 7315ns | 6355ns | 8084ns | base | 2.240 |
| sat_saturate | 7276ns | 6366ns | 7852ns | -0.53% | 2.252 |
| sat_wrap | 4265ns | 3739ns | 4594ns | -41.69% | 3.841 |

## Performance model

- Peak throughput: **4.382 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.109 | 48.1% |
| sat_saturate | 2.110 | 48.2% |
| sat_wrap | 3.586 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 9091ns | 9091ns | base |
| sat_saturate | 9042ns | 9042ns | -0.54% |
| sat_wrap | 6031ns | 6031ns | -33.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 7770ns | base | --- | [7675, 7800] | --- | --- | --- | --- |
| sat_saturate | 7764ns | no significant difference | [-5, +18]ns | [7676, 7780] | no | 0.3041 | 0.3041 | 0 |
| sat_wrap | 4570ns | -3238.9ns (-41.7%) | [-3281, -3204]ns | [4569, 4570] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 6386ns | +21.3% | -28.4% |
| 1 | 2 | 6407ns | -0.1% | -28.5% |
| 1 | 3 | 7790ns | +0.4% | -52.0% |
| 1 | 4 | 7805ns | +0.2% | -41.4% |
| 1 | 5 | 7831ns | -0.7% | -41.6% |
| 1 | 6 | 6392ns | +0.4% | -28.5% |
| 1 | 7 | 7823ns | -18.2% | -52.2% |
| 1 | 8 | 7818ns | -0.1% | -52.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | 0.056 | ok |
| sat_saturate | -0.129 | ok |
| sat_wrap | 0.056 | ok |

**Consistency summary:**

- **sat_saturate**: won 3/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 22.7ns | 7315.1ns | 0.3% |  |
| sat_saturate | 22.6ns | 7276.1ns | 0.3% |  |
| sat_wrap | 22.3ns | 4265.3ns | 0.5% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 6355.1-8083.8 ns)
   6355.1 |########################################
   6441.6 |
   6528.0 |
   6614.4 |
   6700.9 |
   6787.3 |
   6873.7 |
   6960.2 |
   7046.6 |
   7133.0 |
   7219.5 |
   7305.9 |
   7392.4 |
   7478.8 |
   7565.2 |####
   7651.7 |#######
   7738.1 |######################################
   7824.5 |###########################
   7911.0 |
   7997.4 |
  (11 below, 3 above range)

sat_saturate (n=160, range 6365.6-7851.8 ns)
   6365.6 |######################
   6439.9 |#####
   6514.2 |#
   6588.5 |
   6662.8 |
   6737.1 |
   6811.4 |
   6885.7 |
   6960.0 |
   7034.3 |
   7108.7 |
   7183.0 |
   7257.3 |
   7331.6 |
   7405.9 |
   7480.2 |
   7554.5 |###
   7628.8 |###
   7703.1 |##############
   7777.4 |########################################
  (16 below, 12 above range)

sat_wrap (n=160, range 3738.7-4594.2 ns)
   3738.7 |##################
   3781.5 |
   3824.3 |
   3867.1 |
   3909.8 |
   3952.6 |
   3995.4 |
   4038.2 |
   4080.9 |
   4123.7 |
   4166.5 |
   4209.3 |
   4252.0 |
   4294.8 |
   4337.6 |
   4380.4 |
   4423.1 |
   4465.9 |
   4508.7 |
   4551.5 |########################################
  (14 below, 3 above range)

```
