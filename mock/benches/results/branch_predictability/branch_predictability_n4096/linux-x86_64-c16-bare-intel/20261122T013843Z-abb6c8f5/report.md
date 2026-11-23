# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (4.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 3.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bp_skew_high dominates: 22% faster than the next best (bp_skew_low)

bp_skew_high (3.75 us) leads bp_skew_low (4.57 us) by 22%, a clear separation rather than a photo finish. CV 18.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bp_skew_high is fastest but the noisiest (CV 18.5%)

bp_skew_high wins on median (3.75 us) yet has the highest variance (CV 18.5%), while bp_balanced is the steadiest (CV 6.1%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bp_skew_high's edge over baseline is significant but tiny (-12 ns, 0.27%)

bp_skew_high differs from baseline bp_balanced by -12 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 3746.6 ns median (-18.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.22x (fastest 3746.6 ns, slowest 4567.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 2112577410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 6313ns | 6453ns | 5717ns | 6452ns | 6493ns | base |
| bp_skew_high | 5871ns | 5288ns | 5271ns | 5699ns | 6986ns | -7.01% |
| bp_skew_low | 6021ns | 6449ns | 5276ns | 6116ns | 6477ns | -4.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 4470ns | 4050ns | 4588ns | base | 0.916 |
| bp_skew_high | 4155ns | 3737ns | 4940ns | -7.04% | 0.986 |
| bp_skew_low | 4260ns | 3737ns | 4579ns | -4.70% | 0.962 |

## Performance model

- Peak throughput: **1.096 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.897 | 81.8% |
| bp_skew_high | 1.093 | 99.7% |
| bp_skew_low | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 6313ns | 6313ns | base |
| bp_skew_high | 5871ns | 5871ns | -7.01% |
| bp_skew_low | 6021ns | 6021ns | -4.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 4567ns | base | --- | [4567, 4568] | --- | --- | --- | --- |
| bp_skew_high | 3747ns | -12.3ns (-0.3%) | [-820, -9]ns | [3746, 3792] | YES | 0.0000 | 0.0000 | 1 |
| bp_skew_low | 4567ns | -0.7ns (-0.0%) | [-9, -0]ns | [4567, 4567] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 3737ns | +0.0% | +0.0% |
| 1 | 2 | 4573ns | -18.3% | -18.3% |
| 1 | 3 | 4568ns | -0.0% | -0.0% |
| 1 | 4 | 4572ns | -18.3% | -18.3% |
| 1 | 5 | 4573ns | -0.1% | -0.1% |
| 1 | 6 | 4568ns | -0.0% | +0.1% |
| 1 | 7 | 4567ns | -18.0% | +0.0% |
| 1 | 8 | 4567ns | -18.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.022 | ok |
| bp_skew_high | -0.141 | ok |
| bp_skew_low | 0.061 | ok |

**Consistency summary:**

- **bp_skew_high**: won 5/8, lost 0/8
- **bp_skew_low**: won 3/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 23.1ns | 4470.0ns | 0.5% |  |
| bp_skew_high | 21.5ns | 4155.1ns | 0.5% |  |
| bp_skew_low | 21.8ns | 4259.8ns | 0.5% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 4050.4-4587.6 ns)
   4050.4 |
   4077.3 |
   4104.2 |
   4131.0 |
   4157.9 |
   4184.7 |
   4211.6 |
   4238.4 |
   4265.3 |
   4292.1 |
   4319.0 |
   4345.9 |
   4372.7 |
   4399.6 |
   4426.4 |
   4453.3 |
   4480.1 |
   4507.0 |
   4533.9 |
   4560.7 |########################################
  (20 below, 1 above range)

bp_skew_high (n=160, range 3736.8-4940.5 ns)
   3736.8 |########################################
   3797.0 |
   3857.2 |
   3917.3 |
   3977.5 |
   4037.7 |
   4097.9 |
   4158.1 |
   4218.3 |
   4278.5 |
   4338.6 |
   4398.8 |
   4459.0 |
   4519.2 |#######################
   4579.4 |####
   4639.6 |
   4699.7 |
   4759.9 |
   4820.1 |
   4880.3 |
  (6 below, 4 above range)

bp_skew_low (n=160, range 3736.6-4579.1 ns)
   3736.6 |####################
   3778.8 |
   3820.9 |
   3863.0 |
   3905.1 |
   3947.3 |
   3989.4 |
   4031.5 |
   4073.6 |
   4115.7 |
   4157.9 |
   4200.0 |
   4242.1 |
   4284.2 |
   4326.4 |
   4368.5 |
   4410.6 |
   4452.7 |
   4494.8 |
   4537.0 |########################################
  (16 below, 12 above range)

```
