# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (37.62 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_spec at 37.41 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.6% (fastest 37.41 ms to slowest 37.62 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_rowm shows alternating (throttle bounce) (autocorr -0.57)

riv_rowm's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (212.47 us) is smaller than the fastest variant's own run-to-run std-dev (380.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_spec** at 37409396.5 ns median (-0.6% vs baseline)
- Spread: 1.01x (fastest 37409396.5 ns, slowest 37621863.9 ns)

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
| Counter frequency | 2112768073 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 38193476ns | 37627321ns | 37248973ns | 37575915ns | 39592070ns | base |
| riv_rowm | 37961856ns | 37554611ns | 37199362ns | 37470085ns | 39080761ns | -0.61% |
| riv_scr | 37676667ns | 37531858ns | 37118965ns | 37455865ns | 38286722ns | -1.35% |
| riv_snake | 37644626ns | 37487467ns | 37325061ns | 37504365ns | 38014801ns | -1.44% |
| riv_spec | 37489994ns | 37413298ns | 37196976ns | 37358975ns | 37833032ns | -1.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 38189220ns | 37244751ns | 39588021ns | base | 0.000 |
| riv_rowm | 37957741ns | 37195208ns | 39076710ns | -0.61% | 0.000 |
| riv_scr | 37672572ns | 37115050ns | 38282404ns | -1.35% | 0.000 |
| riv_snake | 37640588ns | 37321086ns | 38010642ns | -1.44% | 0.000 |
| riv_spec | 37485869ns | 37193016ns | 37828731ns | -1.84% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_scr; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.7% |
| riv_rowm | 0.000 | 98.8% |
| riv_scr | 0.000 | 98.9% |
| riv_snake | 0.000 | 99.0% |
| riv_spec | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 38193476ns | 38193476ns | base |
| riv_rowm | 37961856ns | 37961856ns | -0.61% |
| riv_scr | 37676667ns | 37676667ns | -1.35% |
| riv_snake | 37644626ns | 37644626ns | -1.44% |
| riv_spec | 37489994ns | 37489994ns | -1.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 37621864ns | base | --- | [37487920, 37781821] | --- | --- | --- | --- |
| riv_rowm | 37550410ns | no significant difference | [-410653, +120450]ns | [37316416, 37729280] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 37527717ns | no significant difference | [-451482, +113816]ns | [37360675, 37678522] | no | 0.6409 | 0.4807 | 0 |
| riv_snake | 37483388ns | no significant difference | [-379299, +146673]ns | [37396819, 37847528] | no | 0.6409 | 0.4807 | 0 |
| riv_spec | 37409396ns | no significant difference | [-356547, +12688]ns | [37270262, 37499837] | no | 0.6409 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 37683800ns | +0.2% | -0.9% | -0.6% | -1.1% |
| 1 | 2 | 37208976ns | +0.3% | +0.8% | +0.6% | +0.8% |
| 1 | 3 | 38100333ns | -2.2% | -1.0% | -1.2% | -1.8% |
| 2 | 1 | 38503516ns | -2.0% | -1.4% | -1.7% | -2.5% |
| 2 | 2 | 37596960ns | -1.1% | -1.3% | -0.6% | -0.9% |
| 2 | 3 | 37481262ns | +0.4% | +0.2% | -0.3% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.028 | ok |
| riv_rowm | -0.567 | HIGH- (thermal bounce) |
| riv_scr | -0.280 | moderate- |
| riv_snake | 0.031 | ok |
| riv_spec | 0.027 | ok |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 5/6, lost 1/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 185878018.2ns | 38189219.7ns | 486.7% | HIGH |
| riv_rowm | 185363247.0ns | 37957741.2ns | 488.3% | HIGH |
| riv_scr | 185073712.3ns | 37672571.7ns | 491.3% | HIGH |
| riv_snake | 184661493.8ns | 37640588.0ns | 490.6% | HIGH |
| riv_spec | 184753648.5ns | 37485869.3ns | 492.9% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 37244750.8-39588021.1 ns)
  37244750.8 |##########
  37361914.3 |##########
  37479077.8 |########################################
  37596241.3 |##############################
  37713404.8 |##############################
  37830568.3 |
  37947731.9 |
  38064895.4 |##########
  38182058.9 |
  38299222.4 |
  38416385.9 |##########
  38533549.4 |
  38650713.0 |
  38767876.5 |
  38885040.0 |
  39002203.5 |
  39119367.0 |
  39236530.5 |
  39353694.1 |
  39470857.6 |
  (2 below, 2 above range)

riv_rowm (n=18, range 37195207.6-39076709.8 ns)
  37195207.6 |####################
  37289282.7 |####################
  37383357.8 |#############
  37477432.9 |
  37571508.0 |######
  37665583.1 |########################################
  37759658.2 |
  37853733.4 |
  37947808.5 |
  38041883.6 |
  38135958.7 |
  38230033.8 |
  38324108.9 |
  38418184.0 |
  38512259.1 |
  38606334.2 |
  38700409.3 |
  38794484.4 |
  38888559.6 |
  38982634.7 |
  (1 below, 2 above range)

riv_scr (n=18, range 37115049.6-38282403.6 ns)
  37115049.6 |#############
  37173417.3 |
  37231785.0 |#############
  37290152.7 |
  37348520.4 |########################################
  37406888.1 |#############
  37465255.8 |#############
  37523623.5 |##########################
  37581991.2 |#############
  37640358.9 |#############
  37698726.6 |#############
  37757094.3 |
  37815462.0 |
  37873829.7 |
  37932197.4 |##########################
  37990565.1 |#############
  38048932.8 |
  38107300.5 |
  38165668.2 |
  38224035.9 |
  (2 below, 1 above range)

riv_snake (n=18, range 37321085.9-38010642.1 ns)
  37321085.9 |########################################
  37355563.7 |########################################
  37390041.5 |####################
  37424519.3 |########################################
  37458997.1 |########################################
  37493474.9 |
  37527952.7 |
  37562430.6 |
  37596908.4 |
  37631386.2 |####################
  37665864.0 |
  37700341.8 |
  37734819.6 |
  37769297.4 |
  37803775.3 |########################################
  37838253.1 |########################################
  37872730.9 |
  37907208.7 |
  37941686.5 |
  37976164.3 |
  (1 below, 3 above range)

riv_spec (n=18, range 37193015.9-37828731.3 ns)
  37193015.9 |####################
  37224801.7 |##########
  37256587.4 |####################
  37288373.2 |##########
  37320159.0 |
  37351944.7 |##########
  37383730.5 |##########
  37415516.3 |####################
  37447302.0 |
  37479087.8 |########################################
  37510873.6 |##########
  37542659.4 |
  37574445.1 |
  37606230.9 |
  37638016.7 |
  37669802.4 |
  37701588.2 |
  37733374.0 |
  37765159.7 |
  37796945.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=492.4% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=492.1% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=491.1% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=491.7% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=492.7% of algo (FFI overhead may distort results)
