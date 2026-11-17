# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (3.76 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 3.62 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (riv_snake, riv_rowm) are a dead heat (<1%)

riv_snake (3.62 ms) and riv_rowm (3.65 ms) differ by 0.65%, inside the noise, even though the wider field spreads 3.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### riv_scr shows alternating (throttle bounce) (autocorr -0.85)

riv_scr's per-pass series has lag-1 autocorrelation -0.85, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.9% of the fastest

All 5 variants sit between 3.62 ms and 3.76 ms - a 3.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_snake** at 3623645.8 ns median (-3.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.04x (fastest 3623645.8 ns, slowest 3764606.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 3781625ns | 3770058ns | 3695850ns | 3747519ns | 3875672ns | base |
| riv_rowm | 3672426ns | 3652781ns | 3603558ns | 3642550ns | 3751674ns | -2.89% |
| riv_scr | 3784995ns | 3748119ns | 3697275ns | 3737138ns | 3900642ns | +0.09% |
| riv_snake | 3640462ns | 3629194ns | 3559544ns | 3606968ns | 3731160ns | -3.73% |
| riv_spec | 3709120ns | 3712967ns | 3635188ns | 3689491ns | 3775531ns | -1.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 3776168ns | 3690168ns | 3870208ns | base | 0.000 |
| riv_rowm | 3666777ns | 3598222ns | 3746133ns | -2.90% | 0.000 |
| riv_scr | 3779542ns | 3692188ns | 3894947ns | +0.09% | 0.000 |
| riv_snake | 3634939ns | 3554175ns | 3725399ns | -3.74% | 0.000 |
| riv_spec | 3703666ns | 3629701ns | 3770173ns | -1.92% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 94.4% |
| riv_rowm | 0.000 | 97.4% |
| riv_scr | 0.000 | 95.0% |
| riv_snake | 0.000 | 98.1% |
| riv_spec | 0.000 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 3781625ns | 3781625ns | base |
| riv_rowm | 3672426ns | 3672426ns | -2.89% |
| riv_scr | 3784995ns | 3784995ns | +0.09% |
| riv_snake | 3640462ns | 3640462ns | -3.73% |
| riv_spec | 3709120ns | 3709120ns | -1.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 3764606ns | base | --- | [3706256, 3800173] | --- | --- | --- | --- |
| riv_rowm | 3647317ns | -104943.8ns (-2.8%) | [-124527, -62538]ns | [3617283, 3703729] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| riv_scr | 3742206ns | no significant difference | [-32352, +44094]ns | [3707238, 3819575] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 3623646ns | -127789.6ns (-3.4%) | [-181815, -95904]ns | [3575338, 3660777] | YES | 0.0006 | 0.0001 | 0 |
| riv_spec | 3707742ns | -50039.6ns (-1.3%) | [-119744, -3483]ns | [3655792, 3736325] | YES (adj: no) | 0.1283 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 3707375ns | -2.3% | +0.4% | -2.0% | -0.1% |
| 1 | 2 | 3761346ns | -3.1% | -0.0% | -2.8% | -2.6% |
| 1 | 3 | 3767867ns | -3.1% | -1.6% | -2.7% | -3.5% |
| 2 | 1 | 3796308ns | -2.8% | +0.0% | -5.8% | -3.0% |
| 2 | 2 | 3693183ns | +0.6% | +0.3% | -1.4% | +0.8% |
| 2 | 3 | 3705138ns | -2.5% | +1.5% | -4.0% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.062 | ok |
| riv_rowm | -0.023 | ok |
| riv_scr | -0.848 | HIGH- (thermal bounce) |
| riv_snake | -0.241 | moderate- |
| riv_spec | 0.336 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 1/6, lost 3/6
- **riv_snake**: won 6/6, lost 0/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 67002742.4ns | 3776168.1ns | 1774.4% | HIGH |
| riv_rowm | 67001318.5ns | 3666776.6ns | 1827.3% | HIGH |
| riv_scr | 67045017.4ns | 3779541.9ns | 1773.9% | HIGH |
| riv_snake | 67029197.0ns | 3634938.9ns | 1844.0% | HIGH |
| riv_spec | 67019982.0ns | 3703666.0ns | 1809.6% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 3690168.1-3870207.6 ns)
  3690168.1 |########################################
  3699170.0 |##########################
  3708172.0 |#############
  3717174.0 |
  3726176.0 |
  3735178.0 |
  3744179.9 |#############
  3753181.9 |#############
  3762183.9 |#############
  3771185.9 |
  3780187.8 |#############
  3789189.8 |#############
  3798191.8 |########################################
  3807193.8 |
  3816195.8 |
  3825197.7 |
  3834199.7 |
  3843201.7 |#############
  3852203.7 |
  3861205.7 |
  (1 below, 2 above range)

riv_rowm (n=18, range 3598222.2-3746133.3 ns)
  3598222.2 |########################################
  3605617.8 |########################################
  3613013.3 |####################
  3620408.9 |####################
  3627804.4 |####################
  3635200.0 |
  3642595.6 |####################
  3649991.1 |####################
  3657386.7 |
  3664782.2 |########################################
  3672177.8 |
  3679573.3 |
  3686968.9 |####################
  3694364.4 |
  3701760.0 |
  3709155.5 |####################
  3716551.1 |####################
  3723946.7 |
  3731342.2 |####################
  3738737.8 |
  (1 below, 2 above range)

riv_scr (n=18, range 3692187.5-3894947.2 ns)
  3692187.5 |########################################
  3702325.5 |##########################
  3712463.5 |##########################
  3722601.5 |#############
  3732739.4 |
  3742877.4 |
  3753015.4 |##########################
  3763153.4 |
  3773291.4 |
  3783429.4 |
  3793567.4 |#############
  3803705.3 |#############
  3813843.3 |
  3823981.3 |#############
  3834119.3 |
  3844257.3 |#############
  3854395.3 |
  3864533.3 |#############
  3874671.2 |
  3884809.2 |
  (1 below, 2 above range)

riv_snake (n=18, range 3554175.0-3725399.3 ns)
  3554175.0 |########################################
  3562736.2 |####################
  3571297.4 |########################################
  3579858.6 |########################################
  3588419.9 |
  3596981.1 |
  3605542.3 |
  3614103.5 |####################
  3622664.7 |
  3631225.9 |########################################
  3639787.2 |####################
  3648348.4 |####################
  3656909.6 |
  3665470.8 |####################
  3674032.0 |####################
  3682593.2 |
  3691154.4 |
  3699715.7 |
  3708276.9 |
  3716838.1 |
  (1 below, 3 above range)

riv_spec (n=18, range 3629701.4-3770172.9 ns)
  3629701.4 |####################
  3636725.0 |####################
  3643748.6 |####################
  3650772.1 |
  3657795.7 |####################
  3664819.3 |####################
  3671842.9 |
  3678866.4 |####################
  3685890.0 |
  3692913.6 |
  3699937.2 |####################
  3706960.7 |########################################
  3713984.3 |
  3721007.9 |####################
  3728031.5 |
  3735055.0 |####################
  3742078.6 |########################################
  3749102.2 |
  3756125.8 |
  3763149.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1778.3% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1835.4% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1790.3% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1848.2% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1807.5% of algo (FFI overhead may distort results)
