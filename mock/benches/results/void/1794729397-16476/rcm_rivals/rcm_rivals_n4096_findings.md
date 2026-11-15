# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 37.20 ms to slowest 37.34 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_rcm shows alternating (throttle bounce) (autocorr -0.71)

riv_rcm's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (139.84 us) is smaller than the fastest variant's own run-to-run std-dev (468.05 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_rowm** at 37201788.0 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 37201788.0 ns, slowest 37341625.0 ns)

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
| Counter frequency | 2112802731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 37395293ns | 37236667ns | 37013910ns | 37163641ns | 37933462ns | base |
| riv_rowm | 37308024ns | 37205809ns | 36913643ns | 37119755ns | 37787620ns | -0.23% |
| riv_scr | 37767979ns | 37345448ns | 37045439ns | 37294146ns | 38840000ns | +1.00% |
| riv_snake | 37538641ns | 37257133ns | 36936966ns | 37171389ns | 38390356ns | +0.38% |
| riv_spec | 37236050ns | 37229004ns | 37102236ns | 37197550ns | 37360705ns | -0.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 37391323ns | 37010030ns | 37929432ns | base | 0.000 |
| riv_rowm | 37304019ns | 36909734ns | 37783458ns | -0.23% | 0.000 |
| riv_scr | 37764035ns | 37041595ns | 38836023ns | +1.00% | 0.000 |
| riv_snake | 37534595ns | 36932997ns | 38386243ns | +0.38% | 0.000 |
| riv_spec | 37232130ns | 37098259ns | 37356704ns | -0.43% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 99.1% |
| riv_rowm | 0.000 | 99.2% |
| riv_scr | 0.000 | 98.8% |
| riv_snake | 0.000 | 99.1% |
| riv_spec | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 37395293ns | 37395293ns | base |
| riv_rowm | 37308024ns | 37308024ns | -0.23% |
| riv_scr | 37767979ns | 37767979ns | +1.00% |
| riv_snake | 37538641ns | 37538641ns | +0.38% |
| riv_spec | 37236050ns | 37236050ns | -0.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 37232577ns | base | --- | [37082762, 37267945] | --- | --- | --- | --- |
| riv_rowm | 37201788ns | no significant difference | [-141806, +167567]ns | [36964745, 37264009] | no | 0.9515 | 0.2379 | 0 |
| riv_scr | 37341625ns | no significant difference | [-152848, +483218]ns | [37193195, 37511931] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 37252974ns | no significant difference | [-99156, +145969]ns | [37021893, 37331540] | no | 1.0000 | 1.0000 | 0 |
| riv_spec | 37225215ns | no significant difference | [-144823, +93132]ns | [37136402, 37313621] | no | 0.9614 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 37005474ns | +0.5% | +1.7% | +0.8% | +0.7% |
| 1 | 2 | 37552453ns | +0.6% | -0.7% | -0.0% | -0.6% |
| 1 | 3 | 37058585ns | -0.4% | +1.1% | +0.5% | +0.2% |
| 2 | 1 | 37267277ns | -0.9% | +0.2% | -0.9% | -0.4% |
| 2 | 2 | 37118270ns | +0.2% | -0.1% | -0.3% | +0.2% |
| 2 | 3 | 37245303ns | +0.1% | -0.4% | +0.0% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.712 | HIGH- (thermal bounce) |
| riv_rowm | -0.176 | ok |
| riv_scr | 0.158 | ok |
| riv_snake | 0.324 | moderate+ |
| riv_spec | 0.094 | ok |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 3/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 2/6, lost 2/6
- **riv_spec**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 183892462.1ns | 37391323.2ns | 491.8% | HIGH |
| riv_rowm | 183868088.9ns | 37304018.7ns | 492.9% | HIGH |
| riv_scr | 183916099.6ns | 37764035.0ns | 487.0% | HIGH |
| riv_snake | 184068919.8ns | 37534594.5ns | 490.4% | HIGH |
| riv_spec | 183918725.6ns | 37232129.9ns | 494.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 37010029.7-37929432.3 ns)
  37010029.7 |################
  37055999.8 |########
  37101970.0 |########################
  37147940.1 |
  37193910.2 |########
  37239880.4 |########################################
  37285850.5 |
  37331820.6 |
  37377790.7 |
  37423760.9 |
  37469731.0 |
  37515701.1 |################
  37561671.3 |
  37607641.4 |########
  37653611.5 |
  37699581.6 |
  37745551.8 |
  37791521.9 |
  37837492.0 |
  37883462.2 |
  (2 below, 1 above range)

riv_rowm (n=18, range 36909733.8-37783457.5 ns)
  36909733.8 |####################
  36953420.0 |####################
  36997106.2 |
  37040792.4 |
  37084478.5 |
  37128164.7 |##########
  37171850.9 |########################################
  37215537.1 |####################
  37259223.3 |##########
  37302909.5 |
  37346595.7 |
  37390281.8 |
  37433968.0 |
  37477654.2 |
  37521340.4 |
  37565026.6 |
  37608712.8 |
  37652399.0 |
  37696085.1 |
  37739771.3 |####################
  (2 below, 2 above range)

riv_scr (n=18, range 37041595.4-38836022.6 ns)
  37041595.4 |########################################
  37131316.8 |
  37221038.1 |##############################
  37310759.5 |####################
  37400480.8 |##############################
  37490202.2 |##########
  37579923.5 |##########
  37669644.9 |
  37759366.3 |
  37849087.6 |
  37938809.0 |
  38028530.3 |##########
  38118251.7 |
  38207973.0 |
  38297694.4 |
  38387415.8 |
  38477137.1 |
  38566858.5 |
  38656579.8 |
  38746301.2 |
  (1 below, 2 above range)

riv_snake (n=18, range 36932997.4-38386243.5 ns)
  36932997.4 |##############################
  37005659.7 |##########
  37078322.0 |
  37150984.3 |##########
  37223646.6 |########################################
  37296308.9 |##############################
  37368971.2 |
  37441633.5 |
  37514295.8 |##############################
  37586958.1 |
  37659620.4 |
  37732282.7 |
  37804945.0 |
  37877607.3 |
  37950269.6 |
  38022931.9 |
  38095594.3 |
  38168256.6 |
  38240918.9 |
  38313581.2 |
  (2 below, 1 above range)

riv_spec (n=18, range 37098258.9-37356703.5 ns)
  37098258.9 |
  37111181.2 |########################################
  37124103.4 |########################################
  37137025.6 |####################
  37149947.8 |
  37162870.1 |####################
  37175792.3 |####################
  37188714.5 |
  37201636.8 |####################
  37214559.0 |
  37227481.2 |
  37240403.4 |####################
  37253325.7 |
  37266247.9 |####################
  37279170.1 |####################
  37292092.4 |####################
  37305014.6 |
  37317936.8 |####################
  37330859.0 |####################
  37343781.3 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=493.6% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=493.9% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=492.1% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=493.4% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=493.6% of algo (FFI overhead may distort results)
