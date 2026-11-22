# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 41% faster than the next best (kernel_nway)

head_tail_dyn (33.87 us) leads kernel_nway (47.79 us) by 41%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 37% (significant)

head_tail_dyn is -19.96 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.1x slower than the field

std_parallel (71.35 us) is 2.1x the fastest (33.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.63)

head_tail's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn} vs {kernel_nway, kernel_seq, head_tail, sequential, nway, std_parallel} (41% apart)

The field splits into a fast tier {head_tail_dyn} and a slow tier {kernel_nway, kernel_seq, head_tail, sequential, nway, std_parallel} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: head_tail_dyn** at 33866.7 ns median (-37.5% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.11x (fastest 33866.7 ns, slowest 71350.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
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
| head_tail | 54592ns | 54665ns | 52228ns | 54266ns | 56264ns | base |
| head_tail_dyn | 34363ns | 34323ns | 33438ns | 34130ns | 35175ns | -37.06% |
| kernel_nway | 48845ns | 48412ns | 45392ns | 47909ns | 51976ns | -10.53% |
| kernel_seq | 52241ns | 52198ns | 52100ns | 52177ns | 52407ns | -4.31% |
| nway | 59663ns | 59790ns | 55586ns | 59079ns | 62578ns | +9.29% |
| sequential | 57010ns | 56900ns | 55771ns | 56600ns | 58245ns | +4.43% |
| std_parallel | 73093ns | 71746ns | 68800ns | 71642ns | 77417ns | +33.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 54083ns | 51712ns | 55719ns | base | 1.212 |
| head_tail_dyn | 33883ns | 33015ns | 34689ns | -37.35% | 1.934 |
| kernel_nway | 48178ns | 44614ns | 51324ns | -10.92% | 1.360 |
| kernel_seq | 52052ns | 51926ns | 52197ns | -3.76% | 1.259 |
| nway | 59079ns | 55010ns | 61999ns | +9.24% | 1.109 |
| sequential | 56828ns | 55596ns | 58051ns | +5.07% | 1.153 |
| std_parallel | 72691ns | 68433ns | 76999ns | +34.41% | 0.902 |

## Performance model

- Peak throughput: **1.985 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.210 | 61.0% |
| head_tail_dyn | 1.935 | 97.5% |
| kernel_nway | 1.371 | 69.1% |
| kernel_seq | 1.260 | 63.5% |
| nway | 1.106 | 55.7% |
| sequential | 1.155 | 58.2% |
| std_parallel | 0.919 | 46.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 54592ns | 54592ns | base |
| head_tail_dyn | 34363ns | 34363ns | -37.06% |
| kernel_nway | 48845ns | 48845ns | -10.53% |
| kernel_seq | 52241ns | 52241ns | -4.31% |
| nway | 59663ns | 59663ns | +9.29% |
| sequential | 57010ns | 57010ns | +4.43% |
| std_parallel | 73093ns | 73093ns | +33.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 54165ns | base | --- | [53323, 54869] | --- | --- | --- | --- |
| head_tail_dyn | 33867ns | -19956.3ns (-36.8%) | [-21379, -19435]ns | [33400, 34077] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 47788ns | -6533.3ns (-12.1%) | [-7985, -4113]ns | [46565, 48973] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 52023ns | -2035.4ns (-3.8%) | [-2879, -1354]ns | [51965, 52100] | YES | 0.0002 | 0.0001 | 0 |
| nway | 59231ns | +4239.6ns (+7.8%) | [+2721, +6933]ns | [56877, 61296] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56731ns | +2733.3ns (+5.0%) | [+1733, +3896]ns | [55823, 57244] | YES | 0.0013 | 0.0013 | 0 |
| std_parallel | 71350ns | +16989.6ns (+31.4%) | [+16138, +19404]ns | [70348, 73806] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 53956ns | -36.5% | -5.8% | -3.6% | +13.9% | +5.3% | +36.3% |
| 1 | 2 | 55498ns | -39.3% | -16.4% | -6.3% | +7.3% | +0.2% | +29.6% |
| 1 | 3 | 53496ns | -37.0% | -11.5% | -2.8% | +4.2% | +7.4% | +32.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.635 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.178 | ok |
| kernel_nway | -0.298 | moderate- |
| kernel_seq | -0.381 | moderate- |
| nway | -0.024 | ok |
| sequential | -0.593 | HIGH- (thermal bounce) |
| std_parallel | -0.013 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 113130.8ns | 54083.3ns | 209.2% | HIGH |
| head_tail_dyn | 97684.0ns | 33882.9ns | 288.3% | HIGH |
| kernel_nway | 114886.8ns | 48178.0ns | 238.5% | HIGH |
| kernel_seq | 104187.3ns | 52051.8ns | 200.2% | HIGH |
| nway | 117153.5ns | 59078.9ns | 198.3% | HIGH |
| sequential | 113861.3ns | 56827.6ns | 200.4% | HIGH |
| std_parallel | 139072.2ns | 72691.4ns | 191.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 51712.5-55719.4 ns)
  51712.5 |
  51912.8 |##########
  52113.2 |
  52313.5 |
  52513.9 |
  52714.2 |
  52914.6 |##########
  53114.9 |####################
  53315.3 |##########
  53515.6 |##########
  53716.0 |##########
  53916.3 |
  54116.7 |########################################
  54317.0 |
  54517.4 |
  54717.7 |##########
  54918.0 |##########
  55118.4 |##########
  55318.7 |
  55519.1 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 33015.3-34688.9 ns)
  33015.3 |
  33098.9 |
  33182.6 |
  33266.3 |########################################
  33350.0 |
  33433.7 |##########################
  33517.3 |#############
  33601.0 |
  33684.7 |
  33768.4 |#############
  33852.1 |########################################
  33935.7 |#############
  34019.4 |
  34103.1 |
  34186.8 |########################################
  34270.5 |
  34354.1 |
  34437.8 |
  34521.5 |
  34605.2 |
  (2 below, 2 above range)

kernel_nway (n=18, range 44613.9-51324.3 ns)
  44613.9 |
  44949.4 |
  45284.9 |#############
  45620.4 |
  45956.0 |#############
  46291.5 |#############
  46627.0 |#############
  46962.5 |#############
  47298.0 |#############
  47633.6 |########################################
  47969.1 |#############
  48304.6 |#############
  48640.1 |
  48975.6 |
  49311.2 |#############
  49646.7 |
  49982.2 |#############
  50317.7 |
  50653.3 |
  50988.8 |
  (2 below, 3 above range)

kernel_seq (n=18, range 51926.4-52197.2 ns)
  51926.4 |#############
  51939.9 |##########################
  51953.5 |##########################
  51967.0 |#############
  51980.6 |#############
  51994.1 |
  52007.6 |#############
  52021.2 |
  52034.7 |########################################
  52048.3 |
  52061.8 |
  52075.3 |#############
  52088.9 |
  52102.4 |#############
  52116.0 |
  52129.5 |
  52143.1 |
  52156.6 |#############
  52170.1 |
  52183.7 |
  (1 below, 3 above range)

nway (n=18, range 55009.7-61999.3 ns)
  55009.7 |####################
  55359.2 |
  55708.7 |
  56058.2 |########################################
  56407.6 |
  56757.1 |
  57106.6 |
  57456.1 |####################
  57805.6 |
  58155.0 |
  58504.5 |####################
  58854.0 |########################################
  59203.5 |####################
  59552.9 |####################
  59902.4 |
  60251.9 |
  60601.4 |####################
  60950.9 |####################
  61300.3 |####################
  61649.8 |####################
  (2 below, 3 above range)

sequential (n=18, range 55595.8-58051.4 ns)
  55595.8 |########################################
  55718.6 |
  55841.4 |
  55964.2 |#############
  56086.9 |
  56209.7 |
  56332.5 |#############
  56455.3 |
  56578.1 |##########################
  56700.8 |
  56823.6 |#############
  56946.4 |##########################
  57069.2 |#############
  57191.9 |
  57314.7 |#############
  57437.5 |#############
  57560.3 |
  57683.1 |
  57805.8 |
  57928.6 |
  (2 below, 3 above range)

std_parallel (n=18, range 68433.3-76999.3 ns)
  68433.3 |
  68861.6 |
  69289.9 |
  69718.2 |##########################
  70146.5 |##########################
  70574.8 |#############
  71003.1 |########################################
  71431.4 |
  71859.7 |
  72288.0 |#############
  72716.3 |
  73144.6 |#############
  73572.9 |##########################
  74001.2 |#############
  74429.5 |
  74857.8 |
  75286.1 |#############
  75714.4 |
  76142.7 |
  76571.0 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=208.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=288.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=240.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=200.3% of algo (FFI overhead may distort results)
- **nway**: bridge=200.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=201.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=196.7% of algo (FFI overhead may distort results)
