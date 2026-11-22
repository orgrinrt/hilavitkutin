# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (3.84 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 3.72 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 3.3% of the fastest

All 5 variants sit between 3.72 ms and 3.84 ms - a 3.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_snake** at 3720110.4 ns median (-3.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.03x (fastest 3720110.4 ns, slowest 3843410.4 ns)

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
| riv_rcm | 3871536ns | 3847998ns | 3771986ns | 3826501ns | 3988864ns | base |
| riv_rowm | 3786069ns | 3769181ns | 3680074ns | 3752007ns | 3890159ns | -2.21% |
| riv_scr | 3862799ns | 3828348ns | 3790165ns | 3828054ns | 3951235ns | -0.23% |
| riv_snake | 3730853ns | 3725071ns | 3658510ns | 3708810ns | 3800089ns | -3.63% |
| riv_spec | 3950522ns | 3781681ns | 3718056ns | 3768706ns | 4339478ns | +2.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 3866584ns | 3766950ns | 3983974ns | base | 0.000 |
| riv_rowm | 3781001ns | 3675236ns | 3885059ns | -2.21% | 0.000 |
| riv_scr | 3857846ns | 3784911ns | 3946487ns | -0.23% | 0.000 |
| riv_snake | 3725765ns | 3653469ns | 3795030ns | -3.64% | 0.000 |
| riv_spec | 3943715ns | 3712871ns | 4329331ns | +1.99% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 95.1% |
| riv_rowm | 0.000 | 97.1% |
| riv_scr | 0.000 | 95.6% |
| riv_snake | 0.000 | 98.2% |
| riv_spec | 0.000 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 3871536ns | 3871536ns | base |
| riv_rowm | 3786069ns | 3786069ns | -2.21% |
| riv_scr | 3862799ns | 3862799ns | -0.23% |
| riv_snake | 3730853ns | 3730853ns | -3.63% |
| riv_spec | 3950522ns | 3950522ns | +2.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 3843410ns | base | --- | [3790921, 3895823] | --- | --- | --- | --- |
| riv_rowm | 3763810ns | -83333.4ns (-2.2%) | [-126033, -40477]ns | [3716483, 3798088] | YES | 0.0151 | 0.0075 | 0 |
| riv_scr | 3823144ns | no significant difference | [-39038, +54838]ns | [3809850, 3897610] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 3720110ns | -138747.9ns (-3.6%) | [-178910, -86825]ns | [3692242, 3741840] | YES | 0.0006 | 0.0001 | 0 |
| riv_spec | 3776752ns | no significant difference | [-94185, +56950]ns | [3743906, 3880875] | no | 0.6409 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 3770204ns | -1.4% | +1.1% | -2.1% | +0.2% |
| 1 | 2 | 3812217ns | -3.4% | +0.1% | -4.2% | -2.7% |
| 1 | 3 | 3880008ns | +1.8% | +0.7% | -1.8% | +3.4% |
| 2 | 1 | 3858683ns | -1.1% | +3.3% | -3.5% | -1.8% |
| 2 | 2 | 3858988ns | -1.9% | -0.9% | -3.2% | -2.1% |
| 2 | 3 | 3833438ns | -2.4% | -0.9% | -2.7% | -2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.255 | moderate+ |
| riv_rowm | -0.095 | ok |
| riv_scr | 0.147 | ok |
| riv_snake | -0.290 | moderate- |
| riv_spec | -0.304 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 2/6, lost 3/6
- **riv_snake**: won 6/6, lost 0/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 67147433.1ns | 3866583.8ns | 1736.6% | HIGH |
| riv_rowm | 67253151.8ns | 3781000.9ns | 1778.7% | HIGH |
| riv_scr | 67191642.1ns | 3857846.3ns | 1741.7% | HIGH |
| riv_snake | 67142933.8ns | 3725765.1ns | 1802.1% | HIGH |
| riv_spec | 67704950.2ns | 3943715.5ns | 1716.8% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 3766950.0-3983973.6 ns)
  3766950.0 |########################################
  3777801.2 |#############
  3788652.4 |#############
  3799503.5 |
  3810354.7 |#############
  3821205.9 |#############
  3832057.1 |#############
  3842908.3 |#############
  3853759.4 |##########################
  3864610.6 |
  3875461.8 |#############
  3886313.0 |
  3897164.2 |
  3908015.4 |#############
  3918866.5 |#############
  3929717.7 |#############
  3940568.9 |#############
  3951420.1 |
  3962271.3 |
  3973122.4 |
  (1 below, 1 above range)

riv_rowm (n=18, range 3675236.1-3885059.0 ns)
  3675236.1 |#############
  3685727.2 |
  3696218.4 |
  3706709.5 |########################################
  3717200.7 |
  3727691.8 |
  3738183.0 |##########################
  3748674.1 |#############
  3759165.3 |
  3769656.4 |#############
  3780147.6 |#############
  3790638.7 |##########################
  3801129.9 |#############
  3811621.0 |#############
  3822112.1 |#############
  3832603.3 |
  3843094.4 |
  3853585.6 |
  3864076.7 |
  3874567.9 |
  (2 below, 2 above range)

riv_scr (n=18, range 3784911.1-3946486.8 ns)
  3784911.1 |##########################
  3792989.9 |#############
  3801068.7 |#############
  3809147.5 |##########################
  3817226.3 |########################################
  3825305.0 |#############
  3833383.8 |
  3841462.6 |
  3849541.4 |
  3857620.2 |
  3865699.0 |
  3873777.7 |
  3881856.5 |##########################
  3889935.3 |
  3898014.1 |
  3906092.9 |#############
  3914171.7 |#############
  3922250.4 |
  3930329.2 |
  3938408.0 |#############
  (1 below, 2 above range)

riv_snake (n=18, range 3653469.4-3795029.9 ns)
  3653469.4 |##########################
  3660547.5 |
  3667625.5 |
  3674703.5 |
  3681781.5 |
  3688859.5 |########################################
  3695937.6 |#############
  3703015.6 |
  3710093.6 |
  3717171.6 |##########################
  3724249.7 |#############
  3731327.7 |##########################
  3738405.7 |
  3745483.7 |##########################
  3752561.7 |
  3759639.8 |
  3766717.8 |
  3773795.8 |
  3780873.8 |
  3787951.8 |
  (2 below, 3 above range)

riv_spec (n=18, range 3712870.8-4329330.5 ns)
  3712870.8 |##############################
  3743693.8 |##############################
  3774516.8 |########################################
  3805339.8 |
  3836162.8 |
  3866985.8 |##########
  3897808.7 |##########
  3928631.7 |##########
  3959454.7 |##########
  3990277.7 |##########
  4021100.7 |
  4051923.7 |
  4082746.7 |
  4113569.6 |
  4144392.6 |
  4175215.6 |
  4206038.6 |
  4236861.6 |
  4267684.6 |
  4298507.5 |
  (2 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1745.9% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1783.3% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1755.5% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1804.2% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1781.0% of algo (FFI overhead may distort results)
