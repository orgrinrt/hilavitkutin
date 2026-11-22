# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (43.75 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_spec at 42.08 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (riv_spec, riv_scr) are a dead heat (<1%)

riv_spec (42.08 ms) and riv_scr (42.47 ms) differ by 0.93%, inside the noise, even though the wider field spreads 4.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.67 ms) is smaller than the fastest variant's own run-to-run std-dev (5.37 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_spec vs stability leader riv_rowm (+2% speed for 1.8x steadier)

riv_spec is fastest (42.08 ms, CV 12.8%); riv_rowm gives up 1.7% median for 1.8x lower variance (CV 7.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_spec** at 42076856.2 ns median (-3.8% vs baseline)
- Spread: 1.04x (fastest 42076856.2 ns, slowest 43750041.6 ns)

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
| riv_rcm | 43711460ns | 43760552ns | 38377508ns | 42708589ns | 47882742ns | base |
| riv_rowm | 43861364ns | 42800540ns | 40630694ns | 42456746ns | 47583626ns | +0.34% |
| riv_scr | 42732716ns | 42477202ns | 39025256ns | 41685606ns | 46157113ns | -2.24% |
| riv_snake | 43206442ns | 43192110ns | 33775847ns | 42725733ns | 48642803ns | -1.16% |
| riv_spec | 43441735ns | 42084850ns | 36169807ns | 42069745ns | 49135685ns | -0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 43703056ns | 38369608ns | 47873708ns | base | 0.000 |
| riv_rowm | 43853608ns | 40623094ns | 47575315ns | +0.34% | 0.000 |
| riv_scr | 42725088ns | 39017746ns | 46148840ns | -2.24% | 0.000 |
| riv_snake | 43198195ns | 33766983ns | 48633984ns | -1.16% | 0.000 |
| riv_spec | 43433218ns | 36162149ns | 49124649ns | -0.62% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 77.2% |
| riv_rowm | 0.000 | 78.9% |
| riv_scr | 0.000 | 79.5% |
| riv_snake | 0.000 | 78.2% |
| riv_spec | 0.000 | 80.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 43711460ns | 43711460ns | base |
| riv_rowm | 43861364ns | 43861364ns | +0.34% |
| riv_scr | 42732716ns | 42732716ns | -2.24% |
| riv_snake | 43206442ns | 43206442ns | -1.16% |
| riv_spec | 43441735ns | 43441735ns | -0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 43750042ns | base | --- | [41260415, 44935977] | --- | --- | --- | --- |
| riv_rowm | 42792488ns | no significant difference | [-2993283, +3638154]ns | [41712433, 45644835] | no | 1.0000 | 1.0000 | 0 |
| riv_scr | 42469862ns | no significant difference | [-2917823, +896131]ns | [40718381, 43133177] | no | 1.0000 | 0.4807 | 0 |
| riv_snake | 43183702ns | no significant difference | [-2626838, +2372033]ns | [42304944, 43875704] | no | 1.0000 | 0.8145 | 0 |
| riv_spec | 42076856ns | no significant difference | [-2981473, +3195021]ns | [40924506, 45556133] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 44958138ns | -5.2% | -4.7% | -4.6% | +3.5% |
| 1 | 2 | 41182054ns | +0.5% | -1.1% | +11.9% | +2.2% |
| 1 | 3 | 46277554ns | -8.8% | -7.8% | +14.7% | -9.9% |
| 2 | 1 | 43644046ns | +0.4% | -2.5% | -1.2% | +2.5% |
| 2 | 2 | 42912758ns | -2.4% | -5.2% | +0.9% | -5.5% |
| 2 | 3 | 40696467ns | +9.9% | -2.2% | -17.9% | -12.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.341 | moderate- |
| riv_rowm | -0.280 | moderate- |
| riv_scr | -0.026 | ok |
| riv_snake | 0.099 | ok |
| riv_spec | 0.068 | ok |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 6/6, lost 0/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 553321241.4ns | 43703055.8ns | 1266.1% | HIGH |
| riv_rowm | 554253518.5ns | 43853607.9ns | 1263.9% | HIGH |
| riv_scr | 552121074.3ns | 42725087.7ns | 1292.3% | HIGH |
| riv_snake | 552509105.3ns | 43198195.4ns | 1279.0% | HIGH |
| riv_spec | 553895856.9ns | 43433217.6ns | 1275.3% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 38369608.3-47873708.3 ns)
  38369608.3 |
  38844813.3 |
  39320018.3 |#############
  39795223.3 |
  40270428.3 |##########################
  40745633.3 |#############
  41220838.3 |##########################
  41696043.3 |
  42171248.3 |
  42646453.3 |#############
  43121658.3 |
  43596863.3 |##########################
  44072068.3 |#############
  44547273.3 |########################################
  45022478.3 |
  45497683.3 |
  45972888.3 |#############
  46448093.3 |#############
  46923298.3 |
  47398503.3 |
  (1 below, 2 above range)

riv_rowm (n=18, range 40623094.4-47575315.3 ns)
  40623094.4 |
  40970705.5 |########################################
  41318316.5 |#############
  41665927.6 |#############
  42013538.6 |##########################
  42361149.6 |#############
  42708760.7 |#############
  43056371.7 |
  43403982.8 |#############
  43751593.8 |#############
  44099204.8 |
  44446815.9 |#############
  44794426.9 |
  45142038.0 |
  45489649.0 |
  45837260.1 |
  46184871.1 |
  46532482.1 |#############
  46880093.2 |#############
  47227704.2 |
  (1 below, 3 above range)

riv_scr (n=18, range 39017745.8-46148840.3 ns)
  39017745.8 |
  39374300.6 |#############
  39730855.3 |#############
  40087410.0 |#############
  40443964.7 |##########################
  40800519.4 |#############
  41157074.2 |
  41513628.9 |
  41870183.6 |#############
  42226738.3 |##########################
  42583293.1 |########################################
  42939847.8 |
  43296402.5 |#############
  43652957.2 |
  44009511.9 |
  44366066.7 |#############
  44722621.4 |
  45079176.1 |
  45435730.8 |
  45792285.6 |#############
  (1 below, 2 above range)

riv_snake (n=18, range 33766983.3-48633984.0 ns)
  33766983.3 |
  34510333.4 |######
  35253683.4 |
  35997033.4 |
  36740383.5 |
  37483733.5 |
  38227083.5 |
  38970433.6 |
  39713783.6 |
  40457133.6 |######
  41200483.7 |
  41943833.7 |#############
  42687183.8 |########################################
  43430533.8 |#############
  44173883.8 |
  44917233.9 |
  45660583.9 |######
  46403933.9 |######
  47147284.0 |
  47890634.0 |
  (2 below, 2 above range)

riv_spec (n=18, range 36162148.6-49124649.3 ns)
  36162148.6 |
  36810273.6 |
  37458398.7 |
  38106523.7 |
  38754648.7 |
  39402773.8 |
  40050898.8 |########################################
  40699023.8 |##########################
  41347148.9 |#############
  41995273.9 |##########################
  42643399.0 |
  43291524.0 |
  43939649.0 |#############
  44587774.1 |#############
  45235899.1 |##########################
  45884024.1 |
  46532149.2 |#############
  47180274.2 |
  47828399.2 |#############
  48476524.3 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1263.8% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1290.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1297.5% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1276.3% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1309.0% of algo (FFI overhead may distort results)
