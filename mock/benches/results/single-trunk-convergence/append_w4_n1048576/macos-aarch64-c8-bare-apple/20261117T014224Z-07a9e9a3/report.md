# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 30% faster than the next best (std_parallel)

kernel_nway (68.87 us) leads std_parallel (89.67 us) by 30%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 38% (significant)

kernel_nway is -42.09 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 10.8x slower than the field

sequential (746.93 us) is 10.8x the fastest (68.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.8x the fastest

Fastest kernel_nway (68.87 us) to slowest sequential (746.93 us): 10.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 68870.9 ns median (-37.7% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 10.85x (fastest 68870.9 ns, slowest 746927.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 113345ns | 112406ns | 111617ns | 112377ns | 115662ns | base |
| head_tail_dyn | 112355ns | 112146ns | 110575ns | 112066ns | 113679ns | -0.87% |
| kernel_nway | 73364ns | 72181ns | 69138ns | 71709ns | 77960ns | -35.27% |
| kernel_seq | 113282ns | 112162ns | 111162ns | 111936ns | 116361ns | -0.06% |
| nway | 249324ns | 243581ns | 236874ns | 242338ns | 266027ns | +119.97% |
| sequential | 747479ns | 749058ns | 733676ns | 745924ns | 756712ns | +559.47% |
| std_parallel | 93076ns | 91706ns | 88376ns | 91628ns | 97597ns | -17.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 111218ns | 110222ns | 112803ns | base | 9.428 |
| head_tail_dyn | 110809ns | 110033ns | 111879ns | -0.37% | 9.463 |
| kernel_nway | 69486ns | 65882ns | 72688ns | -37.52% | 15.090 |
| kernel_seq | 111729ns | 109964ns | 114590ns | +0.46% | 9.385 |
| nway | 247019ns | 234728ns | 263460ns | +122.10% | 4.245 |
| sequential | 746121ns | 732558ns | 755515ns | +570.86% | 1.405 |
| std_parallel | 90592ns | 86524ns | 94147ns | -18.55% | 11.575 |

## Performance model

- Peak throughput: **15.916 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.479 | 59.6% |
| head_tail_dyn | 9.492 | 59.6% |
| kernel_nway | 15.225 | 95.7% |
| kernel_seq | 9.501 | 59.7% |
| nway | 4.344 | 27.3% |
| sequential | 1.404 | 8.8% |
| std_parallel | 11.693 | 73.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 113345ns | 113345ns | base |
| head_tail_dyn | 112355ns | 112355ns | -0.87% |
| kernel_nway | 73364ns | 73364ns | -35.27% |
| kernel_seq | 113282ns | 113282ns | -0.06% |
| nway | 249324ns | 249324ns | +119.97% |
| sequential | 747479ns | 747479ns | +559.47% |
| std_parallel | 93076ns | 93076ns | -17.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 110619ns | base | --- | [110346, 111108] | --- | --- | --- | --- |
| head_tail_dyn | 110471ns | no significant difference | [-633, +167]ns | [110177, 110835] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 68871ns | -42087.5ns (-38.0%) | [-43696, -40485]ns | [68267, 70417] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 110369ns | no significant difference | [-467, +454]ns | [110248, 111398] | no | 0.4807 | 0.4807 | 0 |
| nway | 241385ns | +131106.2ns (+118.5%) | [+127756, +132817]ns | [238440, 244200] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 746927ns | +635025.0ns (+574.1%) | [+629685, +640508]ns | [740948, 751256] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 89673ns | -20762.5ns (-18.8%) | [-22717, -19252]ns | [88981, 91775] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 110683ns | -0.1% | -38.2% | -0.3% | +114.8% | +576.8% | -17.2% |
| 1 | 2 | 110346ns | -0.2% | -38.1% | +0.7% | +119.4% | +571.4% | -19.2% |
| 1 | 3 | 110679ns | -0.1% | -36.9% | -0.3% | +131.8% | +574.9% | -19.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.667 | HIGH- (thermal bounce) |
| kernel_nway | -0.224 | moderate- |
| kernel_seq | -0.667 | HIGH- (thermal bounce) |
| nway | -0.058 | ok |
| sequential | -0.623 | HIGH- (thermal bounce) |
| std_parallel | -0.231 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5719404.4ns | 111218.3ns | 5142.5% | HIGH |
| head_tail_dyn | 5702020.1ns | 110809.5ns | 5145.8% | HIGH |
| kernel_nway | 5698090.7ns | 69486.1ns | 8200.3% | HIGH |
| kernel_seq | 5740645.8ns | 111728.7ns | 5138.0% | HIGH |
| nway | 5885381.5ns | 247019.4ns | 2382.6% | HIGH |
| sequential | 6341475.9ns | 746120.8ns | 849.9% | HIGH |
| std_parallel | 5694056.0ns | 90592.4ns | 6285.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 110222.2-112802.8 ns)
  110222.2 |########################################
  110351.3 |####################
  110480.3 |##########
  110609.3 |####################
  110738.3 |####################
  110867.4 |
  110996.4 |
  111125.4 |
  111254.5 |
  111383.5 |####################
  111512.5 |##########
  111641.5 |
  111770.6 |
  111899.6 |
  112028.6 |
  112157.6 |
  112286.7 |
  112415.7 |
  112544.7 |
  112673.8 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 110033.3-111879.2 ns)
  110033.3 |##########################
  110125.6 |########################################
  110217.9 |
  110310.2 |##########################
  110402.5 |
  110494.8 |##########################
  110587.1 |
  110679.4 |#############
  110771.7 |##########################
  110864.0 |
  110956.3 |
  111048.6 |
  111140.8 |
  111233.1 |
  111325.4 |#############
  111417.7 |
  111510.0 |
  111602.3 |
  111694.6 |
  111786.9 |
  (2 below, 3 above range)

kernel_nway (n=18, range 65881.9-72687.5 ns)
  65881.9 |##########
  66222.2 |##########
  66562.5 |
  66902.8 |##########
  67243.0 |
  67583.3 |
  67923.6 |##########
  68263.9 |########################################
  68604.2 |
  68944.4 |##########
  69284.7 |####################
  69625.0 |
  69965.3 |
  70305.6 |####################
  70645.8 |
  70986.1 |
  71326.4 |
  71666.7 |
  72006.9 |##########
  72347.2 |##########
  (1 below, 2 above range)

kernel_seq (n=18, range 109963.9-114589.6 ns)
  109963.9 |####################
  110195.2 |########################################
  110426.5 |
  110657.8 |######
  110889.0 |######
  111120.3 |######
  111351.6 |######
  111582.9 |
  111814.2 |######
  112045.5 |
  112276.7 |
  112508.0 |
  112739.3 |
  112970.6 |
  113201.9 |
  113433.1 |
  113664.4 |######
  113895.7 |
  114127.0 |
  114358.3 |######
  (1 below, 1 above range)

nway (n=18, range 234727.8-263460.4 ns)
  234727.8 |####################
  236164.4 |##########
  237601.1 |####################
  239037.7 |##########
  240474.3 |########################################
  241911.0 |##########
  243347.6 |####################
  244784.2 |
  246220.8 |
  247657.5 |
  249094.1 |
  250530.7 |
  251967.4 |
  253404.0 |
  254840.6 |##########
  256277.3 |
  257713.9 |##########
  259150.5 |
  260587.2 |
  262023.8 |
  (1 below, 2 above range)

sequential (n=18, range 732558.3-755514.6 ns)
  732558.3 |
  733706.1 |
  734854.0 |
  736001.8 |
  737149.6 |
  738297.4 |
  739445.2 |####################
  740593.0 |########################################
  741740.8 |
  742888.6 |
  744036.5 |##########
  745184.3 |
  746332.1 |##########
  747479.9 |####################
  748627.7 |####################
  749775.5 |
  750923.3 |
  752071.1 |##########
  753219.0 |##########
  754366.8 |
  (1 below, 3 above range)

std_parallel (n=18, range 86523.6-94146.5 ns)
  86523.6 |
  86904.7 |#############
  87285.9 |
  87667.0 |##########################
  88048.2 |
  88429.3 |#############
  88810.5 |
  89191.6 |########################################
  89572.8 |##########################
  89953.9 |#############
  90335.1 |
  90716.2 |
  91097.4 |
  91478.5 |##########################
  91859.7 |########################################
  92240.8 |
  92621.9 |
  93003.1 |
  93384.2 |
  93765.4 |#############
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=5148.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5154.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=8249.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5164.5% of algo (FFI overhead may distort results)
- **nway**: bridge=2422.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=847.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=6339.1% of algo (FFI overhead may distort results)
