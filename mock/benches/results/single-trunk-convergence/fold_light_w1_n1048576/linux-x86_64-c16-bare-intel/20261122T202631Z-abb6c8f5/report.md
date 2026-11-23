# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.1x slower than the field

std_parallel (230.67 us) is 2.1x the fastest (108.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 68.6%)

kernel_nway wins on median (108.37 us) yet has the highest variance (CV 68.6%), while sequential is the steadiest (CV 3.1%, 111.40 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.64)

sequential's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, sequential, kernel_seq, head_tail_dyn, head_tail, nway} vs {std_parallel} (66% apart)

The field splits into a fast tier {kernel_nway, sequential, kernel_seq, head_tail_dyn, head_tail, nway} and a slow tier {std_parallel} with a 66% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader kernel_nway vs stability leader sequential (+3% speed for 21.9x steadier)

kernel_nway is fastest (108.37 us, CV 68.6%); sequential gives up 2.8% median for 21.9x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_nway is inconsistent: worst-20% is 2.3x its best-20%

kernel_nway's best 20% of batches run at 101.01 us but its worst 20% at 231.90 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 108373.2 ns median (-15.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.13x (fastest 108373.2 ns, slowest 230673.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112795948 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 158719ns | 127929ns | 105154ns | 130236ns | 228227ns | base |
| head_tail_dyn | 152302ns | 119833ns | 99921ns | 118209ns | 229633ns | -4.04% |
| kernel_nway | 148281ns | 108602ns | 101251ns | 106835ns | 233964ns | -6.58% |
| kernel_seq | 140840ns | 113798ns | 105654ns | 111528ns | 202402ns | -11.26% |
| nway | 170337ns | 139192ns | 123873ns | 140980ns | 237606ns | +7.32% |
| sequential | 110329ns | 111636ns | 104838ns | 110061ns | 113476ns | -30.49% |
| std_parallel | 301648ns | 231037ns | 210815ns | 230963ns | 453092ns | +90.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 157975ns | 104930ns | 226617ns | base | 6.638 |
| head_tail_dyn | 151499ns | 99740ns | 227722ns | -4.10% | 6.921 |
| kernel_nway | 147436ns | 101006ns | 231899ns | -6.67% | 7.112 |
| kernel_seq | 140281ns | 105435ns | 201286ns | -11.20% | 7.475 |
| nway | 170081ns | 123709ns | 237226ns | +7.66% | 6.165 |
| sequential | 110058ns | 104598ns | 113168ns | -30.33% | 9.527 |
| std_parallel | 300499ns | 210505ns | 450387ns | +90.22% | 3.489 |

## Performance model

- Peak throughput: **10.513 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 8.213 | 78.1% |
| head_tail_dyn | 8.769 | 83.4% |
| kernel_nway | 9.676 | 92.0% |
| kernel_seq | 9.244 | 87.9% |
| nway | 7.547 | 71.8% |
| sequential | 9.413 | 89.5% |
| std_parallel | 4.546 | 43.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 158719ns | 158719ns | base |
| head_tail_dyn | 152302ns | 152302ns | -4.04% |
| kernel_nway | 148281ns | 148281ns | -6.58% |
| kernel_seq | 140840ns | 140840ns | -11.26% |
| nway | 170337ns | 170337ns | +7.32% |
| sequential | 110329ns | 110329ns | -30.49% |
| std_parallel | 301648ns | 301648ns | +90.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 127679ns | base | --- | [105690, 191511] | --- | --- | --- | --- |
| head_tail_dyn | 119571ns | no significant difference | [-46686, +14045]ns | [117602, 132284] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 108373ns | -3411.7ns (-2.7%) | [-34307, -431]ns | [103686, 147492] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_seq | 113430ns | no significant difference | [-71339, +8318]ns | [107114, 129392] | no | 0.8145 | 0.8145 | 0 |
| nway | 138941ns | +17482.8ns (+13.7%) | [+4024, +21633]ns | [124215, 195539] | YES (adj: no) | 0.0927 | 0.0309 | 0 |
| sequential | 111399ns | no significant difference | [-78910, +1809]ns | [106837, 112888] | no | 0.1444 | 0.0963 | 0 |
| std_parallel | 230673ns | +110488.9ns (+86.5%) | [+106345, +156255]ns | [215516, 337985] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 191511ns | -46.1% | -42.9% | -44.6% | +3.1% | -41.0% | +30.1% |
| 1 | 2 | 107605ns | +10.6% | -2.8% | +2.0% | +18.1% | -2.6% | +145.1% |
| 1 | 3 | 106175ns | +15.5% | +21.6% | +12.1% | +19.4% | +4.9% | +103.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.158 | ok |
| head_tail_dyn | -0.077 | ok |
| kernel_nway | -0.280 | moderate- |
| kernel_seq | -0.040 | ok |
| nway | -0.164 | ok |
| sequential | -0.643 | HIGH- (thermal bounce) |
| std_parallel | -0.358 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 159135.6ns | 157975.2ns | 100.7% | HIGH |
| head_tail_dyn | 155392.1ns | 151499.3ns | 102.6% | HIGH |
| kernel_nway | 149634.4ns | 147436.1ns | 101.5% | HIGH |
| kernel_seq | 139416.7ns | 140281.2ns | 99.4% | HIGH |
| nway | 171418.8ns | 170081.0ns | 100.8% | HIGH |
| sequential | 110660.4ns | 110058.0ns | 100.5% | HIGH |
| std_parallel | 312979.3ns | 300498.9ns | 104.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 104930.1-226616.7 ns)
  104930.1 |########################################
  111014.5 |
  117098.8 |
  123183.1 |
  129267.4 |
  135351.8 |
  141436.1 |#####
  147520.4 |
  153604.8 |
  159689.1 |
  165773.4 |
  171857.7 |
  177942.1 |
  184026.4 |#####
  190110.7 |###############
  196195.1 |
  202279.4 |#####
  208363.7 |
  214448.0 |
  220532.4 |#####
  (1 below, 2 above range)

head_tail_dyn (n=18, range 99740.3-227722.1 ns)
  99740.3 |####################
  106139.4 |
  112538.4 |########################################
  118937.5 |########################################
  125336.6 |##########
  131735.7 |##########
  138134.8 |
  144533.9 |
  150933.0 |
  157332.1 |
  163731.2 |
  170130.3 |
  176529.3 |
  182928.4 |
  189327.5 |
  195726.6 |##########
  202125.7 |
  208524.8 |
  214923.9 |
  221323.0 |
  (2 below, 3 above range)

kernel_nway (n=18, range 101006.3-231898.5 ns)
  101006.3 |########################################
  107550.9 |#################
  114095.5 |
  120640.1 |
  127184.7 |
  133729.3 |
  140273.9 |#####
  146818.6 |
  153363.2 |#####
  159907.8 |
  166452.4 |
  172997.0 |
  179541.6 |
  186086.2 |
  192630.8 |
  199175.5 |
  205720.1 |#####
  212264.7 |
  218809.3 |
  225353.9 |#####
  (2 below, 2 above range)

kernel_seq (n=18, range 105434.6-201286.2 ns)
  105434.6 |########################################
  110227.1 |
  115019.7 |######################
  119812.3 |
  124604.9 |
  129397.5 |
  134190.1 |
  138982.6 |#####
  143775.2 |
  148567.8 |
  153360.4 |#####
  158153.0 |#####
  162945.5 |
  167738.1 |
  172530.7 |
  177323.3 |
  182115.9 |
  186908.4 |
  191701.0 |
  196493.6 |
  (2 below, 2 above range)

nway (n=18, range 123709.0-237226.4 ns)
  123709.0 |########################################
  129384.9 |########################
  135060.8 |
  140736.6 |########
  146412.5 |
  152088.4 |
  157764.2 |########
  163440.1 |
  169116.0 |
  174791.8 |
  180467.7 |
  186143.6 |
  191819.4 |########################
  197495.3 |################
  203171.2 |
  208847.0 |
  214522.9 |
  220198.8 |
  225874.6 |
  231550.5 |
  (1 below, 2 above range)

sequential (n=18, range 104597.7-113167.6 ns)
  104597.7 |#############
  105026.2 |
  105454.7 |######
  105883.2 |
  106311.7 |
  106740.2 |
  107168.7 |
  107597.2 |
  108025.7 |######
  108454.2 |
  108882.6 |
  109311.1 |
  109739.6 |
  110168.1 |######
  110596.6 |#############
  111025.1 |
  111453.6 |
  111882.1 |######
  112310.6 |
  112739.1 |########################################
  (2 below, 2 above range)

std_parallel (n=18, range 210504.5-450386.6 ns)
  210504.5 |########################################
  222498.6 |#####
  234492.7 |#####
  246486.8 |#####
  258480.9 |
  270475.1 |#####
  282469.2 |#####
  294463.3 |
  306457.4 |
  318451.5 |
  330445.6 |
  342439.7 |
  354433.8 |
  366427.9 |
  378422.0 |#####
  390416.1 |#####
  402410.2 |
  414404.3 |
  426398.4 |
  438392.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=37.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=48.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=50.4% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=46.5% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: CV=35.3% (high variance, measurements may be unstable)
- **nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=41.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=97.2% of algo (FFI overhead may distort results)
