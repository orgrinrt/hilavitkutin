# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 45% faster than the next best (kernel_nway)

head_tail_dyn (34.12 us) leads kernel_nway (49.36 us) by 45%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 37% (significant)

head_tail_dyn is -19.55 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.7x slower than the field

std_parallel (92.69 us) is 2.7x the fastest (34.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.58)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: head_tail_dyn** at 34116.7 ns median (-36.1% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.72x (fastest 34116.7 ns, slowest 92689.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
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
| head_tail | 51277ns | 53971ns | 39968ns | 51909ns | 55983ns | base |
| head_tail_dyn | 34681ns | 34638ns | 33488ns | 34385ns | 35721ns | -32.37% |
| kernel_nway | 49900ns | 49933ns | 46046ns | 49442ns | 52513ns | -2.69% |
| kernel_seq | 52815ns | 52410ns | 52049ns | 52346ns | 53901ns | +3.00% |
| nway | 69447ns | 69173ns | 64294ns | 68241ns | 73831ns | +35.43% |
| sequential | 56964ns | 56610ns | 55808ns | 56399ns | 58390ns | +11.09% |
| std_parallel | 93399ns | 93131ns | 88365ns | 92373ns | 97456ns | +82.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 50705ns | 39300ns | 55446ns | base | 1.292 |
| head_tail_dyn | 34182ns | 33074ns | 35154ns | -32.59% | 1.917 |
| kernel_nway | 49268ns | 45407ns | 51858ns | -2.83% | 1.330 |
| kernel_seq | 52599ns | 51861ns | 53631ns | +3.73% | 1.246 |
| nway | 68867ns | 63712ns | 73273ns | +35.82% | 0.952 |
| sequential | 56781ns | 55639ns | 58200ns | +11.98% | 1.154 |
| std_parallel | 92989ns | 88004ns | 97039ns | +83.39% | 0.705 |

## Performance model

- Peak throughput: **1.982 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.227 | 61.9% |
| head_tail_dyn | 1.921 | 96.9% |
| kernel_nway | 1.328 | 67.0% |
| kernel_seq | 1.255 | 63.3% |
| nway | 0.955 | 48.2% |
| sequential | 1.161 | 58.6% |
| std_parallel | 0.707 | 35.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 51277ns | 51277ns | base |
| head_tail_dyn | 34681ns | 34681ns | -32.37% |
| kernel_nway | 49900ns | 49900ns | -2.69% |
| kernel_seq | 52815ns | 52815ns | +3.00% |
| nway | 69447ns | 69447ns | +35.43% |
| sequential | 56964ns | 56964ns | +11.09% |
| std_parallel | 93399ns | 93399ns | +82.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 53429ns | base | --- | [49579, 54681] | --- | --- | --- | --- |
| head_tail_dyn | 34117ns | -19547.9ns (-36.6%) | [-20210, -16127]ns | [33625, 34594] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 49360ns | no significant difference | [-6415, +2804]ns | [48221, 50498] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 52210ns | no significant difference | [-2723, +2900]ns | [52048, 52604] | no | 0.2379 | 0.2379 | 0 |
| nway | 68590ns | +14714.6ns (+27.5%) | [+12481, +21492]ns | [65729, 71615] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56433ns | +3714.6ns (+7.0%) | [+2188, +6971]ns | [55877, 57098] | YES | 0.0002 | 0.0001 | 0 |
| std_parallel | 92690ns | +42431.2ns (+79.4%) | [+37717, +45533]ns | [90779, 95435] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 41681ns | -18.1% | +20.8% | +29.4% | +74.3% | +33.6% | +123.2% |
| 1 | 2 | 52962ns | -35.8% | -9.0% | -1.7% | +22.7% | +6.6% | +81.1% |
| 1 | 3 | 54908ns | -37.6% | -10.8% | -5.3% | +24.9% | +5.4% | +64.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.095 | ok |
| head_tail_dyn | -0.580 | HIGH- (thermal bounce) |
| kernel_nway | -0.400 | moderate- |
| kernel_seq | -0.154 | ok |
| nway | -0.481 | moderate- |
| sequential | -0.022 | ok |
| std_parallel | -0.505 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 112985.4ns | 50705.1ns | 222.8% | HIGH |
| head_tail_dyn | 98039.6ns | 34181.7ns | 286.8% | HIGH |
| kernel_nway | 111944.9ns | 49267.8ns | 227.2% | HIGH |
| kernel_seq | 105590.0ns | 52598.6ns | 200.7% | HIGH |
| nway | 134889.1ns | 68866.9ns | 195.9% | HIGH |
| sequential | 113663.2ns | 56780.6ns | 200.2% | HIGH |
| std_parallel | 123227.6ns | 92988.6ns | 132.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 39300.0-55445.9 ns)
  39300.0 |
  40107.3 |
  40914.6 |#############
  41721.9 |#############
  42529.2 |
  43336.5 |
  44143.8 |
  44951.0 |
  45758.3 |
  46565.6 |
  47372.9 |
  48180.2 |#############
  48987.5 |
  49794.8 |#############
  50602.1 |
  51409.4 |
  52216.7 |##########################
  53024.0 |##########################
  53831.3 |########################################
  54638.6 |########################################
  (2 below, 2 above range)

head_tail_dyn (n=18, range 33073.6-35154.1 ns)
  33073.6 |####################
  33177.6 |
  33281.7 |
  33385.7 |########################################
  33489.7 |
  33593.7 |
  33697.8 |########################################
  33801.8 |####################
  33905.8 |
  34009.8 |####################
  34113.9 |####################
  34217.9 |####################
  34321.9 |####################
  34426.0 |####################
  34530.0 |
  34634.0 |####################
  34738.0 |
  34842.1 |########################################
  34946.1 |
  35050.1 |
  (2 below, 2 above range)

kernel_nway (n=18, range 45407.0-51857.6 ns)
  45407.0 |
  45729.5 |
  46052.0 |
  46374.6 |
  46697.1 |
  47019.6 |
  47342.2 |
  47664.7 |##########################
  47987.2 |########################################
  48309.8 |#############
  48632.3 |
  48954.8 |#############
  49277.4 |
  49599.9 |########################################
  49922.4 |
  50245.0 |#############
  50567.5 |#############
  50890.0 |#############
  51212.6 |
  51535.1 |#############
  (2 below, 2 above range)

kernel_seq (n=18, range 51861.1-53630.6 ns)
  51861.1 |##########################
  51949.6 |#############
  52038.0 |########################################
  52126.5 |#############
  52215.0 |#############
  52303.5 |#############
  52391.9 |#############
  52480.4 |#############
  52568.9 |
  52657.4 |#############
  52745.8 |
  52834.3 |
  52922.8 |
  53011.3 |
  53099.7 |
  53188.2 |
  53276.7 |
  53365.1 |
  53453.6 |
  53542.1 |
  (2 below, 4 above range)

nway (n=18, range 63712.5-73272.9 ns)
  63712.5 |
  64190.5 |####################
  64668.5 |########################################
  65146.6 |####################
  65624.6 |
  66102.6 |####################
  66580.6 |
  67058.6 |
  67536.7 |########################################
  68014.7 |####################
  68492.7 |
  68970.7 |########################################
  69448.8 |
  69926.8 |
  70404.8 |
  70882.8 |########################################
  71360.8 |
  71838.9 |####################
  72316.9 |####################
  72794.9 |####################
  (1 below, 2 above range)

sequential (n=18, range 55638.9-58200.0 ns)
  55638.9 |########################################
  55767.0 |####################
  55895.0 |########################################
  56023.1 |
  56151.1 |####################
  56279.2 |####################
  56407.2 |####################
  56535.3 |####################
  56663.3 |
  56791.4 |####################
  56919.4 |####################
  57047.5 |
  57175.6 |####################
  57303.6 |
  57431.7 |
  57559.7 |####################
  57687.8 |
  57815.8 |
  57943.9 |
  58071.9 |####################
  (2 below, 2 above range)

std_parallel (n=18, range 88004.1-97038.9 ns)
  88004.1 |####################
  88455.9 |
  88907.6 |########################################
  89359.3 |
  89811.1 |####################
  90262.8 |
  90714.6 |
  91166.3 |########################################
  91618.0 |
  92069.8 |####################
  92521.5 |########################################
  92973.2 |
  93425.0 |####################
  93876.7 |####################
  94328.5 |
  94780.2 |
  95231.9 |########################################
  95683.7 |
  96135.4 |####################
  96587.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=213.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=287.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=221.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=200.7% of algo (FFI overhead may distort results)
- **nway**: bridge=194.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=131.2% of algo (FFI overhead may distort results)
