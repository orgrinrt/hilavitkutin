# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_rcm is fastest but the noisiest (CV 7.0%)

riv_rcm wins on median (438.07 us) yet has the highest variance (CV 7.0%), while riv_scr is the steadiest (CV 2.8%, 464.38 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (26.86 us) is smaller than the fastest variant's own run-to-run std-dev (30.64 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (438.07 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader riv_rcm vs stability leader riv_scr (+6% speed for 2.5x steadier)

riv_rcm is fastest (438.07 us, CV 7.0%); riv_scr gives up 6.0% median for 2.5x lower variance (CV 2.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 438071.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.06x (fastest 438071.5 ns, slowest 464929.3 ns)

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
| riv_rcm | 446464ns | 440182ns | 414214ns | 434072ns | 481178ns | base |
| riv_rowm | 450458ns | 463886ns | 417741ns | 449043ns | 468939ns | +0.89% |
| riv_scr | 469377ns | 466494ns | 455370ns | 465373ns | 482388ns | +5.13% |
| riv_snake | 474729ns | 466992ns | 451245ns | 465925ns | 499676ns | +6.33% |
| riv_spec | 457592ns | 461992ns | 424206ns | 457995ns | 473681ns | +2.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 444306ns | 412037ns | 479006ns | base | 0.000 |
| riv_rowm | 448306ns | 415454ns | 466897ns | +0.90% | 0.000 |
| riv_scr | 467145ns | 453061ns | 480128ns | +5.14% | 0.000 |
| riv_snake | 472547ns | 449036ns | 497475ns | +6.36% | 0.000 |
| riv_spec | 455397ns | 422105ns | 471332ns | +2.50% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rcm; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 94.1% |
| riv_rowm | 0.000 | 89.2% |
| riv_scr | 0.000 | 88.7% |
| riv_snake | 0.000 | 88.6% |
| riv_spec | 0.000 | 89.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 446464ns | 446464ns | base |
| riv_rowm | 450458ns | 450458ns | +0.89% |
| riv_scr | 469377ns | 469377ns | +5.13% |
| riv_snake | 474729ns | 474729ns | +6.33% |
| riv_spec | 457592ns | 457592ns | +2.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 438072ns | base | --- | [415535, 465061] | --- | --- | --- | --- |
| riv_rowm | 461693ns | no significant difference | [-30661, +46901]ns | [418938, 465145] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 464381ns | no significant difference | [-1770, +45425]ns | [461426, 469683] | no | 0.3172 | 0.2379 | 0 |
| riv_snake | 464929ns | +36127.8ns (+8.2%) | [+20704, +46990]ns | [462067, 473383] | YES | 0.0302 | 0.0075 | 0 |
| riv_spec | 459778ns | +26424.9ns (+6.0%) | [+2839, +40002]ns | [451392, 463618] | YES (adj: no) | 0.1925 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 413334ns | +13.1% | +11.6% | +12.6% | +10.2% |
| 1 | 2 | 465232ns | -0.2% | +0.5% | +3.2% | +3.0% |
| 1 | 3 | 464058ns | -10.4% | -0.5% | -0.2% | -0.0% |
| 2 | 1 | 415716ns | +0.2% | +11.7% | +11.7% | +10.8% |
| 2 | 2 | 418375ns | +11.5% | +10.2% | +10.2% | +7.5% |
| 2 | 3 | 493592ns | -7.0% | -3.0% | +4.8% | -15.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.233 | moderate- |
| riv_rowm | 0.141 | ok |
| riv_scr | -0.288 | moderate- |
| riv_snake | -0.177 | ok |
| riv_spec | 0.214 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 2/6, lost 4/6
- **riv_snake**: won 1/6, lost 5/6
- **riv_spec**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 2873299.6ns | 444306.3ns | 646.7% | HIGH |
| riv_rowm | 2870957.7ns | 448305.9ns | 640.4% | HIGH |
| riv_scr | 2873708.6ns | 467145.4ns | 615.2% | HIGH |
| riv_snake | 2876501.4ns | 472546.7ns | 608.7% | HIGH |
| riv_spec | 2873853.5ns | 455397.3ns | 631.1% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 412036.7-479005.7 ns)
  412036.7 |##############################
  415385.1 |##############################
  418733.6 |##########
  422082.0 |
  425430.5 |
  428778.9 |
  432127.4 |
  435475.8 |
  438824.3 |
  442172.7 |
  445521.2 |
  448869.7 |
  452218.1 |
  455566.6 |##########
  458915.0 |
  462263.5 |########################################
  465611.9 |##########
  468960.4 |
  472308.8 |
  475657.3 |
  (2 below, 3 above range)

riv_rowm (n=18, range 415454.3-466897.3 ns)
  415454.3 |########################################
  418026.5 |##########
  420598.6 |
  423170.8 |
  425742.9 |
  428315.1 |
  430887.2 |
  433459.4 |
  436031.5 |
  438603.7 |
  441175.8 |
  443748.0 |
  446320.1 |
  448892.3 |
  451464.4 |
  454036.6 |
  456608.7 |##########
  459180.9 |####################
  461753.0 |##############################
  464325.2 |##############################
  (1 below, 3 above range)

riv_scr (n=18, range 453061.3-480127.7 ns)
  453061.3 |
  454414.6 |
  455768.0 |
  457121.3 |
  458474.6 |
  459827.9 |##############################
  461181.2 |########################################
  462534.5 |
  463887.9 |##############################
  465241.2 |##########
  466594.5 |##########
  467947.8 |
  469301.1 |
  470654.5 |##########
  472007.8 |
  473361.1 |##########
  474714.4 |
  476067.7 |
  477421.0 |
  478774.4 |##########
  (1 below, 2 above range)

riv_snake (n=18, range 449035.7-497475.4 ns)
  449035.7 |
  451457.7 |
  453879.6 |
  456301.6 |##########
  458723.6 |##############################
  461145.6 |####################
  463567.6 |########################################
  465989.6 |####################
  468411.6 |
  470833.5 |
  473255.5 |
  475677.5 |
  478099.5 |##########
  480521.5 |
  482943.5 |##########
  485365.5 |
  487787.5 |
  490209.4 |
  492631.4 |
  495053.4 |
  (1 below, 3 above range)

riv_spec (n=18, range 422105.1-471332.0 ns)
  422105.1 |
  424566.4 |
  427027.8 |
  429489.1 |
  431950.5 |
  434411.8 |##########
  436873.2 |
  439334.5 |
  441795.8 |
  444257.2 |
  446718.5 |##########
  449179.9 |##########
  451641.2 |##########
  454102.6 |####################
  456563.9 |
  459025.3 |########################################
  461486.6 |####################
  463948.0 |
  466409.3 |##########
  468870.7 |
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=655.5% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=621.7% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=618.7% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=617.5% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=624.4% of algo (FFI overhead may distort results)
