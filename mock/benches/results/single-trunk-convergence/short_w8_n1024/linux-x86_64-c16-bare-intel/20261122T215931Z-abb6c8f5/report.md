# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 6.7x slower than the field

std_parallel (77.95 us) is 6.7x the fastest (11.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (sequential, kernel_seq) are a dead heat (<1%)

sequential (11.61 us) and kernel_seq (11.71 us) differ by 0.88%, inside the noise, even though the wider field spreads 571.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} vs {nway, std_parallel} (205% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail, head_tail_dyn, kernel_nway} and a slow tier {nway, std_parallel} with a 205% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest sequential (11.61 us) to slowest std_parallel (77.95 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 6.86 us but its worst 20% at 25.21 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 11610.0 ns median (-12.6% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 6.71x (fastest 11610.0 ns, slowest 77949.9 ns)

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
| Counter frequency | 2112783970 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14250ns | 14140ns | 11786ns | 13772ns | 16197ns | base |
| head_tail_dyn | 14282ns | 14308ns | 12268ns | 13922ns | 15829ns | +0.23% |
| kernel_nway | 17850ns | 17628ns | 15069ns | 17251ns | 20139ns | +25.27% |
| kernel_seq | 12106ns | 12457ns | 8669ns | 11802ns | 14280ns | -15.04% |
| nway | 53940ns | 52143ns | 45590ns | 50680ns | 63005ns | +278.54% |
| sequential | 15142ns | 11831ns | 6999ns | 10788ns | 25746ns | +6.27% |
| std_parallel | 111531ns | 78207ns | 68992ns | 76432ns | 185448ns | +682.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 13421ns | 11168ns | 15180ns | base | 0.076 |
| head_tail_dyn | 13530ns | 11667ns | 14878ns | +0.82% | 0.076 |
| kernel_nway | 17131ns | 14594ns | 19264ns | +27.64% | 0.060 |
| kernel_seq | 11366ns | 8237ns | 13396ns | -15.31% | 0.090 |
| nway | 53215ns | 45066ns | 62098ns | +296.51% | 0.019 |
| sequential | 14837ns | 6863ns | 25209ns | +10.55% | 0.069 |
| std_parallel | 111003ns | 68744ns | 184413ns | +727.08% | 0.009 |

## Performance model

- Peak throughput: **0.149 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.077 | 51.7% |
| head_tail_dyn | 0.075 | 50.4% |
| kernel_nway | 0.061 | 40.7% |
| kernel_seq | 0.087 | 58.6% |
| nway | 0.020 | 13.3% |
| sequential | 0.088 | 59.1% |
| std_parallel | 0.013 | 8.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14250ns | 14250ns | base |
| head_tail_dyn | 14282ns | 14282ns | +0.23% |
| kernel_nway | 17850ns | 17850ns | +25.27% |
| kernel_seq | 12106ns | 12106ns | -15.04% |
| nway | 53940ns | 53940ns | +278.54% |
| sequential | 15142ns | 15142ns | +6.27% |
| std_parallel | 111531ns | 111531ns | +682.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13283ns | base | --- | [12420, 14581] | --- | --- | --- | --- |
| head_tail_dyn | 13608ns | no significant difference | [-1650, +1994]ns | [12522, 13910] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 16869ns | +3697.6ns (+27.8%) | [+1921, +4552]ns | [16019, 17708] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 11712ns | -2304.1ns (-17.3%) | [-2769, -1362]ns | [10233, 12123] | YES | 0.0000 | 0.0000 | 0 |
| nway | 51430ns | +37025.7ns (+278.7%) | [+33767, +43056]ns | [47665, 56658] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 11610ns | no significant difference | [-5503, +8591]ns | [6949, 23251] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 77950ns | +64036.8ns (+482.1%) | [+60949, +72891]ns | [73544, 88337] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14951ns | -13.7% | +21.2% | -12.3% | +215.2% | -17.3% | +479.1% |
| 1 | 2 | 13719ns | -0.2% | +12.3% | -14.6% | +258.2% | -15.4% | +461.5% |
| 1 | 3 | 11536ns | +19.9% | +42.8% | -22.0% | +439.0% | +14.6% | +566.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.017 | ok |
| head_tail_dyn | -0.096 | ok |
| kernel_nway | -0.425 | moderate- |
| kernel_seq | -0.022 | ok |
| nway | -0.101 | ok |
| sequential | -0.478 | moderate- |
| std_parallel | -0.158 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13883.2ns | 13421.1ns | 103.4% | HIGH |
| head_tail_dyn | 13834.7ns | 13530.5ns | 102.2% | HIGH |
| kernel_nway | 18300.2ns | 17130.7ns | 106.8% | HIGH |
| kernel_seq | 11664.0ns | 11366.0ns | 102.6% | HIGH |
| nway | 54133.5ns | 53215.2ns | 101.7% | HIGH |
| sequential | 15344.6ns | 14837.1ns | 103.4% | HIGH |
| std_parallel | 105023.4ns | 111003.4ns | 94.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11167.8-15180.1 ns)
  11167.8 |
  11368.4 |################
  11569.0 |########
  11769.7 |
  11970.3 |
  12170.9 |########
  12371.5 |########
  12572.1 |
  12772.7 |
  12973.4 |
  13174.0 |########################################
  13374.6 |
  13575.2 |
  13775.8 |
  13976.4 |
  14177.0 |########
  14377.7 |########
  14578.3 |########
  14778.9 |
  14979.5 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 11667.1-14877.9 ns)
  11667.1 |
  11827.7 |#############
  11988.2 |
  12148.8 |#############
  12309.3 |##########################
  12469.8 |#############
  12630.4 |
  12790.9 |
  12951.5 |
  13112.0 |
  13272.5 |
  13433.1 |########################################
  13593.6 |#############
  13754.2 |########################################
  13914.7 |#############
  14075.2 |#############
  14235.8 |
  14396.3 |
  14556.9 |
  14717.4 |
  (1 below, 3 above range)

kernel_nway (n=18, range 14593.8-19263.9 ns)
  14593.8 |
  14827.3 |
  15060.8 |####################
  15294.3 |
  15527.8 |####################
  15761.3 |####################
  15994.8 |####################
  16228.3 |####################
  16461.8 |
  16695.3 |########################################
  16928.8 |####################
  17162.3 |####################
  17395.8 |########################################
  17629.3 |####################
  17862.8 |
  18096.3 |
  18329.8 |
  18563.4 |####################
  18796.9 |
  19030.4 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 8236.8-13396.3 ns)
   8236.8 |
   8494.8 |
   8752.8 |#############
   9010.8 |#############
   9268.7 |
   9526.7 |
   9784.7 |#############
  10042.7 |
  10300.6 |#############
  10558.6 |
  10816.6 |
  11074.5 |##########################
  11332.5 |
  11590.5 |##########################
  11848.5 |########################################
  12106.4 |##########################
  12364.4 |
  12622.4 |
  12880.4 |
  13138.3 |
  (2 below, 3 above range)

nway (n=18, range 45066.0-62097.6 ns)
  45066.0 |
  45917.5 |##########################
  46769.1 |##########################
  47620.7 |########################################
  48472.3 |
  49323.9 |
  50175.4 |#############
  51027.0 |
  51878.6 |#############
  52730.2 |
  53581.8 |#############
  54433.3 |##########################
  55284.9 |
  56136.5 |
  56988.1 |
  57839.7 |#############
  58691.2 |
  59542.8 |
  60394.4 |#############
  61246.0 |
  (1 below, 3 above range)

sequential (n=18, range 6862.9-25208.6 ns)
   6862.9 |########################################
   7780.2 |
   8697.4 |
   9614.7 |
  10532.0 |
  11449.3 |
  12366.6 |
  13283.9 |
  14201.2 |
  15118.5 |#####
  16035.8 |
  16953.0 |#####
  17870.3 |
  18787.6 |#####
  19704.9 |
  20622.2 |
  21539.5 |
  22456.8 |#####
  23374.1 |#####
  24291.4 |##########
  (1 below, 2 above range)

std_parallel (n=18, range 68743.7-184413.3 ns)
  68743.7 |########################################
  74527.2 |##########################
  80310.7 |#############
  86094.2 |
  91877.6 |#############
  97661.1 |
  103444.6 |
  109228.1 |
  115011.6 |
  120795.0 |
  126578.5 |
  132362.0 |######
  138145.5 |
  143929.0 |
  149712.4 |
  155495.9 |
  161279.4 |
  167062.9 |
  172846.4 |
  178629.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=103.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=108.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.4% of algo (FFI overhead may distort results)
- **nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **sequential**: CV=56.5% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=104.8% of algo (FFI overhead may distort results)
- **std_parallel**: CV=78.1% (high variance, measurements may be unstable)
- **std_parallel**: bridge=95.1% of algo (FFI overhead may distort results)
