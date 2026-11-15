# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.2x slower than the field

sequential (131.22 us) is 2.2x the fastest (60.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader kernel_seq vs stability leader head_tail (+5% speed for 1.2x steadier)

kernel_seq is fastest (60.35 us, CV 14.3%); head_tail gives up 5.1% median for 1.2x lower variance (CV 12.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### sequential is inconsistent: worst-20% is 2.0x its best-20%

sequential's best 20% of batches run at 72.21 us but its worst 20% at 147.16 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 60354.4 ns median (-4.9% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 2.17x (fastest 60354.4 ns, slowest 131216.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 66444ns | 64207ns | 57305ns | 63188ns | 75897ns | base |
| head_tail_dyn | 72810ns | 72030ns | 56829ns | 69657ns | 85530ns | +9.58% |
| kernel_nway | 69444ns | 69183ns | 56159ns | 66558ns | 80416ns | +4.52% |
| kernel_seq | 62643ns | 61189ns | 54266ns | 59431ns | 71649ns | -5.72% |
| nway | 134334ns | 119559ns | 95591ns | 120246ns | 174838ns | +102.18% |
| sequential | 125111ns | 131873ns | 72556ns | 127237ns | 148201ns | +88.30% |
| std_parallel | 93029ns | 93452ns | 77524ns | 89178ns | 106558ns | +40.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 65794ns | 56532ns | 75295ns | base | 0.996 |
| head_tail_dyn | 71916ns | 56228ns | 84081ns | +9.30% | 0.911 |
| kernel_nway | 68724ns | 55504ns | 79811ns | +4.45% | 0.954 |
| kernel_seq | 61929ns | 53831ns | 70602ns | -5.87% | 1.058 |
| nway | 133623ns | 95061ns | 173967ns | +103.09% | 0.490 |
| sequential | 124336ns | 72212ns | 147160ns | +88.98% | 0.527 |
| std_parallel | 92267ns | 76961ns | 105600ns | +40.24% | 0.710 |

## Performance model

- Peak throughput: **1.217 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.033 | 84.8% |
| head_tail_dyn | 0.916 | 75.2% |
| kernel_nway | 0.958 | 78.7% |
| kernel_seq | 1.086 | 89.2% |
| nway | 0.552 | 45.3% |
| sequential | 0.499 | 41.0% |
| std_parallel | 0.706 | 58.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 66444ns | 66444ns | base |
| head_tail_dyn | 72810ns | 72810ns | +9.58% |
| kernel_nway | 69444ns | 69444ns | +4.52% |
| kernel_seq | 62643ns | 62643ns | -5.72% |
| nway | 134334ns | 134334ns | +102.18% |
| sequential | 125111ns | 125111ns | +88.30% |
| std_parallel | 93029ns | 93029ns | +40.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 63444ns | base | --- | [60207, 70283] | --- | --- | --- | --- |
| head_tail_dyn | 71541ns | no significant difference | [-912, +11670]ns | [63803, 79221] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 68420ns | no significant difference | [-4779, +10436]ns | [61755, 71290] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 60354ns | no significant difference | [-9016, +402]ns | [56350, 62999] | no | 0.2855 | 0.2379 | 0 |
| nway | 118797ns | +53053.1ns (+83.6%) | [+39479, +102096]ns | [108569, 165410] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 131216ns | +62757.4ns (+98.9%) | [+53150, +72995]ns | [125893, 138386] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 92769ns | +28028.5ns (+44.2%) | [+17090, +34374]ns | [82198, 98455] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 62028ns | +9.2% | -2.7% | -9.8% | +82.0% | +125.4% | +30.8% |
| 1 | 2 | 71725ns | +13.3% | -4.6% | -13.8% | +51.4% | +82.9% | +47.0% |
| 1 | 3 | 61408ns | +6.7% | +16.1% | +4.6% | +181.6% | +41.6% | +49.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.665 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.655 | HIGH- (thermal bounce) |
| kernel_nway | -0.047 | ok |
| kernel_seq | -0.038 | ok |
| nway | -0.203 | moderate- |
| sequential | -0.088 | ok |
| std_parallel | -0.541 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 420892.3ns | 65794.4ns | 639.7% | HIGH |
| head_tail_dyn | 459218.0ns | 71916.2ns | 638.5% | HIGH |
| kernel_nway | 412245.8ns | 68724.4ns | 599.9% | HIGH |
| kernel_seq | 436115.3ns | 61929.4ns | 704.2% | HIGH |
| nway | 488864.0ns | 133623.1ns | 365.9% | HIGH |
| sequential | 455651.4ns | 124336.1ns | 366.5% | HIGH |
| std_parallel | 445984.5ns | 92267.1ns | 483.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 56531.5-75295.1 ns)
  56531.5 |
  57469.7 |####################
  58407.9 |##########
  59346.1 |
  60284.2 |
  61222.4 |########################################
  62160.6 |
  63098.8 |
  64036.9 |
  64975.1 |##########
  65913.3 |##########
  66851.5 |##########
  67789.7 |
  68727.8 |
  69666.0 |####################
  70604.2 |
  71542.4 |
  72480.5 |##########
  73418.7 |
  74356.9 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 56227.8-84080.7 ns)
  56227.8 |########################################
  57620.5 |####################
  59013.1 |####################
  60405.8 |
  61798.4 |
  63191.1 |
  64583.7 |####################
  65976.3 |
  67369.0 |
  68761.6 |########################################
  70154.3 |####################
  71546.9 |####################
  72939.6 |
  74332.2 |
  75724.8 |####################
  77117.5 |####################
  78510.1 |########################################
  79902.8 |
  81295.4 |
  82688.1 |####################
  (1 below, 3 above range)

kernel_nway (n=18, range 55504.4-79811.2 ns)
  55504.4 |
  56719.8 |####################
  57935.1 |####################
  59150.4 |####################
  60365.8 |####################
  61581.1 |####################
  62796.4 |
  64011.8 |####################
  65227.1 |
  66442.5 |####################
  67657.8 |########################################
  68873.1 |####################
  70088.5 |########################################
  71303.8 |####################
  72519.1 |########################################
  73734.5 |####################
  74949.8 |
  76165.1 |
  77380.5 |
  78595.8 |
  (1 below, 1 above range)

kernel_seq (n=18, range 53830.7-70602.1 ns)
  53830.7 |
  54669.3 |##########################
  55507.9 |##########################
  56346.4 |#############
  57185.0 |
  58023.6 |##########################
  58862.1 |#############
  59700.7 |
  60539.3 |
  61377.8 |########################################
  62216.4 |#############
  63055.0 |#############
  63893.5 |
  64732.1 |
  65570.7 |
  66409.2 |#############
  67247.8 |
  68086.4 |
  68924.9 |#############
  69763.5 |
  (1 below, 2 above range)

nway (n=18, range 95061.2-173967.1 ns)
  95061.2 |####################
  99006.5 |
  102951.8 |####################
  106897.1 |
  110842.4 |########################################
  114787.7 |
  118733.0 |
  122678.3 |##########
  126623.6 |
  130568.9 |##########
  134514.2 |
  138459.5 |
  142404.7 |
  146350.0 |
  150295.3 |
  154240.6 |
  158185.9 |####################
  162131.2 |
  166076.5 |##########
  170021.8 |####################
  (1 below, 2 above range)

sequential (n=18, range 72212.2-147159.5 ns)
  72212.2 |####################
  75959.6 |
  79706.9 |
  83454.3 |
  87201.7 |
  90949.0 |
  94696.4 |
  98443.8 |##########
  102191.1 |
  105938.5 |
  109685.9 |
  113433.2 |
  117180.6 |
  120928.0 |
  124675.3 |##############################
  128422.7 |########################################
  132170.1 |##########
  135917.4 |##########
  139664.8 |##########
  143412.2 |####################
  (1 below, 2 above range)

std_parallel (n=18, range 76960.9-105600.5 ns)
  76960.9 |####################
  78392.8 |
  79824.8 |########################################
  81256.8 |########################################
  82688.8 |
  84120.8 |
  85552.8 |####################
  86984.7 |
  88416.7 |
  89848.7 |####################
  91280.7 |####################
  92712.7 |########################################
  94144.6 |####################
  95576.6 |####################
  97008.6 |
  98440.6 |
  99872.6 |####################
  101304.5 |
  102736.5 |
  104168.5 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=659.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=601.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=592.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=677.8% of algo (FFI overhead may distort results)
- **nway**: CV=24.3% (high variance, measurements may be unstable)
- **nway**: bridge=414.1% of algo (FFI overhead may distort results)
- **sequential**: CV=21.2% (high variance, measurements may be unstable)
- **sequential**: bridge=344.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=480.2% of algo (FFI overhead may distort results)
