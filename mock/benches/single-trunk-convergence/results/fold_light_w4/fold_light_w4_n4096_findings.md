# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 183% faster than the next best (kernel_seq)

sequential (8.80 us) leads kernel_seq (24.91 us) by 183%, a clear separation rather than a photo finish. CV 16.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 71% (significant)

sequential is -23.29 us (71%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 9.7x slower than the field

std_parallel (85.68 us) is 9.7x the fastest (8.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} (183% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, head_tail, kernel_nway, nway, std_parallel} with a 183% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.7x the fastest

Fastest sequential (8.80 us) to slowest std_parallel (85.68 us): 9.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 2.0x its best-20%

kernel_nway's best 20% of batches run at 35.97 us but its worst 20% at 72.49 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 8796.5 ns median (-73.1% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 9.74x (fastest 8796.5 ns, slowest 85676.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 33264ns | 33178ns | 28218ns | 32515ns | 36912ns | base |
| head_tail_dyn | 32731ns | 31938ns | 27935ns | 31063ns | 37630ns | -1.60% |
| kernel_nway | 51076ns | 39953ns | 36348ns | 40973ns | 73594ns | +53.55% |
| kernel_seq | 26757ns | 25336ns | 23284ns | 24657ns | 31645ns | -19.56% |
| nway | 60446ns | 60358ns | 48402ns | 57734ns | 70535ns | +81.71% |
| sequential | 9424ns | 9048ns | 7690ns | 8846ns | 11157ns | -71.67% |
| std_parallel | 86597ns | 86381ns | 79055ns | 85637ns | 91807ns | +160.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 32703ns | 27812ns | 36205ns | base | 0.125 |
| head_tail_dyn | 32185ns | 27369ns | 37009ns | -1.58% | 0.127 |
| kernel_nway | 50373ns | 35969ns | 72489ns | +54.03% | 0.081 |
| kernel_seq | 26153ns | 22544ns | 30888ns | -20.03% | 0.157 |
| nway | 59996ns | 47989ns | 70017ns | +83.46% | 0.068 |
| sequential | 9122ns | 7446ns | 10789ns | -72.11% | 0.449 |
| std_parallel | 86079ns | 78698ns | 91298ns | +163.21% | 0.048 |

## Performance model

- Peak throughput: **0.550 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.125 | 22.7% |
| head_tail_dyn | 0.130 | 23.6% |
| kernel_nway | 0.104 | 18.8% |
| kernel_seq | 0.164 | 29.9% |
| nway | 0.068 | 12.4% |
| sequential | 0.466 | 84.6% |
| std_parallel | 0.048 | 8.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 33264ns | 33264ns | base |
| head_tail_dyn | 32731ns | 32731ns | -1.60% |
| kernel_nway | 51076ns | 51076ns | +53.55% |
| kernel_seq | 26757ns | 26757ns | -19.56% |
| nway | 60446ns | 60446ns | +81.71% |
| sequential | 9424ns | 9424ns | -71.67% |
| std_parallel | 86597ns | 86597ns | +160.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 32756ns | base | --- | [31160, 33634] | --- | --- | --- | --- |
| head_tail_dyn | 31491ns | no significant difference | [-5082, +2155]ns | [29406, 32723] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 39558ns | +8705.9ns (+26.6%) | [+4528, +22441]ns | [37885, 55734] | YES | 0.0016 | 0.0013 | 0 |
| kernel_seq | 24910ns | -7742.1ns (-23.6%) | [-10610, -3998]ns | [23399, 26478] | YES | 0.0016 | 0.0013 | 0 |
| nway | 59969ns | +27690.0ns (+84.5%) | [+18747, +33483]ns | [53306, 65799] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 8796ns | -23287.8ns (-71.1%) | [-24266, -22412]ns | [8180, 9752] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 85676ns | +52531.6ns (+160.4%) | [+49953, +58697]ns | [83538, 89915] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30707ns | +2.6% | +91.5% | -13.8% | +129.7% | -72.8% | +197.6% |
| 1 | 2 | 33275ns | -0.8% | +15.4% | -27.2% | +73.0% | -77.4% | +152.9% |
| 1 | 3 | 33482ns | -16.3% | +14.7% | -30.9% | +58.2% | -70.2% | +152.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.130 | ok |
| head_tail_dyn | -0.358 | moderate- |
| kernel_nway | -0.166 | ok |
| kernel_seq | -0.027 | ok |
| nway | -0.047 | ok |
| sequential | -0.382 | moderate- |
| std_parallel | -0.199 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 35318.9ns | 32703.1ns | 108.0% | HIGH |
| head_tail_dyn | 32508.2ns | 32184.8ns | 101.0% | HIGH |
| kernel_nway | 47397.0ns | 50373.0ns | 94.1% | HIGH |
| kernel_seq | 25546.3ns | 26153.4ns | 97.7% | HIGH |
| nway | 60062.6ns | 59995.5ns | 100.1% | HIGH |
| sequential | 10621.4ns | 9121.9ns | 116.4% | HIGH |
| std_parallel | 86002.6ns | 86079.0ns | 99.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 27811.9-36205.0 ns)
  27811.9 |
  28231.6 |
  28651.2 |#############
  29070.9 |#############
  29490.5 |#############
  29910.2 |
  30329.8 |#############
  30749.5 |
  31169.1 |
  31588.8 |#############
  32008.4 |##########################
  32428.1 |#############
  32847.7 |########################################
  33267.4 |#############
  33687.0 |#############
  34106.7 |
  34526.3 |#############
  34946.0 |
  35365.6 |
  35785.3 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 27369.4-37009.0 ns)
  27369.4 |########################################
  27851.4 |
  28333.4 |####################
  28815.3 |
  29297.3 |########################################
  29779.3 |
  30261.3 |####################
  30743.3 |####################
  31225.2 |########################################
  31707.2 |########################################
  32189.2 |####################
  32671.2 |########################################
  33153.1 |
  33635.1 |
  34117.1 |
  34599.1 |
  35081.1 |####################
  35563.0 |
  36045.0 |
  36527.0 |
  (1 below, 2 above range)

kernel_nway (n=18, range 35968.7-72488.8 ns)
  35968.7 |##############################
  37794.7 |########################################
  39620.7 |####################
  41446.7 |
  43272.7 |
  45098.7 |
  46924.7 |
  48750.7 |
  50576.7 |
  52402.7 |##########
  54228.8 |##########
  56054.8 |##########
  57880.8 |####################
  59706.8 |##########
  61532.8 |
  63358.8 |
  65184.8 |
  67010.8 |
  68836.8 |
  70662.8 |
  (2 below, 1 above range)

kernel_seq (n=18, range 22543.6-30888.3 ns)
  22543.6 |
  22960.8 |##########################
  23378.0 |########################################
  23795.3 |#############
  24212.5 |
  24629.8 |##########################
  25047.0 |##########################
  25464.2 |
  25881.5 |#############
  26298.7 |#############
  26715.9 |##########################
  27133.2 |
  27550.4 |
  27967.7 |
  28384.9 |
  28802.1 |
  29219.4 |
  29636.6 |
  30053.8 |
  30471.1 |
  (2 below, 2 above range)

nway (n=18, range 47989.0-70017.4 ns)
  47989.0 |####################
  49090.4 |
  50191.8 |####################
  51293.3 |####################
  52394.7 |
  53496.1 |
  54597.5 |####################
  55698.9 |####################
  56800.3 |
  57901.8 |####################
  59003.2 |####################
  60104.6 |########################################
  61206.0 |####################
  62307.4 |
  63408.9 |
  64510.3 |
  65611.7 |####################
  66713.1 |####################
  67814.5 |####################
  68915.9 |
  (2 below, 3 above range)

sequential (n=18, range 7445.8-10789.4 ns)
   7445.8 |########################################
   7613.0 |
   7780.2 |
   7947.4 |####################
   8114.5 |########################################
   8281.7 |########################################
   8448.9 |
   8616.1 |
   8783.2 |
   8950.4 |
   9117.6 |####################
   9284.8 |####################
   9452.0 |########################################
   9619.1 |
   9786.3 |####################
   9953.5 |####################
  10120.7 |
  10287.8 |
  10455.0 |
  10622.2 |
  (2 below, 3 above range)

std_parallel (n=18, range 78697.8-91298.3 ns)
  78697.8 |
  79327.8 |
  79957.8 |
  80587.9 |
  81217.9 |########################################
  81847.9 |
  82477.9 |####################
  83108.0 |
  83738.0 |####################
  84368.0 |########################################
  84998.0 |####################
  85628.1 |####################
  86258.1 |
  86888.1 |
  87518.1 |####################
  88148.1 |
  88778.2 |####################
  89408.2 |########################################
  90038.2 |
  90668.2 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=102.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.2% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=49.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.5% of algo (FFI overhead may distort results)
- **nway**: bridge=100.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.7% of algo (FFI overhead may distort results)
