# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_scr shows alternating (throttle bounce) (autocorr -0.53)

riv_scr's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader riv_snake vs stability leader riv_rcm (+4% speed for 1.7x steadier)

riv_snake is fastest (3.66 ms, CV 3.5%); riv_rcm gives up 4.0% median for 1.7x lower variance (CV 2.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.5% of the fastest

All 5 variants sit between 3.66 ms and 3.83 ms - a 4.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_snake** at 3662541.6 ns median (-3.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.04x (fastest 3662541.6 ns, slowest 3825856.2 ns)

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
| riv_rcm | 3838295ns | 3815167ns | 3743585ns | 3806099ns | 3933944ns | base |
| riv_rowm | 4059598ns | 3769023ns | 3658561ns | 3737009ns | 4744001ns | +5.77% |
| riv_scr | 3841428ns | 3832004ns | 3750249ns | 3804646ns | 3942192ns | +0.08% |
| riv_snake | 3713422ns | 3668117ns | 3591957ns | 3653755ns | 3863655ns | -3.25% |
| riv_spec | 3821111ns | 3737004ns | 3662494ns | 3736678ns | 4027069ns | -0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 3832319ns | 3736881ns | 3927942ns | base | 0.000 |
| riv_rowm | 4053770ns | 3652601ns | 4738240ns | +5.78% | 0.000 |
| riv_scr | 3835478ns | 3744401ns | 3936156ns | +0.08% | 0.000 |
| riv_snake | 3707634ns | 3586287ns | 3857860ns | -3.25% | 0.000 |
| riv_spec | 3815100ns | 3656488ns | 4021009ns | -0.45% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 94.1% |
| riv_rowm | 0.000 | 95.3% |
| riv_scr | 0.000 | 93.7% |
| riv_snake | 0.000 | 97.9% |
| riv_spec | 0.000 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 3838295ns | 3838295ns | base |
| riv_rowm | 4059598ns | 4059598ns | +5.77% |
| riv_scr | 3841428ns | 3841428ns | +0.08% |
| riv_snake | 3713422ns | 3713422ns | -3.25% |
| riv_spec | 3821111ns | 3821111ns | -0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 3809798ns | base | --- | [3768244, 3882310] | --- | --- | --- | --- |
| riv_rowm | 3763138ns | no significant difference | [-129477, +52092]ns | [3688490, 3846229] | no | 0.2379 | 0.2379 | 0 |
| riv_scr | 3825856ns | no significant difference | [-65950, +36848]ns | [3760233, 3879844] | no | 0.1925 | 0.0963 | 0 |
| riv_snake | 3662542ns | -134864.6ns (-3.5%) | [-176275, -78496]ns | [3613806, 3754440] | YES | 0.0302 | 0.0075 | 0 |
| riv_spec | 3731338ns | no significant difference | [-110438, +49858]ns | [3692571, 3884194] | no | 0.2379 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 3872633ns | -2.0% | -2.9% | -5.1% | -4.9% |
| 1 | 2 | 3862246ns | +0.6% | -1.1% | -7.1% | -2.7% |
| 1 | 3 | 3822233ns | -0.9% | +1.0% | -4.5% | -2.1% |
| 2 | 1 | 3763625ns | -2.3% | +0.6% | -3.3% | -1.1% |
| 2 | 2 | 3904233ns | -2.5% | -0.0% | -4.6% | +6.1% |
| 2 | 3 | 3793354ns | -1.5% | -1.2% | -2.5% | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.449 | moderate- |
| riv_rowm | -0.083 | ok |
| riv_scr | -0.526 | HIGH- (thermal bounce) |
| riv_snake | 0.082 | ok |
| riv_spec | -0.327 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 3/6, lost 2/6
- **riv_snake**: won 6/6, lost 0/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 67064475.0ns | 3832319.4ns | 1750.0% | HIGH |
| riv_rowm | 67149210.9ns | 4053769.9ns | 1656.5% | HIGH |
| riv_scr | 67059585.2ns | 3835478.5ns | 1748.4% | HIGH |
| riv_snake | 67123151.8ns | 3707634.5ns | 1810.4% | HIGH |
| riv_spec | 67377169.4ns | 3815100.5ns | 1766.1% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 3736880.6-3927942.4 ns)
  3736880.6 |#############
  3746433.7 |
  3755986.7 |########################################
  3765539.8 |#############
  3775092.9 |
  3784646.0 |#############
  3794199.1 |
  3803752.2 |########################################
  3813305.3 |#############
  3822858.4 |
  3832411.5 |
  3841964.5 |
  3851517.6 |
  3861070.7 |#############
  3870623.8 |#############
  3880176.9 |
  3889730.0 |#############
  3899283.1 |#############
  3908836.2 |
  3918389.3 |
  (1 below, 3 above range)

riv_rowm (n=18, range 3652601.4-4738239.6 ns)
  3652601.4 |########################################
  3706883.3 |########################
  3761165.2 |################################
  3815447.1 |
  3869729.0 |################
  3924010.9 |################
  3978292.8 |
  4032574.8 |
  4086856.7 |
  4141138.6 |
  4195420.5 |
  4249702.4 |
  4303984.3 |
  4358266.2 |
  4412548.1 |
  4466830.0 |
  4521111.9 |
  4575393.8 |
  4629675.8 |
  4683957.7 |
  (1 below, 1 above range)

riv_scr (n=18, range 3744401.4-3936155.6 ns)
  3744401.4 |########################################
  3753989.1 |##########################
  3763576.8 |
  3773164.5 |
  3782752.2 |#############
  3792339.9 |
  3801927.6 |#############
  3811515.3 |
  3821103.0 |##########################
  3830690.8 |##########################
  3840278.5 |
  3849866.2 |#############
  3859453.9 |
  3869041.6 |
  3878629.3 |
  3888217.0 |
  3897804.7 |#############
  3907392.4 |
  3916980.1 |#############
  3926567.9 |
  (1 below, 3 above range)

riv_snake (n=18, range 3586287.5-3857860.4 ns)
  3586287.5 |########################################
  3599866.1 |#############
  3613444.8 |#############
  3627023.4 |#############
  3640602.1 |##########################
  3654180.7 |
  3667759.4 |#############
  3681338.0 |
  3694916.6 |##########################
  3708495.3 |
  3722073.9 |#############
  3735652.6 |
  3749231.2 |
  3762809.9 |
  3776388.5 |#############
  3789967.2 |#############
  3803545.8 |
  3817124.5 |
  3830703.1 |
  3844281.8 |#############
  (1 below, 2 above range)

riv_spec (n=18, range 3656487.5-4021009.0 ns)
  3656487.5 |##########################
  3674713.6 |##########################
  3692939.7 |########################################
  3711165.7 |#############
  3729391.8 |#############
  3747617.9 |#############
  3765844.0 |
  3784070.0 |
  3802296.1 |
  3820522.2 |
  3838748.3 |
  3856974.3 |
  3875200.4 |########################################
  3893426.5 |
  3911652.6 |#############
  3929878.6 |
  3948104.7 |#############
  3966330.8 |
  3984556.9 |
  4002782.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1759.3% of algo (FFI overhead may distort results)
- **riv_rowm**: CV=29.4% (high variance, measurements may be unstable)
- **riv_rowm**: bridge=1783.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1751.5% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1830.5% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1797.8% of algo (FFI overhead may distort results)
