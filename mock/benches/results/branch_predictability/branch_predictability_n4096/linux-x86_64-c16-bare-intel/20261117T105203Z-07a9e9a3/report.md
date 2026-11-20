# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 4.57 us to slowest 4.57 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### bp_balanced is fastest but the noisiest (CV 17.0%)

bp_balanced wins on median (4.57 us) yet has the highest variance (CV 17.0%), while bp_skew_high is the steadiest (CV 7.8%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (775 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (4.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bp_balanced vs stability leader bp_skew_high (+0% speed for 2.2x steadier)

bp_balanced is fastest (4.57 us, CV 17.0%); bp_skew_high gives up 0.0% median for 2.2x lower variance (CV 7.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 4567.3 ns median
- Spread: 1.00x (fastest 4567.3 ns, slowest 4567.3 ns)

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
| Counter frequency | 2112755873 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 6240ns | 6456ns | 5280ns | 6216ns | 7273ns | base |
| bp_skew_high | 6173ns | 6456ns | 5284ns | 6365ns | 6488ns | -1.07% |
| bp_skew_low | 6171ns | 6456ns | 5282ns | 6362ns | 6488ns | -1.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 4414ns | 3738ns | 5137ns | base | 0.928 |
| bp_skew_high | 4367ns | 3740ns | 4581ns | -1.07% | 0.938 |
| bp_skew_low | 4365ns | 3738ns | 4582ns | -1.11% | 0.938 |

## Performance model

- Peak throughput: **1.096 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.897 | 81.8% |
| bp_skew_high | 0.897 | 81.8% |
| bp_skew_low | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 6240ns | 6240ns | base |
| bp_skew_high | 6173ns | 6173ns | -1.07% |
| bp_skew_low | 6171ns | 6171ns | -1.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 4567ns | base | --- | [4567, 4567] | --- | --- | --- | --- |
| bp_skew_high | 4567ns | no significant difference | [-0, +0]ns | [4567, 4567] | no | 0.4189 | 0.4189 | 7 |
| bp_skew_low | 4567ns | no significant difference | [-1, +0]ns | [4567, 4567] | no | 0.1304 | 0.0652 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 3737ns | +22.2% | -0.0% |
| 1 | 2 | 4573ns | -0.1% | -0.1% |
| 1 | 3 | 3748ns | +21.9% | +22.0% |
| 1 | 4 | 4568ns | -0.0% | -18.2% |
| 1 | 5 | 4573ns | -18.2% | +0.1% |
| 1 | 6 | 3737ns | +0.0% | +22.4% |
| 1 | 7 | 4578ns | -0.2% | -0.0% |
| 1 | 8 | 4578ns | -0.2% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.472 | moderate- |
| bp_skew_high | 0.292 | moderate+ |
| bp_skew_low | -0.207 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 4/8, lost 2/8
- **bp_skew_low**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 22.8ns | 4414.4ns | 0.5% |  |
| bp_skew_high | 22.6ns | 4367.0ns | 0.5% |  |
| bp_skew_low | 22.4ns | 4365.2ns | 0.5% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 3737.7-5136.9 ns)
   3737.7 |#################
   3807.7 |
   3877.6 |
   3947.6 |
   4017.6 |
   4087.5 |
   4157.5 |
   4227.4 |
   4297.4 |
   4367.4 |
   4437.3 |
   4507.3 |########################################
   4577.2 |########################################
   4647.2 |
   4717.2 |
   4787.1 |
   4857.1 |
   4927.0 |
   4997.0 |
   5067.0 |
  (29 below, 6 above range)

bp_skew_high (n=160, range 3740.2-4581.5 ns)
   3740.2 |#####
   3782.3 |
   3824.4 |
   3866.4 |
   3908.5 |
   3950.5 |
   3992.6 |
   4034.7 |
   4076.7 |
   4118.8 |
   4160.9 |
   4202.9 |
   4245.0 |
   4287.1 |
   4329.1 |
   4371.2 |
   4413.2 |
   4455.3 |
   4497.4 |
   4539.4 |########################################
  (21 below, 3 above range)

bp_skew_low (n=160, range 3737.6-4582.0 ns)
   3737.6 |###
   3779.8 |
   3822.1 |
   3864.3 |
   3906.5 |
   3948.7 |
   3990.9 |
   4033.2 |
   4075.4 |
   4117.6 |
   4159.8 |
   4202.1 |
   4244.3 |
   4286.5 |
   4328.7 |
   4370.9 |
   4413.2 |
   4455.4 |
   4497.6 |
   4539.8 |########################################
  (29 below, 6 above range)

```
