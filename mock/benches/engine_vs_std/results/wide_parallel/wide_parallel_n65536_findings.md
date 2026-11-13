# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (796.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest engine_par at 463.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### engine_par beats baseline by 48% (significant)

engine_par is -382.38 us (48%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {engine_par, std_par} vs {std, engine} (54% apart)

The field splits into a fast tier {engine_par, std_par} and a slow tier {std, engine} with a 54% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### engine_par is inconsistent: worst-20% is 1.8x its best-20%

engine_par's best 20% of batches run at 355.47 us but its worst 20% at 627.52 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: engine_par** at 463111.8 ns median (-41.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.72x (fastest 463111.8 ns, slowest 796499.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, four independent heavy chains |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000004 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 821467ns | 798319ns | 773064ns | 784934ns | 860968ns | base |
| engine_par | 518597ns | 463863ns | 359021ns | 425818ns | 628076ns | -36.87% |
| std | 887878ns | 767739ns | 738000ns | 755793ns | 1024412ns | +8.08% |
| std_par | 546610ns | 500908ns | 461358ns | 472506ns | 623501ns | -33.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 820105ns | 772476ns | 859383ns | base | 0.080 |
| engine_par | 517478ns | 355471ns | 627524ns | -36.90% | 0.127 |
| std | 885729ns | 736544ns | 1022088ns | +8.00% | 0.074 |
| std_par | 544491ns | 459516ns | 621488ns | -33.61% | 0.120 |

## Performance model

- Peak throughput: **0.184 Gops/s** (engine_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.082 | 44.6% |
| engine_par | 0.142 | 76.8% |
| std | 0.086 | 46.4% |
| std_par | 0.132 | 71.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 821467ns | 821467ns | base |
| engine_par | 518597ns | 518597ns | -36.87% |
| std | 887878ns | 887878ns | +8.08% |
| std_par | 546610ns | 546610ns | -33.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 796499ns | base | --- | [779825, 874494] | --- | --- | --- | --- |
| engine_par | 463112ns | -382381.2ns (-48.0%) | [-431687, -206615]ns | [398648, 573210] | YES (adj: no) | 0.1055 | 0.0703 | 0 |
| std | 765978ns | no significant difference | [-115930, +70167]ns | [748836, 842643] | no | 0.7266 | 0.7266 | 0 |
| std_par | 498107ns | -278044.0ns (-34.9%) | [-405012, -223854]ns | [464266, 581986] | YES | 0.0234 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 824551ns | -34.2% | -1.1% | -42.6% |
| 1 | 2 | 796499ns | -48.3% | -5.4% | -29.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 2/2, lost 0/2
- **std**: won 2/2, lost 0/2
- **std_par**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 1172014.8ns | 820105.2ns | 142.9% | HIGH |
| engine_par | 1228193.4ns | 517477.5ns | 237.3% | HIGH |
| std | 1210625.6ns | 885728.9ns | 136.7% | HIGH |
| std_par | 1280778.5ns | 544490.8ns | 235.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 772476.2-859383.2 ns)
  772476.2 |########################################
  776821.5 |########################################
  781166.9 |########################################
  785512.2 |########################################
  789857.6 |
  794202.9 |
  798548.3 |
  802893.7 |########################################
  807239.0 |
  811584.3 |
  815929.7 |
  820275.1 |
  824620.4 |
  828965.8 |
  833311.1 |
  837656.5 |
  842001.8 |
  846347.2 |
  850692.5 |
  855037.9 |
  (0 below, 3 above range)

engine_par (n=8, range 355471.0-627524.2 ns)
  355471.0 |########################################
  369073.7 |
  382676.3 |
  396279.0 |########################################
  409881.6 |
  423484.3 |########################################
  437087.0 |########################################
  450689.6 |
  464292.3 |########################################
  477895.0 |
  491497.6 |
  505100.3 |########################################
  518702.9 |
  532305.6 |
  545908.3 |
  559510.9 |
  573113.6 |########################################
  586716.2 |
  600318.9 |
  613921.6 |
  (0 below, 1 above range)

std (n=8, range 736544.1-1022088.2 ns)
  736544.1 |########################################
  750821.3 |########################################
  765098.5 |####################
  779375.7 |####################
  793652.9 |
  807930.1 |
  822207.3 |
  836484.6 |####################
  850761.8 |
  865039.0 |
  879316.2 |
  893593.4 |
  907870.6 |
  922147.8 |
  936425.0 |
  950702.2 |
  964979.4 |
  979256.6 |
  993533.8 |
  1007811.0 |
  (0 below, 1 above range)

std_par (n=8, range 459516.4-621488.5 ns)
  459516.4 |########################################
  467615.0 |########################################
  475713.6 |
  483812.2 |
  491910.8 |
  500009.4 |
  508108.0 |
  516206.6 |####################
  524305.2 |
  532403.8 |####################
  540502.4 |
  548601.1 |
  556699.7 |
  564798.3 |
  572896.9 |
  580995.5 |####################
  589094.1 |
  597192.7 |
  605291.3 |
  613389.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: bridge=147.5% of algo (FFI overhead may distort results)
- **engine_par**: CV=33.8% (high variance, measurements may be unstable)
- **engine_par**: bridge=266.8% of algo (FFI overhead may distort results)
- **std**: CV=34.3% (high variance, measurements may be unstable)
- **std**: bridge=158.8% of algo (FFI overhead may distort results)
- **std_par**: CV=22.0% (high variance, measurements may be unstable)
- **std_par**: bridge=256.6% of algo (FFI overhead may distort results)
