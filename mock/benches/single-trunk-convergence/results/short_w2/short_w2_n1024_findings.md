# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 240% faster than the next best (kernel_seq)

sequential (7.55 us) leads kernel_seq (25.65 us) by 240%, a clear separation rather than a photo finish. CV 28.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 75% (significant)

sequential is -24.04 us (75%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 6.8x slower than the field

std_parallel (51.65 us) is 6.8x the fastest (7.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, kernel_nway, head_tail, nway, std_parallel} (240% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, kernel_nway, head_tail, nway, std_parallel} with a 240% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.8x the fastest

Fastest sequential (7.55 us) to slowest std_parallel (51.65 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 1.7x its best-20%

sequential's best 20% of batches run at 7.10 us but its worst 20% at 11.77 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7547.0 ns median (-76.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 6.84x (fastest 7547.0 ns, slowest 51645.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 35134ns | 32688ns | 28224ns | 32000ns | 43290ns | base |
| head_tail_dyn | 29243ns | 28613ns | 25382ns | 28246ns | 32668ns | -16.77% |
| kernel_nway | 30231ns | 30730ns | 25718ns | 29378ns | 33767ns | -13.96% |
| kernel_seq | 26293ns | 26279ns | 22701ns | 25661ns | 29037ns | -25.16% |
| nway | 40923ns | 40686ns | 35489ns | 39411ns | 45908ns | +16.48% |
| sequential | 9057ns | 7775ns | 7329ns | 7591ns | 12121ns | -74.22% |
| std_parallel | 53330ns | 51999ns | 48802ns | 51598ns | 58190ns | +51.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 34547ns | 27770ns | 42528ns | base | 0.030 |
| head_tail_dyn | 28775ns | 24944ns | 32121ns | -16.71% | 0.036 |
| kernel_nway | 29759ns | 25261ns | 33220ns | -13.86% | 0.034 |
| kernel_seq | 25798ns | 22229ns | 28592ns | -25.32% | 0.040 |
| nway | 40573ns | 35169ns | 45536ns | +17.44% | 0.025 |
| sequential | 8784ns | 7104ns | 11765ns | -74.57% | 0.117 |
| std_parallel | 52908ns | 48433ns | 57681ns | +53.15% | 0.019 |

## Performance model

- Peak throughput: **0.144 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.032 | 22.1% |
| head_tail_dyn | 0.036 | 25.1% |
| kernel_nway | 0.034 | 23.6% |
| kernel_seq | 0.040 | 27.7% |
| nway | 0.025 | 17.6% |
| sequential | 0.136 | 94.1% |
| std_parallel | 0.020 | 13.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 35134ns | 35134ns | base |
| head_tail_dyn | 29243ns | 29243ns | -16.77% |
| kernel_nway | 30231ns | 30231ns | -13.96% |
| kernel_seq | 26293ns | 26293ns | -25.16% |
| nway | 40923ns | 40923ns | +16.48% |
| sequential | 9057ns | 9057ns | -74.22% |
| std_parallel | 53330ns | 53330ns | +51.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 32150ns | base | --- | [30366, 36007] | --- | --- | --- | --- |
| head_tail_dyn | 28261ns | no significant difference | [-8452, +281]ns | [27270, 31091] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 30143ns | no significant difference | [-8055, +1451]ns | [27127, 31880] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 25647ns | -6930.5ns (-21.6%) | [-10279, -4790]ns | [24183, 27707] | YES | 0.0026 | 0.0013 | 0 |
| nway | 40338ns | +6287.1ns (+19.6%) | [+2139, +12649]ns | [37108, 42911] | YES | 0.0463 | 0.0309 | 0 |
| sequential | 7547ns | -24035.5ns (-74.8%) | [-28100, -20647]ns | [7118, 10916] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 51645ns | +17941.4ns (+55.8%) | [+14983, +22779]ns | [50326, 53843] | YES | 0.0004 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30104ns | -6.1% | +7.6% | -24.7% | +47.7% | -60.4% | +68.4% |
| 1 | 2 | 37099ns | -28.1% | -29.4% | -28.2% | -2.0% | -80.2% | +36.8% |
| 1 | 3 | 30718ns | +4.6% | -0.1% | -7.7% | +29.4% | -76.8% | +83.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.662 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.349 | moderate- |
| kernel_nway | -0.619 | HIGH- (thermal bounce) |
| kernel_seq | -0.033 | ok |
| nway | -0.443 | moderate- |
| sequential | -0.143 | ok |
| std_parallel | -0.160 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 32791.6ns | 34546.7ns | 94.9% | HIGH |
| head_tail_dyn | 33077.6ns | 28774.6ns | 115.0% | HIGH |
| kernel_nway | 30181.4ns | 29758.8ns | 101.4% | HIGH |
| kernel_seq | 26192.6ns | 25798.1ns | 101.5% | HIGH |
| nway | 41210.1ns | 40572.6ns | 101.6% | HIGH |
| sequential | 9175.4ns | 8783.7ns | 104.5% | HIGH |
| std_parallel | 53827.0ns | 52907.8ns | 101.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 27770.3-42528.3 ns)
  27770.3 |####################
  28508.2 |####################
  29246.1 |####################
  29984.0 |########################################
  30721.9 |########################################
  31459.8 |####################
  32197.7 |####################
  32935.6 |####################
  33673.5 |####################
  34411.4 |####################
  35149.3 |
  35887.2 |
  36625.1 |########################################
  37363.0 |####################
  38100.9 |####################
  38838.8 |
  39576.7 |
  40314.6 |
  41052.5 |
  41790.4 |
  (1 below, 1 above range)

head_tail_dyn (n=18, range 24944.0-32121.0 ns)
  24944.0 |#############
  25302.8 |
  25661.7 |#############
  26020.5 |#############
  26379.4 |
  26738.2 |
  27097.1 |##########################
  27455.9 |
  27814.8 |########################################
  28173.6 |##########################
  28532.5 |#############
  28891.4 |
  29250.2 |
  29609.1 |
  29967.9 |
  30326.8 |#############
  30685.6 |
  31044.5 |
  31403.3 |#############
  31762.2 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 25261.2-33219.5 ns)
  25261.2 |####################
  25659.1 |
  26057.0 |
  26454.9 |########################################
  26852.8 |
  27250.8 |####################
  27648.7 |####################
  28046.6 |####################
  28444.5 |
  28842.4 |
  29240.3 |
  29638.2 |####################
  30036.2 |####################
  30434.1 |
  30832.0 |
  31229.9 |########################################
  31627.8 |########################################
  32025.8 |####################
  32423.7 |####################
  32821.6 |
  (2 below, 2 above range)

kernel_seq (n=18, range 22229.2-28591.8 ns)
  22229.2 |
  22547.4 |##########################
  22865.5 |
  23183.6 |
  23501.7 |
  23819.9 |#############
  24138.0 |#############
  24456.1 |
  24774.3 |#############
  25092.4 |
  25410.5 |########################################
  25728.6 |
  26046.8 |
  26364.9 |#############
  26683.0 |#############
  27001.1 |
  27319.3 |#############
  27637.4 |#############
  27955.5 |
  28273.7 |########################################
  (2 below, 1 above range)

nway (n=18, range 35168.8-45535.7 ns)
  35168.8 |####################
  35687.1 |####################
  36205.5 |####################
  36723.8 |####################
  37242.2 |####################
  37760.5 |
  38278.8 |
  38797.2 |####################
  39315.5 |####################
  39833.9 |####################
  40352.2 |########################################
  40870.6 |####################
  41388.9 |
  41907.3 |####################
  42425.6 |
  42944.0 |
  43462.3 |
  43980.6 |####################
  44499.0 |########################################
  45017.3 |
  (1 below, 2 above range)

sequential (n=18, range 7104.1-11765.2 ns)
   7104.1 |########################################
   7337.2 |#####
   7570.2 |#################
   7803.3 |
   8036.4 |
   8269.4 |
   8502.5 |
   8735.5 |
   8968.6 |
   9201.6 |
   9434.7 |
   9667.7 |
   9900.8 |
  10133.8 |
  10366.9 |
  10599.9 |
  10833.0 |#####
  11066.1 |
  11299.1 |
  11532.2 |#####
  (1 below, 4 above range)

std_parallel (n=18, range 48432.7-57681.0 ns)
  48432.7 |####################
  48895.1 |
  49357.5 |####################
  49819.9 |########################################
  50282.4 |####################
  50744.8 |####################
  51207.2 |########################################
  51669.6 |####################
  52132.0 |####################
  52594.4 |####################
  53056.8 |####################
  53519.2 |
  53981.7 |
  54444.1 |####################
  54906.5 |
  55368.9 |
  55831.3 |
  56293.7 |
  56756.1 |
  57218.6 |
  (1 below, 4 above range)

```

## Diagnostics

- **head_tail**: CV=26.9% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=96.0% of algo (FFI overhead may distort results)
- **sequential**: CV=24.2% (high variance, measurements may be unstable)
- **sequential**: bridge=104.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.7% of algo (FFI overhead may distort results)
