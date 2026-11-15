# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (4.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_high at 3.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bp_skew_high dominates: 22% faster than the next best (bp_skew_low)

bp_skew_high (3.75 us) leads bp_skew_low (4.57 us) by 22%, a clear separation rather than a photo finish. CV 10.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bp_skew_high is fastest but the noisiest (CV 10.7%)

bp_skew_high wins on median (3.75 us) yet has the highest variance (CV 10.7%), while bp_skew_low is the steadiest (CV 6.1%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bp_skew_high's edge over baseline is significant but tiny (-11 ns, 0.24%)

bp_skew_high differs from baseline bp_balanced by -11 ns (0.24%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 3746.6 ns median (-18.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 3746.6 ns, slowest 4567.3 ns)

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
| Counter frequency | 2112760617 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 6175ns | 6449ns | 5274ns | 6354ns | 6537ns | base |
| bp_skew_high | 5724ns | 5284ns | 5270ns | 5627ns | 6468ns | -7.31% |
| bp_skew_low | 6315ns | 6455ns | 5718ns | 6454ns | 6493ns | +2.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 4371ns | 3739ns | 4611ns | base | 0.937 |
| bp_skew_high | 4054ns | 3737ns | 4577ns | -7.26% | 1.010 |
| bp_skew_low | 4470ns | 4050ns | 4587ns | +2.26% | 0.916 |

## Performance model

- Peak throughput: **1.096 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.897 | 81.8% |
| bp_skew_high | 1.093 | 99.7% |
| bp_skew_low | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 6175ns | 6175ns | base |
| bp_skew_high | 5724ns | 5724ns | -7.31% |
| bp_skew_low | 6315ns | 6315ns | +2.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 4567ns | base | --- | [4567, 4568] | --- | --- | --- | --- |
| bp_skew_high | 3747ns | -10.8ns (-0.2%) | [-33, -1]ns | [3746, 3748] | YES | 0.0000 | 0.0000 | 4 |
| bp_skew_low | 4567ns | no significant difference | [-0, +1]ns | [4567, 4567] | no | 0.4769 | 0.4769 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 4567ns | -18.2% | +0.2% |
| 1 | 2 | 3737ns | +0.0% | +22.2% |
| 1 | 3 | 4568ns | +0.2% | -18.2% |
| 1 | 4 | 4583ns | -0.3% | -0.3% |
| 1 | 5 | 4567ns | -18.2% | +0.2% |
| 1 | 6 | 3741ns | -0.1% | +22.1% |
| 1 | 7 | 4567ns | -18.0% | -0.0% |
| 1 | 8 | 4578ns | -0.2% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.362 | moderate- |
| bp_skew_high | 0.061 | ok |
| bp_skew_low | -0.152 | ok |

**Consistency summary:**

- **bp_skew_high**: won 6/8, lost 1/8
- **bp_skew_low**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 22.6ns | 4371.1ns | 0.5% |  |
| bp_skew_high | 20.8ns | 4053.6ns | 0.5% |  |
| bp_skew_low | 22.9ns | 4469.8ns | 0.5% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 3738.7-4610.9 ns)
   3738.7 |######
   3782.3 |
   3825.9 |
   3869.6 |
   3913.2 |
   3956.8 |
   4000.4 |
   4044.0 |
   4087.6 |
   4131.2 |
   4174.8 |
   4218.4 |
   4262.0 |
   4305.6 |
   4349.2 |
   4392.9 |
   4436.5 |
   4480.1 |
   4523.7 |###########
   4567.3 |########################################
  (25 below, 7 above range)

bp_skew_high (n=160, range 3736.7-4577.3 ns)
   3736.7 |########################################
   3778.8 |
   3820.8 |
   3862.8 |
   3904.8 |
   3946.9 |
   3988.9 |
   4030.9 |
   4073.0 |
   4115.0 |
   4157.0 |
   4199.0 |
   4241.1 |
   4283.1 |
   4325.1 |
   4367.2 |
   4409.2 |
   4451.2 |
   4493.2 |
   4535.3 |################
  (15 below, 26 above range)

bp_skew_low (n=160, range 4050.2-4587.4 ns)
   4050.2 |
   4077.1 |
   4103.9 |
   4130.8 |
   4157.6 |
   4184.5 |
   4211.4 |
   4238.2 |
   4265.1 |
   4291.9 |
   4318.8 |
   4345.7 |
   4372.5 |
   4399.4 |
   4426.2 |
   4453.1 |
   4480.0 |
   4506.8 |
   4533.7 |
   4560.5 |########################################
  (20 below, 4 above range)

```
