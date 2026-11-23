# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 16% faster than the next best (kernel_nway)

std_parallel (13.11 ms) leads kernel_nway (15.18 ms) by 16%, a clear separation rather than a photo finish. CV 12.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 37% (significant)

std_parallel is -7.67 ms (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### kernel_seq is an outlier: 2.4x slower than the field

kernel_seq (30.91 ms) is 2.4x the fastest (13.11 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: std_parallel** at 13112957.9 ns median (-37.0% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.36x (fastest 13112957.9 ns, slowest 30905963.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112617144 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 23174125ns | 20831322ns | 19748227ns | 20524042ns | 28862199ns | base |
| head_tail_dyn | 26606922ns | 26519512ns | 24293182ns | 25808008ns | 28962164ns | +14.81% |
| kernel_nway | 15608977ns | 15189218ns | 12836337ns | 14435758ns | 18755126ns | -32.64% |
| kernel_seq | 30910165ns | 30912132ns | 30769135ns | 30902859ns | 30991639ns | +33.38% |
| nway | 15378799ns | 15309171ns | 12882626ns | 15295545ns | 16751766ns | -33.64% |
| sequential | 27503143ns | 26228618ns | 26117277ns | 26196693ns | 30155751ns | +18.68% |
| std_parallel | 13799970ns | 13115092ns | 12913596ns | 13106456ns | 15283428ns | -40.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 23168565ns | 19745917ns | 28853191ns | base | 5.793 |
| head_tail_dyn | 26598673ns | 24290936ns | 28953176ns | +14.81% | 5.046 |
| kernel_nway | 15602347ns | 12830739ns | 18745966ns | -32.66% | 8.602 |
| kernel_seq | 30905038ns | 30767827ns | 30983541ns | +33.39% | 4.343 |
| nway | 15376354ns | 12880111ns | 16749250ns | -33.63% | 8.729 |
| sequential | 27501487ns | 26116042ns | 30154355ns | +18.70% | 4.880 |
| std_parallel | 13797026ns | 12910024ns | 15280385ns | -40.45% | 9.728 |

## Performance model

- Peak throughput: **10.461 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.444 | 61.6% |
| head_tail_dyn | 5.063 | 48.4% |
| kernel_nway | 8.840 | 84.5% |
| kernel_seq | 4.343 | 41.5% |
| nway | 8.769 | 83.8% |
| sequential | 5.118 | 48.9% |
| std_parallel | 10.236 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 23174125ns | 23174125ns | base |
| head_tail_dyn | 26606922ns | 26606922ns | +14.81% |
| kernel_nway | 15608977ns | 15608977ns | -32.64% |
| kernel_seq | 30910165ns | 30910165ns | +33.38% |
| nway | 15378799ns | 15378799ns | -33.64% |
| sequential | 27503143ns | 27503143ns | +18.68% |
| std_parallel | 13799970ns | 13799970ns | -40.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 20828810ns | base | --- | [19823694, 28853191] | --- | --- | --- | --- |
| head_tail_dyn | 26511691ns | no significant difference | [-2096279, +7690268]ns | [24331151, 28953176] | no | 0.2188 | 0.2188 | 0 |
| kernel_nway | 15182931ns | -6871706.7ns (-33.0%) | [-13648037, -2178910]ns | [12878143, 18745966] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_seq | 30905964ns | +9996799.3ns (+48.0%) | [+2130351, +11082270]ns | [30825609, 30983541] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| nway | 15306668ns | -4745131.5ns (-22.8%) | [-14663102, -3968400]ns | [14073143, 16749250] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 26226342ns | no significant difference | [-2118566, +8714685]ns | [26123765, 30154355] | no | 0.2188 | 0.2188 | 0 |
| std_parallel | 13112958ns | -7665116.7ns (-36.8%) | [-15855456, -4594044]ns | [12997735, 15280385] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 28242331ns | +2.5% | -33.6% | +9.3% | -50.1% | -7.5% | -53.8% |
| 1 | 2 | 19823694ns | +33.7% | -34.7% | +55.9% | -22.3% | +32.3% | -23.2% |
| 1 | 3 | 21439669ns | +13.5% | -29.5% | +44.3% | -22.5% | +40.6% | -38.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.280 | moderate- |
| head_tail_dyn | -0.001 | ok |
| kernel_nway | -0.410 | moderate- |
| kernel_seq | -0.001 | ok |
| nway | -0.000 | ok |
| sequential | -0.154 | ok |
| std_parallel | -0.666 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 20447527.9ns | 23168564.9ns | 88.3% | HIGH |
| head_tail_dyn | 26737165.9ns | 26598673.0ns | 100.5% | HIGH |
| kernel_nway | 16673121.3ns | 15602346.8ns | 106.9% | HIGH |
| kernel_seq | 30902913.3ns | 30905038.2ns | 100.0% | HIGH |
| nway | 14952097.5ns | 15376353.8ns | 97.2% | HIGH |
| sequential | 27614113.8ns | 27501487.3ns | 100.4% | HIGH |
| std_parallel | 14866700.4ns | 13797026.1ns | 107.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 19745916.6-28853190.8 ns)
  19745916.6 |########################################
  20201280.3 |####################
  20656644.0 |
  21112007.7 |########################################
  21567371.4 |
  22022735.1 |
  22478098.8 |
  22933462.6 |
  23388826.3 |
  23844190.0 |
  24299553.7 |
  24754917.4 |
  25210281.1 |
  25665644.8 |
  26121008.5 |
  26576372.2 |
  27031735.9 |
  27487099.6 |
  27942463.3 |
  28397827.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 24290936.5-28953176.5 ns)
  24290936.5 |########################################
  24524048.5 |
  24757160.5 |
  24990272.5 |
  25223384.5 |
  25456496.5 |
  25689608.5 |
  25922720.5 |
  26155832.5 |####################
  26388944.5 |
  26622056.5 |####################
  26855168.5 |
  27088280.5 |
  27321392.5 |
  27554504.5 |
  27787616.5 |
  28020728.5 |
  28253840.5 |
  28486952.5 |
  28720064.5 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 12830738.7-18745966.0 ns)
  12830738.7 |########################################
  13126500.1 |
  13422261.4 |
  13718022.8 |
  14013784.2 |
  14309545.5 |
  14605306.9 |
  14901068.3 |
  15196829.6 |
  15492591.0 |
  15788352.3 |
  16084113.7 |
  16379875.1 |
  16675636.4 |
  16971397.8 |
  17267159.2 |##########################
  17562920.5 |
  17858681.9 |
  18154443.3 |
  18450204.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 30767826.9-30983541.5 ns)
  30767826.9 |########################################
  30778612.6 |
  30789398.4 |
  30800184.1 |
  30810969.8 |
  30821755.5 |
  30832541.3 |
  30843327.0 |
  30854112.7 |
  30864898.4 |
  30875684.2 |########################################
  30886469.9 |########################################
  30897255.6 |
  30908041.4 |
  30918827.1 |########################################
  30929612.8 |
  30940398.5 |
  30951184.3 |
  30961970.0 |
  30972755.7 |########################################
  (0 below, 1 above range)

nway (n=6, range 12880111.2-16749250.4 ns)
  12880111.2 |#############
  13073568.2 |
  13267025.1 |
  13460482.1 |
  13653939.0 |
  13847396.0 |
  14040853.0 |
  14234309.9 |
  14427766.9 |
  14621223.8 |
  14814680.8 |
  15008137.8 |
  15201594.7 |########################################
  15395051.7 |#############
  15588508.6 |
  15781965.6 |
  15975422.6 |
  16168879.5 |
  16362336.5 |
  16555793.4 |
  (0 below, 1 above range)

sequential (n=6, range 26116042.0-30154354.8 ns)
  26116042.0 |########################################
  26317957.6 |
  26519873.3 |
  26721788.9 |
  26923704.6 |
  27125620.2 |
  27327535.8 |
  27529451.5 |
  27731367.1 |
  27933282.8 |
  28135198.4 |
  28337114.0 |
  28539029.7 |
  28740945.3 |
  28942861.0 |
  29144776.6 |
  29346692.2 |##########
  29548607.9 |
  29750523.5 |
  29952439.2 |
  (0 below, 1 above range)

std_parallel (n=6, range 12910024.5-15280385.4 ns)
  12910024.5 |#############
  13028542.5 |########################################
  13147060.6 |#############
  13265578.6 |
  13384096.7 |
  13502614.7 |
  13621132.8 |
  13739650.8 |
  13858168.8 |
  13976686.9 |
  14095204.9 |
  14213723.0 |
  14332241.0 |
  14450759.1 |
  14569277.1 |
  14687795.1 |
  14806313.2 |
  14924831.2 |
  15043349.3 |
  15161867.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=25.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=97.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=108.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=92.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.4% of algo (FFI overhead may distort results)
