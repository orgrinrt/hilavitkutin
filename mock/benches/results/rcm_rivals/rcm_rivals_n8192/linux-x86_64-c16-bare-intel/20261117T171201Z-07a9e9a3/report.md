# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (74.55 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.8% of the fastest

All 5 variants sit between 74.55 ms and 75.12 ms - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 74552322.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 74552322.8 ns, slowest 75121623.0 ns)

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
| riv_rcm | 74594356ns | 74556208ns | 74103294ns | 74456562ns | 75046579ns | base |
| riv_rowm | 76015598ns | 75125477ns | 74224375ns | 74971788ns | 78476924ns | +1.91% |
| riv_scr | 75307357ns | 74644823ns | 74150106ns | 74620222ns | 76916686ns | +0.96% |
| riv_snake | 75349064ns | 74564281ns | 74072106ns | 74415862ns | 77387346ns | +1.01% |
| riv_spec | 75711722ns | 74910732ns | 74033512ns | 74781302ns | 77946456ns | +1.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 74590426ns | 74099411ns | 75042702ns | base | 0.000 |
| riv_rowm | 76011569ns | 74220240ns | 78472848ns | +1.91% | 0.000 |
| riv_scr | 75303338ns | 74146058ns | 76912740ns | +0.96% | 0.000 |
| riv_snake | 75345034ns | 74068127ns | 77383233ns | +1.01% | 0.000 |
| riv_spec | 75707508ns | 74029479ns | 77941832ns | +1.50% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 99.3% |
| riv_rowm | 0.000 | 98.5% |
| riv_scr | 0.000 | 99.2% |
| riv_snake | 0.000 | 99.3% |
| riv_spec | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 74594356ns | 74594356ns | base |
| riv_rowm | 76015598ns | 76015598ns | +1.91% |
| riv_scr | 75307357ns | 75307357ns | +0.96% |
| riv_snake | 75349064ns | 75349064ns | +1.01% |
| riv_spec | 75711722ns | 75711722ns | +1.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 74552323ns | base | --- | [74254839, 74821387] | --- | --- | --- | --- |
| riv_rowm | 75121623ns | +265300.8ns (+0.4%) | [+150766, +1258508]ns | [74790420, 75760956] | YES | 0.0052 | 0.0013 | 0 |
| riv_scr | 74640779ns | no significant difference | [-170684, +709533]ns | [74478666, 75045150] | no | 0.6409 | 0.4807 | 0 |
| riv_snake | 74560367ns | no significant difference | [-224185, +398628]ns | [74229446, 74935485] | no | 1.0000 | 1.0000 | 0 |
| riv_spec | 74906612ns | no significant difference | [-24967, +581848]ns | [74599599, 75053378] | no | 0.4758 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 74176203ns | -0.0% | -0.0% | +0.0% | -0.3% |
| 1 | 2 | 74450539ns | +0.7% | -0.1% | +3.4% | +0.5% |
| 1 | 3 | 74601974ns | +0.9% | -0.0% | -0.1% | -0.0% |
| 2 | 1 | 74942571ns | +0.5% | -0.4% | -0.4% | -0.0% |
| 2 | 2 | 74770318ns | +0.2% | +0.9% | +0.6% | +0.2% |
| 2 | 3 | 74262007ns | +0.3% | +1.0% | -0.2% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.193 | ok |
| riv_rowm | 0.148 | ok |
| riv_scr | 0.386 | moderate+ |
| riv_snake | -0.438 | moderate- |
| riv_spec | 0.027 | ok |

**Consistency summary:**

- **riv_rowm**: won 0/6, lost 5/6
- **riv_scr**: won 2/6, lost 2/6
- **riv_snake**: won 2/6, lost 2/6
- **riv_spec**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 369097516.2ns | 74590426.0ns | 494.8% | HIGH |
| riv_rowm | 370324134.7ns | 76011569.0ns | 487.2% | HIGH |
| riv_scr | 369648016.4ns | 75303338.4ns | 490.9% | HIGH |
| riv_snake | 369610428.9ns | 75345034.1ns | 490.6% | HIGH |
| riv_spec | 369437308.5ns | 75707508.2ns | 488.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 74099410.6-75042701.9 ns)
  74099410.6 |
  74146575.2 |####################
  74193739.7 |########################################
  74240904.3 |########################################
  74288068.9 |####################
  74335233.4 |
  74382398.0 |
  74429562.6 |####################
  74476727.1 |####################
  74523891.7 |
  74571056.2 |####################
  74618220.8 |
  74665385.4 |####################
  74712549.9 |
  74759714.5 |####################
  74806879.1 |########################################
  74854043.6 |####################
  74901208.2 |####################
  74948372.8 |####################
  74995537.3 |
  (1 below, 1 above range)

riv_rowm (n=18, range 74220240.2-78472848.3 ns)
  74220240.2 |#############
  74432870.6 |#############
  74645501.0 |##########################
  74858131.4 |########################################
  75070761.8 |##########################
  75283392.3 |##########################
  75496022.7 |
  75708653.1 |
  75921283.5 |
  76133913.9 |#############
  76346544.3 |
  76559174.7 |##########################
  76771805.1 |
  76984435.5 |
  77197065.9 |
  77409696.3 |
  77622326.7 |
  77834957.1 |
  78047587.5 |
  78260217.9 |
  (2 below, 2 above range)

riv_scr (n=18, range 74146058.0-76912740.4 ns)
  74146058.0 |########
  74284392.1 |################
  74422726.2 |
  74561060.4 |########################################
  74699394.5 |
  74837728.6 |################
  74976062.7 |################
  75114396.8 |
  75252731.0 |
  75391065.1 |################
  75529399.2 |
  75667733.3 |
  75806067.5 |
  75944401.6 |
  76082735.7 |
  76221069.8 |########
  76359403.9 |
  76497738.1 |
  76636072.2 |
  76774406.3 |
  (2 below, 1 above range)

riv_snake (n=18, range 74068126.7-77383233.1 ns)
  74068126.7 |##############################
  74233882.0 |####################
  74399637.3 |####################
  74565392.7 |########################################
  74731148.0 |
  74896903.3 |
  75062658.6 |##########
  75228414.0 |
  75394169.3 |
  75559924.6 |
  75725679.9 |
  75891435.2 |
  76057190.6 |
  76222945.9 |
  76388701.2 |
  76554456.5 |
  76720211.8 |
  76885967.2 |##########
  77051722.5 |
  77217477.8 |
  (2 below, 3 above range)

riv_spec (n=18, range 74029479.0-77941831.5 ns)
  74029479.0 |
  74225096.6 |
  74420714.2 |####################
  74616331.8 |#############
  74811949.5 |########################################
  75007567.1 |######
  75203184.7 |######
  75398802.4 |
  75594420.0 |
  75790037.6 |
  75985655.2 |
  76181272.9 |
  76376890.5 |
  76572508.1 |
  76768125.8 |
  76963743.4 |
  77159361.0 |
  77354978.6 |
  77550596.3 |
  77746213.9 |######
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=494.9% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=492.6% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=494.1% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=495.2% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=492.4% of algo (FFI overhead may distort results)
