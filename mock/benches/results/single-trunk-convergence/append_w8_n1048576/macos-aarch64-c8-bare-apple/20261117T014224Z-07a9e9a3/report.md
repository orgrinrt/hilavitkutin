# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 46% faster than the next best (std_parallel)

kernel_nway (71.00 us) leads std_parallel (103.76 us) by 46%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 36% (significant)

kernel_nway is -39.98 us (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 10.5x slower than the field

sequential (745.31 us) is 10.5x the fastest (71.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, head_tail, kernel_seq, head_tail_dyn, nway} vs {sequential} (242% apart)

The field splits into a fast tier {kernel_nway, std_parallel, head_tail, kernel_seq, head_tail_dyn, nway} and a slow tier {sequential} with a 242% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.5x the fastest

Fastest kernel_nway (71.00 us) to slowest sequential (745.31 us): 10.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 71000.0 ns median (-36.0% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 10.50x (fastest 71000.0 ns, slowest 745308.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
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
| head_tail | 113045ns | 112579ns | 111515ns | 112512ns | 114610ns | base |
| head_tail_dyn | 113854ns | 113083ns | 111310ns | 112646ns | 116938ns | +0.72% |
| kernel_nway | 75882ns | 73423ns | 70829ns | 73418ns | 82104ns | -32.87% |
| kernel_seq | 113041ns | 112377ns | 111101ns | 112174ns | 115313ns | -0.00% |
| nway | 238925ns | 220371ns | 210613ns | 220088ns | 281336ns | +111.35% |
| sequential | 745928ns | 746590ns | 733582ns | 744931ns | 753597ns | +559.85% |
| std_parallel | 107432ns | 105362ns | 101100ns | 104425ns | 115107ns | -4.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 111337ns | 109968ns | 112891ns | base | 9.418 |
| head_tail_dyn | 112291ns | 110296ns | 114942ns | +0.86% | 9.338 |
| kernel_nway | 72966ns | 67665ns | 78984ns | -34.46% | 14.371 |
| kernel_seq | 111718ns | 110058ns | 113990ns | +0.34% | 9.386 |
| nway | 236242ns | 207526ns | 279074ns | +112.19% | 4.439 |
| sequential | 744368ns | 731978ns | 751876ns | +568.57% | 1.409 |
| std_parallel | 105858ns | 99743ns | 113101ns | -4.92% | 9.905 |

## Performance model

- Peak throughput: **15.497 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.455 | 61.0% |
| head_tail_dyn | 9.418 | 60.8% |
| kernel_nway | 14.769 | 95.3% |
| kernel_seq | 9.446 | 61.0% |
| nway | 4.805 | 31.0% |
| sequential | 1.407 | 9.1% |
| std_parallel | 10.106 | 65.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 113045ns | 113045ns | base |
| head_tail_dyn | 113854ns | 113854ns | +0.72% |
| kernel_nway | 75882ns | 75882ns | -32.87% |
| kernel_seq | 113041ns | 113041ns | -0.00% |
| nway | 238925ns | 238925ns | +111.35% |
| sequential | 745928ns | 745928ns | +559.85% |
| std_parallel | 107432ns | 107432ns | -4.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 110902ns | base | --- | [110483, 111733] | --- | --- | --- | --- |
| head_tail_dyn | 111340ns | no significant difference | [-221, +1077]ns | [110906, 112202] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 71000ns | -39981.2ns (-36.1%) | [-41760, -35394]ns | [69446, 75279] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 111002ns | no significant difference | [-1177, +1421]ns | [110240, 111762] | no | 0.8145 | 0.8145 | 0 |
| nway | 218204ns | +106187.4ns (+95.7%) | [+101971, +124206]ns | [212808, 237773] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 745308ns | +634720.8ns (+572.3%) | [+628021, +638119]ns | [740262, 749183] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 103756ns | -7879.1ns (-7.1%) | [-9175, -3652]ns | [101969, 108062] | YES | 0.0113 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 111906ns | +0.0% | -37.4% | -1.4% | +91.3% | +566.8% | -6.3% |
| 1 | 2 | 110902ns | -0.1% | -35.3% | -0.4% | +94.0% | +567.5% | -2.0% |
| 1 | 3 | 110483ns | +0.7% | -32.6% | +2.4% | +132.4% | +575.9% | -7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.036 | ok |
| head_tail_dyn | -0.442 | moderate- |
| kernel_nway | -0.011 | ok |
| kernel_seq | -0.151 | ok |
| nway | -0.153 | ok |
| sequential | -0.663 | HIGH- (thermal bounce) |
| std_parallel | -0.542 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5749498.4ns | 111336.8ns | 5164.1% | HIGH |
| head_tail_dyn | 5737854.4ns | 112290.5ns | 5109.8% | HIGH |
| kernel_nway | 5717782.9ns | 72965.7ns | 7836.3% | HIGH |
| kernel_seq | 5713678.0ns | 111717.6ns | 5114.4% | HIGH |
| nway | 6112964.3ns | 236242.1ns | 2587.6% | HIGH |
| sequential | 6323747.7ns | 744367.8ns | 849.5% | HIGH |
| std_parallel | 5711187.5ns | 105858.1ns | 5395.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 109968.1-112891.0 ns)
  109968.1 |##########################
  110114.2 |#############
  110260.4 |#############
  110406.5 |
  110552.6 |########################################
  110698.8 |#############
  110844.9 |
  110991.1 |#############
  111137.2 |##########################
  111283.4 |
  111429.5 |
  111575.7 |#############
  111721.8 |
  111868.0 |##########################
  112014.1 |
  112160.2 |
  112306.4 |
  112452.5 |
  112598.7 |
  112744.8 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 110295.8-114941.7 ns)
  110295.8 |#############
  110528.1 |##########################
  110760.4 |#############
  110992.7 |########################################
  111225.0 |##########################
  111457.3 |#############
  111689.6 |#############
  111921.9 |#############
  112154.2 |
  112386.5 |#############
  112618.7 |#############
  112851.0 |
  113083.3 |
  113315.6 |
  113547.9 |
  113780.2 |#############
  114012.5 |
  114244.8 |
  114477.1 |
  114709.4 |
  (1 below, 2 above range)

kernel_nway (n=18, range 67665.3-78984.0 ns)
  67665.3 |
  68231.2 |########################################
  68797.1 |########################################
  69363.1 |########################################
  69929.0 |####################
  70495.0 |####################
  71060.9 |####################
  71626.8 |
  72192.8 |####################
  72758.7 |
  73324.6 |####################
  73890.6 |
  74456.5 |
  75022.5 |####################
  75588.4 |####################
  76154.3 |####################
  76720.3 |
  77286.2 |
  77852.1 |####################
  78418.1 |
  (1 below, 2 above range)

kernel_seq (n=18, range 110058.3-113990.3 ns)
  110058.3 |########################################
  110254.9 |####################
  110451.5 |
  110648.1 |##########
  110844.7 |####################
  111041.3 |
  111237.9 |
  111434.5 |####################
  111631.1 |####################
  111827.7 |
  112024.3 |##########
  112220.9 |
  112417.5 |
  112614.1 |
  112810.7 |
  113007.3 |
  113203.9 |
  113400.5 |
  113597.1 |
  113793.7 |
  (1 below, 3 above range)

nway (n=18, range 207526.4-279073.6 ns)
  207526.4 |########################################
  211103.8 |##########################
  214681.1 |########################################
  218258.5 |#############
  221835.8 |#############
  225413.2 |#############
  228990.6 |
  232567.9 |#############
  236145.3 |
  239722.6 |#############
  243300.0 |
  246877.4 |
  250454.7 |
  254032.1 |##########################
  257609.5 |
  261186.8 |
  264764.2 |
  268341.5 |
  271918.9 |
  275496.3 |
  (1 below, 2 above range)

sequential (n=18, range 731977.8-751876.4 ns)
  731977.8 |
  732972.7 |
  733967.6 |
  734962.6 |
  735957.5 |####################
  736952.4 |####################
  737947.4 |####################
  738942.3 |####################
  739937.2 |
  740932.1 |####################
  741927.1 |
  742922.0 |####################
  743916.9 |########################################
  744911.9 |
  745906.8 |####################
  746901.7 |########################################
  747896.7 |####################
  748891.6 |
  749886.5 |####################
  750881.5 |########################################
  (1 below, 2 above range)

std_parallel (n=18, range 99743.1-113101.4 ns)
  99743.1 |#############
  100411.0 |##########################
  101078.9 |#############
  101746.8 |
  102414.7 |########################################
  103082.6 |#############
  103750.6 |##########################
  104418.5 |
  105086.4 |#############
  105754.3 |
  106422.2 |
  107090.2 |#############
  107758.1 |
  108426.0 |##########################
  109093.9 |
  109761.8 |
  110429.7 |
  111097.7 |
  111765.6 |#############
  112433.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=5145.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5111.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=8035.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5134.4% of algo (FFI overhead may distort results)
- **nway**: bridge=2669.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=847.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=5497.0% of algo (FFI overhead may distort results)
