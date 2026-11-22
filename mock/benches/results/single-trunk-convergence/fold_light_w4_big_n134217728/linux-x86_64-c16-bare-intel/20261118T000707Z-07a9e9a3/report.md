# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 13% faster than the next best (std_parallel)

kernel_nway (15.19 ms) leads std_parallel (17.10 ms) by 13%, a clear separation rather than a photo finish. CV 22.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 27% (significant)

kernel_nway is -5.85 ms (27%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### head_tail_dyn is an outlier: 3.4x slower than the field

head_tail_dyn (52.36 ms) is 3.4x the fastest (15.19 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.51)

kernel_seq's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, head_tail, sequential, kernel_seq, nway} vs {head_tail_dyn} (68% apart)

The field splits into a fast tier {kernel_nway, std_parallel, head_tail, sequential, kernel_seq, nway} and a slow tier {head_tail_dyn} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.4x the fastest

Fastest kernel_nway (15.19 ms) to slowest head_tail_dyn (52.36 ms): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### head_tail_dyn is inconsistent: worst-20% is 5.3x its best-20%

head_tail_dyn's best 20% of batches run at 27.28 ms but its worst 20% at 143.96 ms (5.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 15192396.9 ns median (-30.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.45x (fastest 15192396.9 ns, slowest 52359155.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 21412837ns | 21835311ns | 20236051ns | 21319629ns | 22141042ns | base |
| head_tail_dyn | 74700559ns | 52363588ns | 27288225ns | 44322828ns | 143973324ns | +248.86% |
| kernel_nway | 16641489ns | 15202340ns | 14968721ns | 15171747ns | 19682485ns | -22.28% |
| kernel_seq | 31132552ns | 31132543ns | 30926060ns | 31101097ns | 31282982ns | +45.39% |
| nway | 30809343ns | 31208388ns | 15307621ns | 29369738ns | 40719611ns | +43.88% |
| sequential | 27964586ns | 27820954ns | 26378656ns | 27421293ns | 29572489ns | +30.60% |
| std_parallel | 18525530ns | 17099040ns | 15594925ns | 16624254ns | 22842748ns | -13.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 21405106ns | 20229671ns | 22135374ns | base | 6.270 |
| head_tail_dyn | 74691391ns | 27276310ns | 143958996ns | +248.94% | 1.797 |
| kernel_nway | 16634701ns | 14966125ns | 19676035ns | -22.29% | 8.069 |
| kernel_seq | 31125840ns | 30915847ns | 31272130ns | +45.41% | 4.312 |
| nway | 30804038ns | 15305372ns | 40712630ns | +43.91% | 4.357 |
| sequential | 27963293ns | 26377320ns | 29571282ns | +30.64% | 4.800 |
| std_parallel | 18521912ns | 15592635ns | 22838346ns | -13.47% | 7.246 |

## Performance model

- Peak throughput: **8.968 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.150 | 68.6% |
| head_tail_dyn | 2.563 | 28.6% |
| kernel_nway | 8.835 | 98.5% |
| kernel_seq | 4.312 | 48.1% |
| nway | 4.301 | 48.0% |
| sequential | 4.825 | 53.8% |
| std_parallel | 7.851 | 87.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 21412837ns | 21412837ns | base |
| head_tail_dyn | 74700559ns | 74700559ns | +248.86% |
| kernel_nway | 16641489ns | 16641489ns | -22.28% |
| kernel_seq | 31132552ns | 31132552ns | +45.39% |
| nway | 30809343ns | 30809343ns | +43.88% |
| sequential | 27964586ns | 27964586ns | +30.60% |
| std_parallel | 18525530ns | 18525530ns | -13.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 21823339ns | base | --- | [20256604, 22135374] | --- | --- | --- | --- |
| head_tail_dyn | 52359155ns | +31156038.9ns (+142.8%) | [+5804172, +122898647]ns | [27756023, 143958996] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 15192397ns | -5852310.6ns (-26.8%) | [-6943844, -1515061]ns | [15035670, 19676035] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| kernel_seq | 31129053ns | +9322839.5ns (+42.7%) | [+8966916, +10872449]ns | [30976338, 31272130] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 31202899ns | no significant difference | [-692277, +19693037]ns | [20496585, 40712630] | no | 0.2625 | 0.2188 | 0 |
| sequential | 27819636ns | +5867785.5ns (+26.9%) | [+4492099, +9314677]ns | [26498960, 29571282] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 17095659ns | no significant difference | [-6345511, +2581742]ns | [15631730, 22838346] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 21951851ns | +26.4% | -10.4% | +42.5% | +15.0% | +26.7% | -28.8% |
| 1 | 2 | 22006861ns | +175.6% | -31.6% | +40.8% | +41.8% | +20.4% | -22.3% |
| 1 | 3 | 20256604ns | +569.8% | -25.1% | +53.7% | +77.6% | +46.0% | +12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.183 | ok |
| head_tail_dyn | -0.032 | ok |
| kernel_nway | -0.178 | ok |
| kernel_seq | -0.511 | HIGH- (thermal bounce) |
| nway | -0.003 | ok |
| sequential | -0.451 | moderate- |
| std_parallel | -0.070 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 21396681.2ns | 21405105.6ns | 100.0% | HIGH |
| head_tail_dyn | 66472085.4ns | 74691391.3ns | 89.0% | HIGH |
| kernel_nway | 16525144.8ns | 16634700.7ns | 99.3% | HIGH |
| kernel_seq | 31562363.8ns | 31125840.2ns | 101.4% | HIGH |
| nway | 30955612.2ns | 30804038.1ns | 100.5% | HIGH |
| sequential | 27806974.6ns | 27963292.7ns | 99.4% | HIGH |
| std_parallel | 17489329.8ns | 18521911.7ns | 94.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 20229671.4-22135373.7 ns)
  20229671.4 |########################################
  20324956.5 |
  20420241.6 |
  20515526.7 |
  20610811.9 |
  20706097.0 |
  20801382.1 |
  20896667.2 |
  20991952.3 |
  21087237.4 |
  21182522.5 |
  21277807.7 |
  21373092.8 |
  21468377.9 |
  21563663.0 |
  21658948.1 |
  21754233.2 |####################
  21849518.4 |####################
  21944803.5 |
  22040088.6 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 27276309.8-143958996.2 ns)
  27276309.8 |########################################
  33110444.1 |####################
  38944578.4 |
  44778712.8 |
  50612847.1 |
  56446981.4 |
  62281115.7 |
  68115250.0 |####################
  73949384.3 |
  79783518.7 |
  85617653.0 |####################
  91451787.3 |
  97285921.6 |
  103120055.9 |
  108954190.2 |
  114788324.6 |
  120622458.9 |
  126456593.2 |
  132290727.5 |
  138124861.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 14966124.8-19676035.4 ns)
  14966124.8 |########################################
  15201620.3 |##########################
  15437115.9 |
  15672611.4 |
  15908106.9 |
  16143602.4 |
  16379098.0 |
  16614593.5 |
  16850089.0 |
  17085584.5 |
  17321080.1 |
  17556575.6 |
  17792071.1 |
  18027566.7 |
  18263062.2 |
  18498557.7 |
  18734053.2 |
  18969548.8 |
  19205044.3 |
  19440539.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 30915847.0-31272130.1 ns)
  30915847.0 |####################
  30933661.2 |
  30951475.3 |
  30969289.5 |
  30987103.6 |
  31004917.8 |
  31022731.9 |####################
  31040546.1 |
  31058360.3 |
  31076174.4 |####################
  31093988.6 |
  31111802.7 |
  31129616.9 |
  31147431.0 |
  31165245.2 |########################################
  31183059.4 |
  31200873.5 |
  31218687.7 |
  31236501.8 |
  31254316.0 |
  (0 below, 1 above range)

nway (n=6, range 15305372.5-40712630.2 ns)
  15305372.5 |####################
  16575735.4 |
  17846098.3 |
  19116461.2 |
  20386824.1 |
  21657186.9 |
  22927549.8 |
  24197912.7 |
  25468275.6 |####################
  26738638.5 |
  28009001.4 |
  29279364.3 |
  30549727.1 |########################################
  31820090.0 |
  33090452.9 |
  34360815.8 |####################
  35631178.7 |
  36901541.6 |
  38171904.5 |
  39442267.4 |
  (0 below, 1 above range)

sequential (n=6, range 26377319.7-29571281.5 ns)
  26377319.7 |########################################
  26537017.8 |########################################
  26696715.9 |
  26856414.0 |
  27016112.1 |
  27175810.2 |
  27335508.3 |
  27495206.3 |
  27654904.4 |########################################
  27814602.5 |########################################
  27974300.6 |
  28133998.7 |
  28293696.8 |########################################
  28453394.9 |
  28613093.0 |
  28772791.1 |
  28932489.2 |
  29092187.3 |
  29251885.4 |
  29411583.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 15592634.7-22838346.1 ns)
  15592634.7 |########################################
  15954920.3 |
  16317205.8 |
  16679491.4 |
  17041777.0 |
  17404062.6 |
  17766348.1 |
  18128633.7 |#############
  18490919.3 |
  18853204.8 |
  19215490.4 |
  19577776.0 |
  19940061.5 |
  20302347.1 |
  20664632.7 |
  21026918.2 |
  21389203.8 |
  21751489.4 |#############
  22113775.0 |
  22476060.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=81.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 5.3x (possible bimodal distribution)
- **head_tail_dyn**: bridge=70.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=20.1% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.5% of algo (FFI overhead may distort results)
- **nway**: CV=30.3% (high variance, measurements may be unstable)
- **nway**: bridge=92.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=92.2% of algo (FFI overhead may distort results)
