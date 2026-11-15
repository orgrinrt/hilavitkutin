# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 122% faster than the next best (kernel_seq)

sequential (9.61 us) leads kernel_seq (21.37 us) by 122%, a clear separation rather than a photo finish. CV 16.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 67% (significant)

sequential is -19.66 us (67%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 17.8x slower than the field

std_parallel (170.69 us) is 17.8x the fastest (9.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.62)

sequential's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 17.8x the fastest

Fastest sequential (9.61 us) to slowest std_parallel (170.69 us): 17.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_seq is inconsistent: worst-20% is 1.8x its best-20%

kernel_seq's best 20% of batches run at 19.05 us but its worst 20% at 33.76 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 9610.7 ns median (-67.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 17.76x (fastest 9610.7 ns, slowest 170690.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 8 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 30012ns | 29773ns | 25983ns | 29141ns | 33333ns | base |
| head_tail_dyn | 30554ns | 29754ns | 26100ns | 29491ns | 34377ns | +1.81% |
| kernel_nway | 75781ns | 77264ns | 64205ns | 74251ns | 83863ns | +152.50% |
| kernel_seq | 25246ns | 21760ns | 19407ns | 21184ns | 34260ns | -15.88% |
| nway | 84287ns | 79514ns | 70354ns | 78387ns | 100104ns | +180.84% |
| sequential | 9426ns | 9911ns | 7287ns | 8914ns | 11265ns | -68.59% |
| std_parallel | 173523ns | 171224ns | 145199ns | 168361ns | 195428ns | +478.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 29563ns | 25620ns | 32818ns | base | 0.002 |
| head_tail_dyn | 30090ns | 25747ns | 33870ns | +1.78% | 0.002 |
| kernel_nway | 75234ns | 63645ns | 83320ns | +154.48% | 0.001 |
| kernel_seq | 24780ns | 19054ns | 33761ns | -16.18% | 0.003 |
| nway | 83887ns | 70010ns | 99638ns | +183.75% | 0.001 |
| sequential | 9138ns | 7065ns | 10916ns | -69.09% | 0.007 |
| std_parallel | 172839ns | 144787ns | 194470ns | +484.64% | 0.000 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.002 | 24.1% |
| head_tail_dyn | 0.002 | 24.1% |
| kernel_nway | 0.001 | 9.2% |
| kernel_seq | 0.003 | 33.1% |
| nway | 0.001 | 8.9% |
| sequential | 0.007 | 73.5% |
| std_parallel | 0.000 | 4.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30012ns | 30012ns | base |
| head_tail_dyn | 30554ns | 30554ns | +1.81% |
| kernel_nway | 75781ns | 75781ns | +152.50% |
| kernel_seq | 25246ns | 25246ns | -15.88% |
| nway | 84287ns | 84287ns | +180.84% |
| sequential | 9426ns | 9426ns | -68.59% |
| std_parallel | 173523ns | 173523ns | +478.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 29361ns | base | --- | [27857, 30724] | --- | --- | --- | --- |
| head_tail_dyn | 29343ns | no significant difference | [-1627, +2149]ns | [28257, 32334] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 76768ns | +47568.5ns (+162.0%) | [+41972, +50542]ns | [70734, 81000] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 21370ns | -7644.6ns (-26.0%) | [-9291, -6571]ns | [19359, 24123] | YES | 0.0016 | 0.0013 | 0 |
| nway | 79116ns | +49629.0ns (+169.0%) | [+42052, +67997]ns | [73182, 95525] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9611ns | -19659.0ns (-67.0%) | [-21596, -18768]ns | [7459, 10219] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 170691ns | +142448.7ns (+485.2%) | [+134195, +157694]ns | [162264, 191210] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 28724ns | +4.9% | +155.9% | -25.6% | +179.4% | -66.4% | +483.6% |
| 1 | 2 | 26460ns | +5.6% | +183.2% | -26.8% | +261.0% | -72.8% | +477.5% |
| 1 | 3 | 31439ns | +1.8% | +162.7% | -23.3% | +132.8% | -66.2% | +512.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.469 | moderate- |
| head_tail_dyn | -0.525 | HIGH- (thermal bounce) |
| kernel_nway | -0.090 | ok |
| kernel_seq | -0.446 | moderate- |
| nway | -0.603 | HIGH- (thermal bounce) |
| sequential | -0.616 | HIGH- (thermal bounce) |
| std_parallel | -0.411 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 29823.1ns | 29563.5ns | 100.9% | HIGH |
| head_tail_dyn | 30223.5ns | 30090.1ns | 100.4% | HIGH |
| kernel_nway | 76974.7ns | 75234.4ns | 102.3% | HIGH |
| kernel_seq | 28628.5ns | 24779.6ns | 115.5% | HIGH |
| nway | 84607.6ns | 83887.4ns | 100.9% | HIGH |
| sequential | 9621.9ns | 9137.8ns | 105.3% | HIGH |
| std_parallel | 181706.4ns | 172838.6ns | 105.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 25620.2-32817.8 ns)
  25620.2 |
  25980.1 |#############
  26340.0 |
  26699.8 |#############
  27059.7 |#############
  27419.6 |
  27779.5 |
  28139.3 |
  28499.2 |########################################
  28859.1 |
  29219.0 |##########################
  29578.9 |#############
  29938.7 |#############
  30298.6 |#############
  30658.5 |
  31018.4 |##########################
  31378.2 |#############
  31738.1 |
  32098.0 |
  32457.9 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 25747.5-33870.2 ns)
  25747.5 |####################
  26153.6 |
  26559.7 |####################
  26965.9 |
  27372.0 |####################
  27778.2 |####################
  28184.3 |########################################
  28590.4 |########################################
  28996.6 |
  29402.7 |
  29808.8 |########################################
  30215.0 |
  30621.1 |
  31027.3 |####################
  31433.4 |####################
  31839.5 |
  32245.7 |
  32651.8 |####################
  33057.9 |####################
  33464.1 |####################
  (1 below, 2 above range)

kernel_nway (n=18, range 63644.9-83320.3 ns)
  63644.9 |####################
  64628.7 |####################
  65612.5 |
  66596.2 |
  67580.0 |####################
  68563.8 |
  69547.5 |####################
  70531.3 |####################
  71515.1 |########################################
  72498.8 |
  73482.6 |
  74466.4 |
  75450.1 |####################
  76433.9 |
  77417.7 |########################################
  78401.4 |####################
  79385.2 |
  80369.0 |####################
  81352.7 |####################
  82336.5 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 19053.6-33760.8 ns)
  19053.6 |########################################
  19788.9 |
  20524.3 |#####
  21259.7 |
  21995.0 |###########
  22730.4 |
  23465.7 |#################
  24201.1 |
  24936.5 |#####
  25671.8 |
  26407.2 |
  27142.5 |#####
  27877.9 |
  28613.3 |
  29348.6 |
  30084.0 |
  30819.4 |
  31554.7 |
  32290.1 |
  33025.4 |
  (1 below, 2 above range)

nway (n=18, range 70010.5-99637.9 ns)
  70010.5 |########################################
  71491.9 |########################################
  72973.2 |########################################
  74454.6 |####################
  75936.0 |####################
  77417.3 |
  78898.7 |
  80380.1 |####################
  81861.4 |####################
  83342.8 |
  84824.2 |
  86305.5 |
  87786.9 |
  89268.3 |
  90749.6 |
  92231.0 |####################
  93712.4 |
  95193.7 |########################################
  96675.1 |
  98156.5 |########################################
  (1 below, 2 above range)

sequential (n=18, range 7064.5-10916.1 ns)
   7064.5 |##########################
   7257.1 |
   7449.7 |##########################
   7642.3 |#############
   7834.9 |
   8027.4 |
   8220.0 |
   8412.6 |
   8605.2 |
   8797.8 |
   8990.3 |
   9182.9 |#############
   9375.5 |
   9568.1 |########################################
   9760.7 |#############
   9953.2 |
  10145.8 |#############
  10338.4 |
  10531.0 |##########################
  10723.6 |#############
  (2 below, 2 above range)

std_parallel (n=18, range 144786.8-194470.1 ns)
  144786.8 |
  147271.0 |#############
  149755.2 |
  152239.3 |
  154723.5 |#############
  157207.7 |
  159691.8 |#############
  162176.0 |#############
  164660.1 |##########################
  167144.3 |
  169628.5 |##########################
  172112.6 |#############
  174596.8 |
  177081.0 |
  179565.1 |
  182049.3 |#############
  184533.4 |
  187017.6 |
  189501.8 |#############
  191985.9 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=98.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=98.6% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=38.5% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=116.7% of algo (FFI overhead may distort results)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=95.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=110.3% of algo (FFI overhead may distort results)
