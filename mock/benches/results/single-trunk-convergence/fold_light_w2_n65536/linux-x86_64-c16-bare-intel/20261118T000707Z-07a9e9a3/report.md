# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 92% faster than the next best (head_tail)

sequential (13.45 us) leads head_tail (25.87 us) by 92%, a clear separation rather than a photo finish. CV 107.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 43% (significant)

sequential is -11.25 us (43%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 5.4x slower than the field

std_parallel (72.23 us) is 5.4x the fastest (13.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 107.5%)

sequential wins on median (13.45 us) yet has the highest variance (CV 107.5%), while std_parallel is the steadiest (CV 10.9%, 72.23 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.60)

kernel_seq's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.4x the fastest

Fastest sequential (13.45 us) to slowest std_parallel (72.23 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.2x its best-20%

sequential's best 20% of batches run at 12.04 us but its worst 20% at 38.71 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 13448.7 ns median (-48.0% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.37x (fastest 13448.7 ns, slowest 72233.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 30333ns | 26758ns | 24313ns | 26712ns | 38775ns | base |
| head_tail_dyn | 50549ns | 51156ns | 32493ns | 48793ns | 62209ns | +66.65% |
| kernel_nway | 29804ns | 28541ns | 22672ns | 27738ns | 36467ns | -1.74% |
| kernel_seq | 31611ns | 29342ns | 25563ns | 28761ns | 38911ns | +4.22% |
| nway | 61554ns | 59804ns | 46493ns | 57360ns | 75375ns | +102.93% |
| sequential | 21781ns | 13616ns | 12202ns | 13340ns | 39231ns | -28.19% |
| std_parallel | 72198ns | 73212ns | 60190ns | 70335ns | 80996ns | +138.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 29377ns | 23617ns | 37511ns | base | 2.231 |
| head_tail_dyn | 49664ns | 32065ns | 60721ns | +69.06% | 1.320 |
| kernel_nway | 28692ns | 22093ns | 35017ns | -2.33% | 2.284 |
| kernel_seq | 30684ns | 25061ns | 37469ns | +4.45% | 2.136 |
| nway | 60786ns | 45963ns | 74309ns | +106.92% | 1.078 |
| sequential | 21502ns | 12044ns | 38714ns | -26.80% | 3.048 |
| std_parallel | 71325ns | 59626ns | 79816ns | +142.80% | 0.919 |

## Performance model

- Peak throughput: **5.442 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.533 | 46.6% |
| head_tail_dyn | 1.295 | 23.8% |
| kernel_nway | 2.348 | 43.1% |
| kernel_seq | 2.285 | 42.0% |
| nway | 1.109 | 20.4% |
| sequential | 4.873 | 89.6% |
| std_parallel | 0.907 | 16.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30333ns | 30333ns | base |
| head_tail_dyn | 50549ns | 50549ns | +66.65% |
| kernel_nway | 29804ns | 29804ns | -1.74% |
| kernel_seq | 31611ns | 31611ns | +4.22% |
| nway | 61554ns | 61554ns | +102.93% |
| sequential | 21781ns | 21781ns | -28.19% |
| std_parallel | 72198ns | 72198ns | +138.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 25872ns | base | --- | [24337, 29780] | --- | --- | --- | --- |
| head_tail_dyn | 50604ns | +21050.2ns (+81.4%) | [+18743, +26495]ns | [45353, 53458] | YES | 0.0026 | 0.0013 | 0 |
| kernel_nway | 27912ns | no significant difference | [-476, +2741]ns | [24412, 31363] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 28681ns | no significant difference | [-4660, +5537]ns | [27078, 30519] | no | 0.4807 | 0.4807 | 0 |
| nway | 59119ns | +32042.8ns (+123.9%) | [+22244, +36867]ns | [53326, 63364] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13449ns | -11246.7ns (-43.5%) | [-14299, -9957]ns | [12786, 21519] | YES | 0.0463 | 0.0309 | 0 |
| std_parallel | 72233ns | +45985.4ns (+177.7%) | [+40878, +47276]ns | [64644, 76542] | YES | 0.0004 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25466ns | +78.1% | +9.6% | +9.0% | +144.6% | -20.4% | +150.6% |
| 1 | 2 | 24517ns | +108.8% | -4.0% | +24.6% | +147.0% | -44.4% | +188.4% |
| 1 | 3 | 31133ns | +65.6% | +8.5% | -15.1% | +60.4% | -58.3% | +154.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.248 | moderate- |
| head_tail_dyn | -0.137 | ok |
| kernel_nway | -0.449 | moderate- |
| kernel_seq | -0.600 | HIGH- (thermal bounce) |
| nway | -0.098 | ok |
| sequential | -0.121 | ok |
| std_parallel | -0.003 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 28460.2ns | 29376.6ns | 96.9% | HIGH |
| head_tail_dyn | 51932.7ns | 49664.0ns | 104.6% | HIGH |
| kernel_nway | 28971.5ns | 28691.8ns | 101.0% | HIGH |
| kernel_seq | 30194.0ns | 30684.5ns | 98.4% | HIGH |
| nway | 63042.5ns | 60786.0ns | 103.7% | HIGH |
| sequential | 22098.7ns | 21502.4ns | 102.8% | HIGH |
| std_parallel | 73884.2ns | 71325.0ns | 103.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23616.6-37510.6 ns)
  23616.6 |########################################
  24311.3 |##############################
  25006.0 |##########
  25700.7 |
  26395.4 |##########
  27090.1 |
  27784.8 |
  28479.5 |
  29174.2 |########################################
  29868.9 |
  30563.6 |
  31258.3 |
  31953.0 |##########
  32647.7 |
  33342.4 |##########
  34037.1 |
  34731.8 |
  35426.5 |
  36121.2 |##########
  36815.9 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 32065.1-60721.4 ns)
  32065.1 |
  33497.9 |
  34930.7 |
  36363.5 |
  37796.3 |
  39229.1 |
  40662.0 |##########################
  42094.8 |
  43527.6 |#############
  44960.4 |#############
  46393.2 |
  47826.0 |#############
  49258.9 |##########################
  50691.7 |########################################
  52124.5 |#############
  53557.3 |#############
  54990.1 |
  56422.9 |#############
  57855.8 |#############
  59288.6 |
  (2 below, 2 above range)

kernel_nway (n=18, range 22092.6-35017.3 ns)
  22092.6 |
  22738.8 |####################
  23385.1 |####################
  24031.3 |####################
  24677.5 |########################################
  25323.8 |####################
  25970.0 |
  26616.2 |####################
  27262.5 |
  27908.7 |
  28554.9 |####################
  29201.2 |
  29847.4 |####################
  30493.7 |########################################
  31139.9 |
  31786.1 |####################
  32432.4 |
  33078.6 |####################
  33724.8 |
  34371.1 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 25060.9-37468.8 ns)
  25060.9 |##########################
  25681.3 |#############
  26301.7 |#############
  26922.0 |#############
  27542.4 |########################################
  28162.8 |
  28783.2 |##########################
  29403.6 |#############
  30024.0 |########################################
  30644.4 |
  31264.8 |
  31885.2 |#############
  32505.6 |
  33126.0 |
  33746.4 |
  34366.8 |#############
  34987.2 |
  35607.6 |
  36228.0 |
  36848.4 |
  (1 below, 1 above range)

nway (n=18, range 45962.8-74309.4 ns)
  45962.8 |####################
  47380.1 |
  48797.5 |####################
  50214.8 |####################
  51632.1 |####################
  53049.4 |
  54466.8 |########################################
  55884.1 |####################
  57301.4 |
  58718.8 |########################################
  60136.1 |####################
  61553.4 |########################################
  62970.8 |####################
  64388.1 |
  65805.4 |
  67222.7 |####################
  68640.1 |####################
  70057.4 |
  71474.7 |
  72892.1 |
  (1 below, 2 above range)

sequential (n=18, range 12043.5-38714.0 ns)
  12043.5 |########################################
  13377.1 |######################
  14710.6 |
  16044.1 |
  17377.6 |
  18711.2 |
  20044.7 |
  21378.2 |
  22711.7 |
  24045.3 |
  25378.8 |
  26712.3 |
  28045.8 |#####
  29379.4 |
  30712.9 |
  32046.4 |
  33379.9 |
  34713.5 |
  36047.0 |
  37380.5 |
  (2 below, 4 above range)

std_parallel (n=18, range 59626.1-79815.8 ns)
  59626.1 |####################
  60635.6 |
  61645.1 |####################
  62654.6 |####################
  63664.1 |
  64673.6 |####################
  65683.0 |
  66692.5 |
  67702.0 |
  68711.5 |
  69721.0 |########################################
  70730.5 |####################
  71739.9 |
  72749.4 |####################
  73758.9 |####################
  74768.4 |####################
  75777.9 |########################################
  76787.4 |####################
  77796.8 |
  78806.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=30.7% (high variance, measurements may be unstable)
- **head_tail**: bridge=108.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=23.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=29.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.7% of algo (FFI overhead may distort results)
- **nway**: CV=21.4% (high variance, measurements may be unstable)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: CV=67.2% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **sequential**: bridge=101.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.8% of algo (FFI overhead may distort results)
