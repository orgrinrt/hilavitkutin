# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_form_float) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_form_float has the worst median (9.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_form_intshift at 5.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_form_intshift dominates: 48% faster than the next best (ema_form_q32)

ema_form_intshift (5.15 us) leads ema_form_q32 (7.63 us) by 48%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 43% (significant)

ema_form_intshift is -3.90 us (43%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_intshift shows alternating (throttle bounce) (autocorr -0.60)

ema_form_intshift's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ema_form_intshift** at 5150.8 ns median (-43.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.77x (fastest 5150.8 ns, slowest 9104.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 11373ns | 11342ns | 10985ns | 11317ns | 11929ns | base |
| ema_form_intshift | 7319ns | 7294ns | 7155ns | 7283ns | 7590ns | -35.65% |
| ema_form_q32 | 9849ns | 9735ns | 9706ns | 9776ns | 10212ns | -13.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 9123ns | 8808ns | 9569ns | base | 0.449 |
| ema_form_intshift | 5169ns | 5055ns | 5367ns | -43.34% | 0.792 |
| ema_form_q32 | 7715ns | 7604ns | 8001ns | -15.44% | 0.531 |

## Performance model

- Peak throughput: **0.810 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.450 | 55.5% |
| ema_form_intshift | 0.795 | 98.1% |
| ema_form_q32 | 0.537 | 66.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 11373ns | 11373ns | base |
| ema_form_intshift | 7319ns | 7319ns | -35.65% |
| ema_form_q32 | 9849ns | 9849ns | -13.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 9105ns | base | --- | [8983, 9109] | --- | --- | --- | --- |
| ema_form_intshift | 5151ns | -3902.5ns (-42.9%) | [-3999, -3871]ns | [5105, 5155] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 7627ns | -1375.7ns (-15.1%) | [-1449, -1345]ns | [7614, 7635] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 8833ns | -42.7% | -13.9% |
| 1 | 2 | 9432ns | -43.9% | -19.3% |
| 1 | 3 | 9105ns | -43.4% | -16.2% |
| 1 | 4 | 9205ns | -44.0% | -17.3% |
| 1 | 5 | 8864ns | -42.8% | -11.0% |
| 1 | 6 | 9060ns | -42.4% | -16.0% |
| 1 | 7 | 9162ns | -44.8% | -15.4% |
| 1 | 8 | 9110ns | -42.6% | -13.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.405 | moderate- |
| ema_form_intshift | -0.600 | HIGH- (thermal bounce) |
| ema_form_q32 | -0.090 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 2.9ns | 9122.8ns | 0.0% |  |
| ema_form_intshift | 2.9ns | 5168.9ns | 0.1% |  |
| ema_form_q32 | 2.7ns | 7714.6ns | 0.0% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 8808.2-9568.7 ns)
   8808.2 |##############
   8846.2 |#############
   8884.3 |##########
   8922.3 |####
   8960.3 |##########################
   8998.3 |#####
   9036.4 |##
   9074.4 |################################
   9112.4 |##########
   9150.4 |########
   9188.4 |####
   9226.5 |####
   9264.5 |####
   9302.5 |#
   9340.5 |#
   9378.6 |#
   9416.6 |########################################
   9454.6 |##
   9492.6 |##
   9530.7 |
  (22 below, 7 above range)

ema_form_intshift (n=160, range 5054.8-5366.6 ns)
   5054.8 |########################################
   5070.4 |###
   5086.0 |######
   5101.6 |###
   5117.2 |
   5132.8 |####
   5148.4 |####################
   5163.9 |##
   5179.5 |##
   5195.1 |
   5210.7 |##
   5226.3 |#################
   5241.9 |###
   5257.5 |
   5273.0 |###
   5288.6 |
   5304.2 |#
   5319.8 |#
   5335.4 |
   5351.0 |
  (13 below, 19 above range)

ema_form_q32 (n=160, range 7603.9-8001.3 ns)
   7603.9 |########################################
   7623.8 |################
   7643.7 |#####
   7663.5 |
   7683.4 |#
   7703.3 |###
   7723.1 |##
   7743.0 |##
   7762.9 |
   7782.8 |
   7802.6 |##
   7822.5 |
   7842.4 |
   7862.2 |####
   7882.1 |#####
   7902.0 |##
   7921.9 |
   7941.7 |
   7961.6 |#
   7981.5 |
  (16 below, 12 above range)

```
