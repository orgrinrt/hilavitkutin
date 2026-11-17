# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 44% faster than the next best (nway)

head_tail_dyn (33.88 us) leads nway (48.72 us) by 44%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 37% (significant)

head_tail_dyn is -20.14 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.56)

kernel_seq's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {head_tail_dyn} vs {nway, kernel_nway, kernel_seq, head_tail, sequential, std_parallel} (44% apart)

The field splits into a fast tier {head_tail_dyn} and a slow tier {nway, kernel_nway, kernel_seq, head_tail, sequential, std_parallel} with a 44% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: head_tail_dyn** at 33877.1 ns median (-36.9% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.73x (fastest 33877.1 ns, slowest 58604.2 ns)

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
| head_tail | 54221ns | 54229ns | 52244ns | 53808ns | 55829ns | base |
| head_tail_dyn | 34321ns | 34392ns | 33244ns | 34111ns | 35174ns | -36.70% |
| kernel_nway | 52053ns | 52410ns | 49999ns | 51831ns | 53412ns | -4.00% |
| kernel_seq | 52135ns | 52142ns | 52040ns | 52114ns | 52215ns | -3.85% |
| nway | 49508ns | 49275ns | 47354ns | 48851ns | 51570ns | -8.69% |
| sequential | 56893ns | 56440ns | 56122ns | 56352ns | 58090ns | +4.93% |
| std_parallel | 59534ns | 58952ns | 53603ns | 58240ns | 64442ns | +9.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 53699ns | 51746ns | 55292ns | base | 1.220 |
| head_tail_dyn | 33808ns | 32862ns | 34559ns | -37.04% | 1.938 |
| kernel_nway | 51530ns | 49397ns | 52944ns | -4.04% | 1.272 |
| kernel_seq | 51949ns | 51861ns | 52031ns | -3.26% | 1.262 |
| nway | 48941ns | 46765ns | 51003ns | -8.86% | 1.339 |
| sequential | 56698ns | 55950ns | 57859ns | +5.59% | 1.156 |
| std_parallel | 59168ns | 53274ns | 64065ns | +10.19% | 1.108 |

## Performance model

- Peak throughput: **1.994 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.220 | 61.2% |
| head_tail_dyn | 1.935 | 97.0% |
| kernel_nway | 1.264 | 63.4% |
| kernel_seq | 1.262 | 63.3% |
| nway | 1.345 | 67.5% |
| sequential | 1.165 | 58.4% |
| std_parallel | 1.118 | 56.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 54221ns | 54221ns | base |
| head_tail_dyn | 34321ns | 34321ns | -36.70% |
| kernel_nway | 52053ns | 52053ns | -4.00% |
| kernel_seq | 52135ns | 52135ns | -3.85% |
| nway | 49508ns | 49508ns | -8.69% |
| sequential | 56893ns | 56893ns | +4.93% |
| std_parallel | 59534ns | 59534ns | +9.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 53719ns | base | --- | [52817, 54533] | --- | --- | --- | --- |
| head_tail_dyn | 33877ns | -20135.4ns (-37.5%) | [-20769, -18554]ns | [33079, 34421] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 51862ns | -1850.0ns (-3.4%) | [-2931, -1163]ns | [50585, 52662] | YES | 0.0075 | 0.0075 | 0 |
| kernel_seq | 51944ns | -1777.1ns (-3.3%) | [-2654, -862]ns | [51892, 51988] | YES | 0.0016 | 0.0013 | 0 |
| nway | 48721ns | -4875.0ns (-9.1%) | [-5760, -3738]ns | [47669, 50004] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56256ns | +2825.0ns (+5.3%) | [+1738, +3660]ns | [56006, 56465] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 58604ns | +3991.7ns (+7.4%) | [+2367, +8871]ns | [56644, 61344] | YES | 0.0016 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 54042ns | -37.9% | -1.9% | -3.9% | -9.1% | +3.5% | +10.5% |
| 1 | 2 | 53312ns | -35.5% | -2.0% | -2.5% | -8.4% | +5.7% | +9.2% |
| 1 | 3 | 53262ns | -38.1% | -5.6% | -2.6% | -10.3% | +5.9% | +10.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.135 | ok |
| head_tail_dyn | -0.551 | HIGH- (thermal bounce) |
| kernel_nway | -0.040 | ok |
| kernel_seq | -0.556 | HIGH- (thermal bounce) |
| nway | -0.074 | ok |
| sequential | -0.123 | ok |
| std_parallel | -0.359 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 116227.3ns | 53698.6ns | 216.4% | HIGH |
| head_tail_dyn | 97733.1ns | 33808.3ns | 289.1% | HIGH |
| kernel_nway | 105897.7ns | 51529.6ns | 205.5% | HIGH |
| kernel_seq | 103943.3ns | 51948.6ns | 200.1% | HIGH |
| nway | 118907.0ns | 48940.5ns | 243.0% | HIGH |
| sequential | 113601.6ns | 56698.4ns | 200.4% | HIGH |
| std_parallel | 116833.3ns | 59168.3ns | 197.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 51745.9-55291.7 ns)
  51745.9 |
  51923.2 |
  52100.4 |#############
  52277.7 |#############
  52455.0 |
  52632.3 |#############
  52809.6 |#############
  52986.9 |##########################
  53164.2 |
  53341.5 |
  53518.8 |#############
  53696.1 |#############
  53873.3 |#############
  54050.6 |#############
  54227.9 |#############
  54405.2 |
  54582.5 |
  54759.8 |########################################
  54937.1 |
  55114.4 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 32862.5-34559.0 ns)
  32862.5 |#############
  32947.3 |########################################
  33032.2 |#############
  33117.0 |
  33201.8 |
  33286.6 |
  33371.5 |
  33456.3 |#############
  33541.1 |##########################
  33625.9 |
  33710.8 |
  33795.6 |
  33880.4 |
  33965.2 |
  34050.1 |#############
  34134.9 |
  34219.7 |
  34304.5 |#############
  34389.4 |########################################
  34474.2 |#############
  (1 below, 3 above range)

kernel_nway (n=18, range 49397.2-52943.7 ns)
  49397.2 |
  49574.6 |####################
  49751.9 |
  49929.2 |
  50106.5 |########################################
  50283.9 |
  50461.2 |####################
  50638.5 |########################################
  50815.8 |
  50993.2 |
  51170.5 |####################
  51347.8 |
  51525.1 |####################
  51702.5 |
  51879.8 |
  52057.1 |########################################
  52234.4 |########################################
  52411.8 |
  52589.1 |
  52766.4 |
  (1 below, 5 above range)

kernel_seq (n=18, range 51861.1-52031.2 ns)
  51861.1 |
  51869.6 |#############
  51878.1 |##########################
  51886.6 |#############
  51895.1 |########################################
  51903.6 |
  51912.1 |
  51920.7 |#############
  51929.2 |
  51937.7 |
  51946.2 |
  51954.7 |#############
  51963.2 |#############
  51971.7 |
  51980.2 |##########################
  51988.7 |##########################
  51997.2 |#############
  52005.7 |#############
  52014.2 |
  52022.7 |
  (1 below, 1 above range)

nway (n=18, range 46765.3-51002.8 ns)
  46765.3 |
  46977.2 |
  47189.0 |####################
  47400.9 |####################
  47612.8 |########################################
  47824.7 |########################################
  48036.5 |
  48248.4 |####################
  48460.3 |
  48672.2 |
  48884.0 |####################
  49095.9 |########################################
  49307.8 |
  49519.7 |
  49731.5 |
  49943.4 |####################
  50155.3 |
  50367.2 |####################
  50579.0 |####################
  50790.9 |
  (2 below, 3 above range)

sequential (n=18, range 55950.0-57859.0 ns)
  55950.0 |########################################
  56045.5 |
  56140.9 |########################
  56236.4 |################
  56331.8 |################
  56427.3 |########
  56522.7 |########
  56618.2 |
  56713.6 |########
  56809.1 |
  56904.5 |
  57000.0 |
  57095.4 |
  57190.9 |
  57286.3 |
  57381.8 |
  57477.2 |
  57572.7 |
  57668.1 |
  57763.6 |
  (1 below, 2 above range)

std_parallel (n=18, range 53273.6-64064.6 ns)
  53273.6 |
  53813.1 |
  54352.7 |########################################
  54892.2 |
  55431.8 |
  55971.3 |####################
  56510.9 |########################################
  57050.4 |
  57590.0 |########################################
  58129.5 |
  58669.1 |
  59208.6 |####################
  59748.2 |
  60287.7 |########################################
  60827.3 |####################
  61366.8 |####################
  61906.4 |
  62445.9 |####################
  62985.5 |####################
  63525.0 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=218.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=289.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=204.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=200.1% of algo (FFI overhead may distort results)
- **nway**: bridge=244.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=197.0% of algo (FFI overhead may distort results)
