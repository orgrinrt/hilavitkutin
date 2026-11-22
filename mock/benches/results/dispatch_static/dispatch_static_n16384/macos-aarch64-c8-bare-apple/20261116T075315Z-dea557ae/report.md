# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (2.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_direct at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.51 us to slowest 2.51 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### dispatch_trait shows alternating (throttle bounce) (autocorr -0.56)

dispatch_trait's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (27 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: dispatch_direct** at 2509.6 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 2509.6 ns, slowest 2510.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 4629ns | 4601ns | 4588ns | 4616ns | 4708ns | base |
| dispatch_direct | 4613ns | 4596ns | 4589ns | 4600ns | 4676ns | -0.35% |
| dispatch_trait | 4611ns | 4604ns | 4587ns | 4604ns | 4656ns | -0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2524ns | 2505ns | 2567ns | base | 6.490 |
| dispatch_direct | 2520ns | 2505ns | 2563ns | -0.19% | 6.503 |
| dispatch_trait | 2515ns | 2504ns | 2538ns | -0.38% | 6.515 |

## Performance model

- Peak throughput: **6.542 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 6.525 | 99.7% |
| dispatch_direct | 6.529 | 99.8% |
| dispatch_trait | 6.527 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 4629ns | 4629ns | base |
| dispatch_direct | 4613ns | 4613ns | -0.35% |
| dispatch_trait | 4611ns | 4611ns | -0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2511ns | base | --- | [2510, 2512] | --- | --- | --- | --- |
| dispatch_direct | 2510ns | no significant difference | [-2, +0]ns | [2509, 2510] | no | 0.0845 | 0.0422 | 8 |
| dispatch_trait | 2510ns | no significant difference | [-2, +1]ns | [2509, 2511] | no | 0.4713 | 0.4713 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2556ns | -1.8% | -1.9% |
| 1 | 2 | 2513ns | -0.1% | -0.1% |
| 1 | 3 | 2511ns | -0.1% | -0.2% |
| 1 | 4 | 2509ns | -0.0% | +0.1% |
| 1 | 5 | 2509ns | +0.1% | +0.0% |
| 1 | 6 | 2509ns | +0.0% | +0.1% |
| 1 | 7 | 2509ns | +0.0% | +0.1% |
| 1 | 8 | 2511ns | -0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.049 | ok |
| dispatch_direct | -0.365 | moderate- |
| dispatch_trait | -0.559 | HIGH- (thermal bounce) |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 1/8
- **dispatch_trait**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.9ns | 2524.3ns | 0.1% |  |
| dispatch_direct | 2.9ns | 2519.5ns | 0.1% |  |
| dispatch_trait | 2.9ns | 2514.8ns | 0.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 2505.1-2567.5 ns)
   2505.1 |########################################
   2508.2 |########################################
   2511.4 |#################
   2514.5 |####
   2517.6 |##
   2520.7 |#
   2523.8 |#
   2526.9 |###
   2530.1 |
   2533.2 |#
   2536.3 |####
   2539.4 |#
   2542.5 |#
   2545.7 |#
   2548.8 |######
   2551.9 |######
   2555.0 |###########
   2558.1 |######
   2561.3 |##
   2564.4 |
  (10 below, 7 above range)

dispatch_direct (n=160, range 2504.5-2562.9 ns)
   2504.5 |###################################
   2507.4 |########################################
   2510.4 |###########################
   2513.3 |########
   2516.2 |######
   2519.1 |##
   2522.0 |#
   2524.9 |#
   2527.9 |
   2530.8 |##
   2533.7 |#
   2536.6 |###
   2539.5 |#
   2542.4 |
   2545.4 |
   2548.3 |##
   2551.2 |
   2554.1 |####
   2557.0 |###
   2560.0 |#
  (12 below, 11 above range)

dispatch_trait (n=160, range 2504.4-2538.1 ns)
   2504.4 |###########################
   2506.1 |######################################
   2507.8 |#################################
   2509.5 |########################################
   2511.2 |###########################
   2512.8 |###############
   2514.5 |#############
   2516.2 |#####
   2517.9 |###
   2519.6 |#####
   2521.2 |
   2522.9 |#
   2524.6 |###
   2526.3 |#
   2528.0 |#####
   2529.6 |######
   2531.3 |
   2533.0 |###
   2534.7 |#
   2536.4 |#
  (12 below, 12 above range)

```
