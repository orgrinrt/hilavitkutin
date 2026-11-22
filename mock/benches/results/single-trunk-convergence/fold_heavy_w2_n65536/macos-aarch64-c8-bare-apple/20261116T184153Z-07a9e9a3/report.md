# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 50% faster than the next best (kernel_nway)

head_tail_dyn (34.45 us) leads kernel_nway (51.55 us) by 50%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 36% (significant)

head_tail_dyn is -19.13 us (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn} vs {kernel_nway, nway, kernel_seq, head_tail, sequential, std_parallel} (50% apart)

The field splits into a fast tier {head_tail_dyn} and a slow tier {kernel_nway, nway, kernel_seq, head_tail, sequential, std_parallel} with a 50% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: head_tail_dyn** at 34447.9 ns median (-35.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.82x (fastest 34447.9 ns, slowest 62700.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 2 |
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
| head_tail | 54060ns | 54185ns | 51896ns | 53947ns | 55310ns | base |
| head_tail_dyn | 34703ns | 34902ns | 33264ns | 34633ns | 35526ns | -35.81% |
| kernel_nway | 50458ns | 52085ns | 41718ns | 51083ns | 53890ns | -6.66% |
| kernel_seq | 53391ns | 53267ns | 52336ns | 53011ns | 54488ns | -1.24% |
| nway | 52793ns | 53152ns | 48349ns | 52038ns | 56149ns | -2.34% |
| sequential | 57825ns | 57569ns | 57076ns | 57482ns | 58712ns | +6.96% |
| std_parallel | 63342ns | 63085ns | 59683ns | 62363ns | 66640ns | +17.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 53553ns | 51350ns | 54796ns | base | 1.224 |
| head_tail_dyn | 34235ns | 32844ns | 35026ns | -36.07% | 1.914 |
| kernel_nway | 49929ns | 41242ns | 53391ns | -6.77% | 1.313 |
| kernel_seq | 53136ns | 52149ns | 54148ns | -0.78% | 1.233 |
| nway | 52196ns | 47774ns | 55540ns | -2.53% | 1.256 |
| sequential | 57635ns | 56867ns | 58515ns | +7.62% | 1.137 |
| std_parallel | 62956ns | 59324ns | 66230ns | +17.56% | 1.041 |

## Performance model

- Peak throughput: **1.995 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.221 | 61.2% |
| head_tail_dyn | 1.902 | 95.3% |
| kernel_nway | 1.271 | 63.7% |
| kernel_seq | 1.236 | 61.9% |
| nway | 1.247 | 62.5% |
| sequential | 1.142 | 57.2% |
| std_parallel | 1.045 | 52.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 54060ns | 54060ns | base |
| head_tail_dyn | 34703ns | 34703ns | -35.81% |
| kernel_nway | 50458ns | 50458ns | -6.66% |
| kernel_seq | 53391ns | 53391ns | -1.24% |
| nway | 52793ns | 52793ns | -2.34% |
| sequential | 57825ns | 57825ns | +6.96% |
| std_parallel | 63342ns | 63342ns | +17.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 53690ns | base | --- | [53044, 54100] | --- | --- | --- | --- |
| head_tail_dyn | 34448ns | -19127.1ns (-35.6%) | [-20033, -18475]ns | [33825, 34715] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 51554ns | -2827.1ns (-5.3%) | [-4533, -1310]ns | [48704, 52925] | YES | 0.0463 | 0.0309 | 0 |
| kernel_seq | 53040ns | no significant difference | [-1565, +350]ns | [52540, 53496] | no | 0.2855 | 0.2379 | 0 |
| nway | 52573ns | no significant difference | [-2990, +1033]ns | [49831, 53942] | no | 0.4807 | 0.4807 | 0 |
| sequential | 57394ns | +3772.9ns (+7.0%) | [+3162, +4748]ns | [57140, 58008] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 62700ns | +9095.8ns (+16.9%) | [+7929, +10765]ns | [61065, 64158] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 52938ns | -35.8% | -0.7% | -0.6% | -0.0% | +8.7% | +18.6% |
| 1 | 2 | 53781ns | -36.4% | -4.0% | -1.4% | -5.0% | +7.2% | +15.8% |
| 1 | 3 | 53894ns | -35.5% | -6.9% | -0.7% | -2.7% | +6.3% | +16.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.109 | ok |
| head_tail_dyn | -0.042 | ok |
| kernel_nway | -0.009 | ok |
| kernel_seq | -0.001 | ok |
| nway | -0.627 | HIGH- (thermal bounce) |
| sequential | -0.424 | moderate- |
| std_parallel | -0.667 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 2/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 111375.9ns | 53553.0ns | 208.0% | HIGH |
| head_tail_dyn | 98603.5ns | 34235.4ns | 288.0% | HIGH |
| kernel_nway | 108659.7ns | 49929.2ns | 217.6% | HIGH |
| kernel_seq | 106280.8ns | 53136.1ns | 200.0% | HIGH |
| nway | 118347.2ns | 52196.1ns | 226.7% | HIGH |
| sequential | 115715.3ns | 57635.0ns | 200.8% | HIGH |
| std_parallel | 122247.2ns | 62955.6ns | 194.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 51350.0-54795.8 ns)
  51350.0 |
  51522.3 |
  51694.6 |
  51866.9 |
  52039.2 |
  52211.5 |
  52383.8 |
  52556.0 |
  52728.3 |##########
  52900.6 |####################
  53072.9 |##########
  53245.2 |
  53417.5 |##########
  53589.8 |########################################
  53762.1 |##########
  53934.4 |##########
  54106.7 |##########
  54279.0 |##########
  54451.2 |
  54623.5 |##########
  (2 below, 2 above range)

head_tail_dyn (n=18, range 32844.4-35025.7 ns)
  32844.4 |
  32953.5 |
  33062.6 |
  33171.6 |
  33280.7 |#############
  33389.8 |
  33498.8 |
  33607.9 |#############
  33716.9 |##########################
  33826.0 |
  33935.1 |#############
  34044.1 |
  34153.2 |#############
  34262.3 |#############
  34371.3 |
  34480.4 |##########################
  34589.4 |#############
  34698.5 |########################################
  34807.6 |
  34916.6 |#############
  (2 below, 2 above range)

kernel_nway (n=18, range 41241.7-53391.0 ns)
  41241.7 |
  41849.1 |#############
  42456.6 |
  43064.1 |
  43671.5 |
  44279.0 |
  44886.5 |
  45493.9 |
  46101.4 |
  46708.9 |
  47316.3 |#############
  47923.8 |##########################
  48531.3 |
  49138.7 |#############
  49746.2 |
  50353.7 |
  50961.1 |########################################
  51568.6 |########################################
  52176.1 |
  52783.5 |##########################
  (1 below, 4 above range)

kernel_seq (n=18, range 52148.6-54147.9 ns)
  52148.6 |####################
  52248.6 |####################
  52348.6 |
  52448.5 |########################################
  52548.5 |
  52648.5 |########################################
  52748.4 |
  52848.4 |
  52948.3 |########################################
  53048.3 |########################################
  53148.3 |
  53248.2 |
  53348.2 |
  53448.2 |########################################
  53548.1 |
  53648.1 |
  53748.1 |
  53848.0 |
  53948.0 |####################
  54048.0 |
  (2 below, 3 above range)

nway (n=18, range 47773.6-55539.6 ns)
  47773.6 |####################
  48161.9 |
  48550.2 |####################
  48938.5 |
  49326.8 |####################
  49715.1 |########################################
  50103.4 |
  50491.7 |####################
  50880.0 |
  51268.3 |
  51656.6 |
  52044.9 |####################
  52433.2 |########################################
  52821.5 |########################################
  53209.8 |
  53598.1 |####################
  53986.4 |
  54374.7 |########################################
  54763.0 |
  55151.3 |
  (1 below, 3 above range)

sequential (n=18, range 56866.7-58514.6 ns)
  56866.7 |
  56949.1 |########################################
  57031.5 |####################
  57113.9 |####################
  57196.2 |####################
  57278.6 |########################################
  57361.0 |####################
  57443.4 |####################
  57525.8 |####################
  57608.2 |
  57690.6 |
  57773.0 |
  57855.4 |
  57937.8 |####################
  58020.2 |
  58102.6 |
  58185.0 |
  58267.4 |####################
  58349.8 |####################
  58432.2 |
  (2 below, 3 above range)

std_parallel (n=18, range 59323.6-66229.8 ns)
  59323.6 |####################
  59668.9 |
  60014.2 |####################
  60359.5 |####################
  60704.8 |####################
  61050.2 |####################
  61395.5 |
  61740.8 |####################
  62086.1 |####################
  62431.4 |########################################
  62776.7 |####################
  63122.0 |########################################
  63467.3 |
  63812.7 |
  64158.0 |
  64503.3 |
  64848.6 |####################
  65193.9 |####################
  65539.2 |
  65884.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=208.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=285.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=209.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=199.3% of algo (FFI overhead may distort results)
- **nway**: bridge=222.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=193.6% of algo (FFI overhead may distort results)
