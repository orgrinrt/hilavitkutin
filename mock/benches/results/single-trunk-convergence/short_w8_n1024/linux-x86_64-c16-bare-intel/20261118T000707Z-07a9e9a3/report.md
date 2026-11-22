# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 15% faster than the next best (kernel_seq)

sequential (9.56 us) leads kernel_seq (10.96 us) by 15%, a clear separation rather than a photo finish. CV 87.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 7.7x slower than the field

std_parallel (73.47 us) is 7.7x the fastest (9.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 87.3%)

sequential wins on median (9.56 us) yet has the highest variance (CV 87.3%), while head_tail is the steadiest (CV 7.0%, 12.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (169% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 169% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.7x the fastest

Fastest sequential (9.56 us) to slowest std_parallel (73.47 us): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 6.82 us but its worst 20% at 25.45 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 9562.0 ns median (-24.3% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.68x (fastest 9562.0 ns, slowest 73472.3 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 13231ns | 13581ns | 11776ns | 12977ns | 14340ns | base |
| head_tail_dyn | 15956ns | 15969ns | 11484ns | 14805ns | 19917ns | +20.59% |
| kernel_nway | 20018ns | 19341ns | 15977ns | 19111ns | 23398ns | +51.29% |
| kernel_seq | 11801ns | 11551ns | 10603ns | 11356ns | 13069ns | -10.81% |
| nway | 52512ns | 50510ns | 46445ns | 50060ns | 59224ns | +296.89% |
| sequential | 14524ns | 9749ns | 6960ns | 9415ns | 25970ns | +9.77% |
| std_parallel | 75399ns | 73773ns | 64001ns | 71687ns | 86666ns | +469.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12479ns | 11178ns | 13424ns | base | 0.082 |
| head_tail_dyn | 15110ns | 10982ns | 18690ns | +21.08% | 0.068 |
| kernel_nway | 19264ns | 15480ns | 22412ns | +54.37% | 0.053 |
| kernel_seq | 11017ns | 9970ns | 12029ns | -11.72% | 0.093 |
| nway | 51865ns | 45904ns | 58502ns | +315.62% | 0.020 |
| sequential | 14238ns | 6824ns | 25450ns | +14.10% | 0.072 |
| std_parallel | 75069ns | 63757ns | 86221ns | +501.56% | 0.014 |

## Performance model

- Peak throughput: **0.150 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.081 | 54.0% |
| head_tail_dyn | 0.067 | 44.7% |
| kernel_nway | 0.055 | 36.7% |
| kernel_seq | 0.093 | 62.3% |
| nway | 0.021 | 13.7% |
| sequential | 0.107 | 71.4% |
| std_parallel | 0.014 | 9.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 13231ns | 13231ns | base |
| head_tail_dyn | 15956ns | 15956ns | +20.59% |
| kernel_nway | 20018ns | 20018ns | +51.29% |
| kernel_seq | 11801ns | 11801ns | -10.81% |
| nway | 52512ns | 52512ns | +296.89% |
| sequential | 14524ns | 14524ns | +9.77% |
| std_parallel | 75399ns | 75399ns | +469.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12639ns | base | --- | [11742, 13181] | --- | --- | --- | --- |
| head_tail_dyn | 15253ns | no significant difference | [-850, +4965]ns | [12208, 16720] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 18575ns | +6893.2ns (+54.5%) | [+4692, +9116]ns | [16925, 21500] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 10955ns | -1468.4ns (-11.6%) | [-2405, -809]ns | [10193, 11585] | YES | 0.0020 | 0.0013 | 0 |
| nway | 49892ns | +38213.2ns (+302.3%) | [+35720, +40727]ns | [48451, 54484] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 9562ns | no significant difference | [-4664, +10108]ns | [6954, 23719] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 73472ns | +60492.7ns (+478.6%) | [+56228, +67978]ns | [67727, 80482] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11614ns | +43.7% | +56.6% | -2.0% | +331.4% | -18.4% | +507.2% |
| 1 | 2 | 13060ns | +26.2% | +67.0% | -15.8% | +317.2% | -23.0% | +543.5% |
| 1 | 3 | 13269ns | -16.4% | +27.4% | -19.1% | +267.1% | -13.1% | +422.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.105 | ok |
| head_tail_dyn | -0.148 | ok |
| kernel_nway | -0.625 | HIGH- (thermal bounce) |
| kernel_seq | -0.008 | ok |
| nway | -0.631 | HIGH- (thermal bounce) |
| sequential | -0.041 | ok |
| std_parallel | -0.663 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12828.0ns | 12479.0ns | 102.8% | HIGH |
| head_tail_dyn | 16119.5ns | 15109.6ns | 106.7% | HIGH |
| kernel_nway | 19973.2ns | 19263.8ns | 103.7% | HIGH |
| kernel_seq | 11550.4ns | 11017.0ns | 104.8% | HIGH |
| nway | 52399.7ns | 51864.8ns | 101.0% | HIGH |
| sequential | 14663.3ns | 14237.9ns | 103.0% | HIGH |
| std_parallel | 74566.4ns | 75068.8ns | 99.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11177.7-13424.2 ns)
  11177.7 |####################
  11290.1 |
  11402.4 |####################
  11514.7 |####################
  11627.0 |####################
  11739.3 |####################
  11851.7 |####################
  11964.0 |
  12076.3 |
  12188.6 |####################
  12301.0 |####################
  12413.3 |
  12525.6 |
  12637.9 |
  12750.2 |
  12862.6 |####################
  12974.9 |####################
  13087.2 |########################################
  13199.5 |####################
  13311.9 |########################################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 10981.9-18690.5 ns)
  10981.9 |########################################
  11367.3 |
  11752.8 |
  12138.2 |##########
  12523.6 |
  12909.0 |
  13294.5 |
  13679.9 |
  14065.3 |
  14450.7 |####################
  14836.2 |##########
  15221.6 |##########
  15607.0 |
  15992.5 |##########
  16377.9 |##############################
  16763.3 |##########
  17148.7 |
  17534.2 |
  17919.6 |
  18305.0 |
  (1 below, 3 above range)

kernel_nway (n=18, range 15480.3-22411.8 ns)
  15480.3 |
  15826.9 |########################################
  16173.5 |########################################
  16520.1 |
  16866.6 |
  17213.2 |####################
  17559.8 |
  17906.4 |########################################
  18252.9 |####################
  18599.5 |####################
  18946.1 |
  19292.6 |
  19639.2 |
  19985.8 |
  20332.4 |
  20678.9 |####################
  21025.5 |####################
  21372.1 |########################################
  21718.7 |
  22065.2 |########################################
  (1 below, 2 above range)

kernel_seq (n=18, range 9969.8-12029.3 ns)
   9969.8 |########################################
  10072.7 |
  10175.7 |
  10278.7 |#############
  10381.7 |#############
  10484.7 |
  10587.6 |
  10690.6 |##########################
  10793.6 |
  10896.6 |
  10999.5 |
  11102.5 |#############
  11205.5 |#############
  11308.5 |
  11411.4 |#############
  11514.4 |##########################
  11617.4 |
  11720.4 |########################################
  11823.3 |
  11926.3 |
  (2 below, 1 above range)

nway (n=18, range 45903.9-58501.6 ns)
  45903.9 |
  46533.8 |#############
  47163.6 |#############
  47793.5 |##########################
  48423.4 |##########################
  49053.3 |#############
  49683.2 |########################################
  50313.1 |
  50942.9 |
  51572.8 |#############
  52202.7 |
  52832.6 |
  53462.5 |#############
  54092.4 |
  54722.3 |#############
  55352.1 |
  55982.0 |
  56611.9 |
  57241.8 |
  57871.7 |#############
  (1 below, 3 above range)

sequential (n=18, range 6823.5-25449.9 ns)
   6823.5 |########################################
   7754.8 |
   8686.1 |
   9617.5 |
  10548.8 |
  11480.1 |#####
  12411.4 |#####
  13342.8 |
  14274.1 |
  15205.4 |#####
  16136.7 |
  17068.0 |
  17999.4 |
  18930.7 |
  19862.0 |
  20793.3 |
  21724.7 |
  22656.0 |
  23587.3 |###############
  24518.6 |
  (1 below, 3 above range)

std_parallel (n=18, range 63756.7-86221.0 ns)
  63756.7 |
  64879.9 |
  66003.1 |########################################
  67126.4 |##########################
  68249.6 |
  69372.8 |
  70496.0 |##########################
  71619.2 |
  72742.4 |##########################
  73865.7 |#############
  74988.9 |
  76112.1 |#############
  77235.3 |
  78358.5 |#############
  79481.7 |
  80605.0 |#############
  81728.2 |#############
  82851.4 |
  83974.6 |
  85097.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=22.4% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=104.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.5% of algo (FFI overhead may distort results)
- **nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: CV=58.6% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=106.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.4% of algo (FFI overhead may distort results)
