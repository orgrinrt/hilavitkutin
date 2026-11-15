# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (31.96 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 13.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 67% faster than the next best (ema_form_q32)

ema_form_intshift (13.70 us) leads ema_form_q32 (22.83 us) by 67%, a clear separation rather than a photo finish. CV 0.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 57% (significant)

ema_form_intshift is -18.26 us (57%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_float is an outlier: 2.3x slower than the field

ema_form_float (31.96 us) is 2.3x the fastest (13.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_form_intshift shows alternating (throttle bounce) (autocorr -0.60)

ema_form_intshift's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ema_form_intshift** at 13701.5 ns median (-57.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.33x (fastest 13701.5 ns, slowest 31960.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c97b3e25a1d8f04 |
| Counter frequency | 2112773217 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 33085ns | 33845ns | 29993ns | 33845ns | 33896ns | base |
| ema_form_intshift | 15594ns | 15588ns | 15577ns | 15588ns | 15629ns | -52.87% |
| ema_form_q32 | 24161ns | 24714ns | 21904ns | 24715ns | 24755ns | -26.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 31239ns | 28320ns | 31991ns | base | 0.524 |
| ema_form_intshift | 13705ns | 13692ns | 13724ns | -56.13% | 1.195 |
| ema_form_q32 | 22314ns | 20229ns | 22850ns | -28.57% | 0.734 |

## Performance model

- Peak throughput: **1.197 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.513 | 42.8% |
| ema_form_intshift | 1.196 | 99.9% |
| ema_form_q32 | 0.718 | 60.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 33085ns | 33085ns | base |
| ema_form_intshift | 15594ns | 15594ns | -52.87% |
| ema_form_q32 | 24161ns | 24161ns | -26.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 31960ns | base | --- | [31960, 31961] | --- | --- | --- | --- |
| ema_form_intshift | 13702ns | -18259.6ns (-57.1%) | [-18261, -18258]ns | [13700, 13707] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 22827ns | -9133.3ns (-28.6%) | [-9134, -9132]ns | [22827, 22828] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 31966ns | -57.2% | -28.6% |
| 1 | 2 | 31961ns | -57.1% | -28.6% |
| 1 | 3 | 31967ns | -57.1% | -28.6% |
| 1 | 4 | 31958ns | -57.1% | -41.6% |
| 1 | 5 | 31962ns | -57.1% | -28.6% |
| 1 | 6 | 26148ns | -47.6% | -12.7% |
| 1 | 7 | 31957ns | -57.1% | -28.6% |
| 1 | 8 | 31961ns | -57.1% | -28.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.160 | ok |
| ema_form_intshift | -0.601 | HIGH- (thermal bounce) |
| ema_form_q32 | -0.161 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 23.3ns | 31239.3ns | 0.1% |  |
| ema_form_intshift | 23.9ns | 13705.1ns | 0.2% |  |
| ema_form_q32 | 23.5ns | 22313.6ns | 0.1% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 28320.4-31990.5 ns)
  28320.4 |
  28503.9 |
  28687.4 |
  28870.9 |
  29054.5 |
  29238.0 |
  29421.5 |
  29605.0 |
  29788.5 |
  29972.0 |
  30155.5 |
  30339.0 |
  30522.5 |
  30706.0 |
  30889.5 |
  31073.0 |
  31256.5 |
  31440.0 |
  31623.5 |
  31807.0 |########################################
  (20 below, 5 above range)

ema_form_intshift (n=160, range 13692.4-13723.6 ns)
  13692.4 |
  13694.0 |#
  13695.5 |###
  13697.1 |########################################
  13698.6 |############
  13700.2 |#####
  13701.8 |##
  13703.3 |##
  13704.9 |##
  13706.4 |#######
  13708.0 |###########################
  13709.6 |##############
  13711.1 |#####
  13712.7 |#
  13714.2 |#
  13715.8 |#
  13717.4 |
  13718.9 |
  13720.5 |
  13722.0 |
  (14 below, 5 above range)

ema_form_q32 (n=160, range 20228.6-22850.4 ns)
  20228.6 |
  20359.7 |
  20490.8 |
  20621.9 |
  20753.0 |
  20884.1 |
  21015.1 |
  21146.2 |
  21277.3 |
  21408.4 |
  21539.5 |
  21670.6 |
  21801.7 |
  21932.8 |
  22063.9 |
  22195.0 |
  22326.1 |
  22457.2 |
  22588.2 |
  22719.3 |########################################
  (20 below, 1 above range)

```
