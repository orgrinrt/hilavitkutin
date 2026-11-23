# fold_light, short fiber (1 to 4 morsels), W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 64% faster than the next best (kernel_seq)

sequential (7.04 us) leads kernel_seq (11.55 us) by 64%, a clear separation rather than a photo finish. CV 134.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 37% (significant)

sequential is -5.59 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 10.5x slower than the field

std_parallel (74.18 us) is 10.5x the fastest (7.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 134.8%)

sequential wins on median (7.04 us) yet has the highest variance (CV 134.8%), while nway is the steadiest (CV 11.0%, 52.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.56)

kernel_seq's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, kernel_seq, head_tail_dyn, head_tail, kernel_nway} vs {nway, std_parallel} (221% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail_dyn, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 221% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.5x the fastest

Fastest sequential (7.04 us) to slowest std_parallel (74.18 us): 10.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.7x its best-20%

sequential's best 20% of batches run at 6.77 us but its worst 20% at 25.19 us (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7037.6 ns median (-53.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 10.54x (fastest 7037.6 ns, slowest 74184.1 ns)

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
| head_tail | 16194ns | 15707ns | 10916ns | 14835ns | 20872ns | base |
| head_tail_dyn | 13867ns | 14190ns | 10966ns | 13319ns | 16141ns | -14.37% |
| kernel_nway | 17867ns | 17235ns | 14853ns | 17067ns | 20575ns | +10.33% |
| kernel_seq | 12665ns | 12190ns | 8518ns | 11733ns | 16135ns | -21.79% |
| nway | 53485ns | 53549ns | 44890ns | 51929ns | 60117ns | +230.28% |
| sequential | 13398ns | 7174ns | 6905ns | 7350ns | 25716ns | -17.27% |
| std_parallel | 122415ns | 74467ns | 64706ns | 73398ns | 224796ns | +655.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15249ns | 10437ns | 19449ns | base | 0.001 |
| head_tail_dyn | 13160ns | 10434ns | 15298ns | -13.70% | 0.001 |
| kernel_nway | 17090ns | 14279ns | 19636ns | +12.08% | 0.001 |
| kernel_seq | 11838ns | 8028ns | 14850ns | -22.37% | 0.001 |
| nway | 52847ns | 44418ns | 59417ns | +246.56% | 0.000 |
| sequential | 13117ns | 6766ns | 25189ns | -13.98% | 0.001 |
| std_parallel | 121747ns | 64452ns | 223306ns | +698.40% | 0.000 |

## Performance model

- Peak throughput: **0.002 Gops/s** (sequential; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.001 | 45.0% |
| head_tail_dyn | 0.001 | 49.9% |
| kernel_nway | 0.001 | 41.0% |
| kernel_seq | 0.001 | 58.6% |
| nway | 0.000 | 12.8% |
| sequential | 0.002 | 96.1% |
| std_parallel | 0.000 | 9.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16194ns | 16194ns | base |
| head_tail_dyn | 13867ns | 13867ns | -14.37% |
| kernel_nway | 17867ns | 17867ns | +10.33% |
| kernel_seq | 12665ns | 12665ns | -21.79% |
| nway | 53485ns | 53485ns | +230.28% |
| sequential | 13398ns | 13398ns | -17.27% |
| std_parallel | 122415ns | 122415ns | +655.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15025ns | base | --- | [12316, 17716] | --- | --- | --- | --- |
| head_tail_dyn | 13555ns | no significant difference | [-3846, +79]ns | [11866, 14055] | no | 0.1155 | 0.0963 | 0 |
| kernel_nway | 16490ns | no significant difference | [-132, +3568]ns | [15533, 18235] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 11554ns | -2051.7ns (-13.7%) | [-6698, -1508]ns | [9657, 13866] | YES | 0.0151 | 0.0075 | 0 |
| nway | 52898ns | +37631.2ns (+250.5%) | [+35686, +40305]ns | [48589, 55886] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7038ns | -5594.5ns (-37.2%) | [-7932, -319]ns | [6881, 16820] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| std_parallel | 74184ns | +57840.6ns (+385.0%) | [+54471, +92010]ns | [69941, 112458] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11334ns | -6.7% | +33.8% | -20.2% | +352.8% | -39.0% | +1808.3% |
| 1 | 2 | 15025ns | +4.3% | +5.1% | -5.0% | +260.9% | -52.1% | +360.2% |
| 1 | 3 | 18376ns | -26.2% | +1.2% | -39.5% | +194.2% | -50.8% | +302.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.001 | ok |
| head_tail_dyn | -0.441 | moderate- |
| kernel_nway | -0.078 | ok |
| kernel_seq | -0.563 | HIGH- (thermal bounce) |
| nway | -0.196 | ok |
| sequential | -0.102 | ok |
| std_parallel | -0.184 | ok |

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
| head_tail | 15699.7ns | 15248.9ns | 103.0% | HIGH |
| head_tail_dyn | 13479.2ns | 13159.7ns | 102.4% | HIGH |
| kernel_nway | 17495.4ns | 17090.3ns | 102.4% | HIGH |
| kernel_seq | 12465.5ns | 11838.0ns | 105.3% | HIGH |
| nway | 53681.0ns | 52846.8ns | 101.6% | HIGH |
| sequential | 13668.6ns | 13116.6ns | 104.2% | HIGH |
| std_parallel | 121834.0ns | 121747.0ns | 100.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10436.8-19449.2 ns)
  10436.8 |
  10887.4 |####################
  11338.0 |########################################
  11788.6 |
  12239.3 |
  12689.9 |####################
  13140.5 |####################
  13591.1 |
  14041.8 |####################
  14492.4 |
  14943.0 |########################################
  15393.6 |
  15844.2 |####################
  16294.9 |####################
  16745.5 |
  17196.1 |####################
  17646.7 |########################################
  18097.4 |
  18548.0 |####################
  18998.6 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 10433.6-15298.5 ns)
  10433.6 |########################################
  10676.8 |
  10920.1 |
  11163.3 |
  11406.6 |#############
  11649.8 |
  11893.1 |
  12136.3 |##########################
  12379.6 |#############
  12622.8 |
  12866.0 |
  13109.3 |
  13352.5 |##########################
  13595.8 |#############
  13839.0 |##########################
  14082.3 |#############
  14325.5 |
  14568.8 |
  14812.0 |
  15055.2 |
  (1 below, 4 above range)

kernel_nway (n=18, range 14278.6-19635.8 ns)
  14278.6 |
  14546.5 |
  14814.4 |####################
  15082.2 |####################
  15350.1 |########################################
  15617.9 |########################################
  15885.8 |
  16153.6 |####################
  16421.5 |
  16689.4 |####################
  16957.2 |
  17225.1 |
  17492.9 |
  17760.8 |####################
  18028.6 |########################################
  18296.5 |####################
  18564.4 |####################
  18832.2 |
  19100.1 |####################
  19367.9 |
  (2 below, 2 above range)

kernel_seq (n=18, range 8027.6-14849.6 ns)
   8027.6 |#############
   8368.7 |#############
   8709.8 |
   9050.9 |
   9392.0 |########################################
   9733.1 |
  10074.2 |
  10415.3 |#############
  10756.4 |
  11097.5 |##########################
  11438.6 |
  11779.7 |#############
  12120.8 |
  12461.9 |
  12803.0 |#############
  13144.1 |##########################
  13485.2 |
  13826.3 |
  14167.4 |#############
  14508.5 |#############
  (1 below, 3 above range)

nway (n=18, range 44417.7-59417.2 ns)
  44417.7 |
  45167.6 |
  45917.6 |
  46667.6 |####################
  47417.6 |########################################
  48167.6 |
  48917.5 |####################
  49667.5 |
  50417.5 |####################
  51167.5 |
  51917.4 |####################
  52667.4 |########################################
  53417.4 |####################
  54167.4 |####################
  54917.4 |####################
  55667.3 |####################
  56417.3 |
  57167.3 |
  57917.3 |
  58667.2 |####################
  (2 below, 3 above range)

sequential (n=18, range 6765.9-25189.2 ns)
   6765.9 |########################################
   7687.0 |
   8608.2 |####
   9529.4 |
  10450.5 |####
  11371.7 |
  12292.9 |
  13214.0 |
  14135.2 |
  15056.4 |
  15977.5 |
  16898.7 |
  17819.9 |
  18741.0 |
  19662.2 |
  20583.4 |
  21504.5 |
  22425.7 |
  23346.9 |
  24268.0 |########
  (2 below, 3 above range)

std_parallel (n=18, range 64452.4-223305.9 ns)
  64452.4 |##########################
  72395.1 |########################################
  80337.7 |
  88280.4 |
  96223.1 |
  104165.8 |######
  112108.4 |#############
  120051.1 |
  127993.8 |
  135936.5 |
  143879.1 |
  151821.8 |
  159764.5 |
  167707.1 |
  175649.8 |
  183592.5 |
  191535.2 |
  199477.8 |
  207420.5 |
  215363.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=24.0% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=107.2% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=22.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.6% of algo (FFI overhead may distort results)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: CV=72.3% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sequential**: bridge=103.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=78.6% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **std_parallel**: bridge=102.5% of algo (FFI overhead may distort results)
