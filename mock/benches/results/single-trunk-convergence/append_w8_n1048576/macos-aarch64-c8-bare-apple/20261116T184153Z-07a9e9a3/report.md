# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 52% faster than the next best (std_parallel)

kernel_nway (72.20 us) leads std_parallel (109.53 us) by 52%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 36% (significant)

kernel_nway is -39.99 us (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 10.4x slower than the field

sequential (753.82 us) is 10.4x the fastest (72.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.65)

head_tail's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, std_parallel, head_tail_dyn, kernel_seq, head_tail, nway} vs {sequential} (256% apart)

The field splits into a fast tier {kernel_nway, std_parallel, head_tail_dyn, kernel_seq, head_tail, nway} and a slow tier {sequential} with a 256% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 10.4x the fastest

Fastest kernel_nway (72.20 us) to slowest sequential (753.82 us): 10.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 72202.1 ns median (-35.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 10.44x (fastest 72202.1 ns, slowest 753822.9 ns)

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
| head_tail | 114598ns | 113965ns | 112624ns | 113874ns | 116670ns | base |
| head_tail_dyn | 114117ns | 113733ns | 112368ns | 113368ns | 116115ns | -0.42% |
| kernel_nway | 74664ns | 73985ns | 71393ns | 73682ns | 77772ns | -34.85% |
| kernel_seq | 114574ns | 113381ns | 111922ns | 113050ns | 118185ns | -0.02% |
| nway | 217384ns | 215762ns | 211539ns | 215094ns | 223742ns | +89.69% |
| sequential | 755636ns | 755019ns | 744515ns | 753936ns | 763747ns | +559.38% |
| std_parallel | 110952ns | 111008ns | 105782ns | 110137ns | 114761ns | -3.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 111891ns | 110981ns | 112895ns | base | 9.371 |
| head_tail_dyn | 111496ns | 110935ns | 112223ns | -0.35% | 9.405 |
| kernel_nway | 72635ns | 69394ns | 75397ns | -35.08% | 14.436 |
| kernel_seq | 112943ns | 110933ns | 116218ns | +0.94% | 9.284 |
| nway | 214398ns | 208488ns | 221431ns | +91.61% | 4.891 |
| sequential | 754004ns | 742989ns | 761892ns | +573.87% | 1.391 |
| std_parallel | 109324ns | 104139ns | 113277ns | -2.29% | 9.591 |

## Performance model

- Peak throughput: **15.110 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.403 | 62.2% |
| head_tail_dyn | 9.425 | 62.4% |
| kernel_nway | 14.523 | 96.1% |
| kernel_seq | 9.410 | 62.3% |
| nway | 4.948 | 32.7% |
| sequential | 1.391 | 9.2% |
| std_parallel | 9.573 | 63.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 114598ns | 114598ns | base |
| head_tail_dyn | 114117ns | 114117ns | -0.42% |
| kernel_nway | 74664ns | 74664ns | -34.85% |
| kernel_seq | 114574ns | 114574ns | -0.02% |
| nway | 217384ns | 217384ns | +89.69% |
| sequential | 755636ns | 755636ns | +559.38% |
| std_parallel | 110952ns | 110952ns | -3.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 111510ns | base | --- | [111383, 112469] | --- | --- | --- | --- |
| head_tail_dyn | 111250ns | no significant difference | [-1327, +135]ns | [111090, 111490] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 72202ns | -39991.7ns (-35.9%) | [-41100, -37958]ns | [71054, 73479] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 111435ns | no significant difference | [-504, +1021]ns | [111212, 112398] | no | 0.8145 | 0.8145 | 0 |
| nway | 211906ns | +100250.1ns (+89.9%) | [+98519, +102619]ns | [210796, 215033] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 753823ns | +641833.3ns (+575.6%) | [+638921, +646692]ns | [749896, 760083] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 109531ns | -1731.2ns (-1.6%) | [-5275, -567]ns | [106085, 112175] | YES | 0.0463 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 111262ns | -0.0% | -34.7% | +0.8% | +94.2% | +569.9% | -2.6% |
| 1 | 2 | 112808ns | -1.6% | -35.8% | -1.2% | +87.1% | +572.0% | -0.6% |
| 1 | 3 | 111498ns | +0.2% | -35.9% | -0.3% | +89.9% | +580.3% | -2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.653 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.384 | moderate- |
| kernel_nway | -0.082 | ok |
| kernel_seq | -0.055 | ok |
| nway | -0.246 | moderate- |
| sequential | -0.150 | ok |
| std_parallel | -0.647 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 5808640.3ns | 111891.2ns | 5191.3% | HIGH |
| head_tail_dyn | 5812047.5ns | 111496.3ns | 5212.8% | HIGH |
| kernel_nway | 5815941.0ns | 72635.4ns | 8007.0% | HIGH |
| kernel_seq | 5827823.6ns | 112942.8ns | 5160.0% | HIGH |
| nway | 5927960.0ns | 214397.7ns | 2764.9% | HIGH |
| sequential | 6440684.5ns | 754003.7ns | 854.2% | HIGH |
| std_parallel | 5820470.4ns | 109323.8ns | 5324.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 110980.6-112895.1 ns)
  110980.6 |########
  111076.3 |########
  111172.0 |
  111267.8 |
  111363.5 |########################################
  111459.2 |########################
  111554.9 |
  111650.7 |
  111746.4 |
  111842.1 |
  111937.8 |
  112033.6 |########
  112129.3 |
  112225.0 |
  112320.8 |########
  112416.5 |
  112512.2 |
  112607.9 |########
  112703.7 |
  112799.4 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 110934.7-112222.9 ns)
  110934.7 |########################################
  110999.1 |#############
  111063.6 |
  111128.0 |##########################
  111192.4 |##########################
  111256.8 |#############
  111321.2 |#############
  111385.6 |##########################
  111450.0 |
  111514.4 |#############
  111578.8 |
  111643.2 |
  111707.7 |
  111772.1 |
  111836.5 |##########################
  111900.9 |
  111965.3 |
  112029.7 |
  112094.1 |
  112158.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 69394.4-75397.2 ns)
  69394.4 |#############
  69694.6 |
  69994.7 |#############
  70294.9 |
  70595.0 |
  70895.1 |########################################
  71195.3 |
  71495.4 |
  71795.5 |##########################
  72095.7 |##########################
  72395.8 |
  72696.0 |#############
  72996.1 |#############
  73296.2 |#############
  73596.4 |#############
  73896.5 |#############
  74196.7 |
  74496.8 |
  74796.9 |
  75097.1 |
  (1 below, 3 above range)

kernel_seq (n=18, range 110933.3-116218.0 ns)
  110933.3 |########################
  111197.6 |########################################
  111461.8 |########
  111726.0 |################
  111990.3 |
  112254.5 |################
  112518.7 |
  112783.0 |
  113047.2 |
  113311.5 |
  113575.7 |########
  113839.9 |
  114104.2 |
  114368.4 |########
  114632.6 |
  114896.9 |
  115161.1 |
  115425.3 |
  115689.6 |
  115953.8 |########
  (1 below, 1 above range)

nway (n=18, range 208487.5-221431.2 ns)
  208487.5 |
  209134.7 |#############
  209781.9 |##########################
  210429.1 |
  211076.2 |########################################
  211723.4 |##########################
  212370.6 |#############
  213017.8 |#############
  213665.0 |
  214312.2 |#############
  214959.4 |#############
  215606.6 |
  216253.8 |
  216900.9 |#############
  217548.1 |
  218195.3 |
  218842.5 |
  219489.7 |
  220136.9 |
  220784.1 |
  (2 below, 3 above range)

sequential (n=18, range 742988.9-761891.7 ns)
  742988.9 |
  743934.0 |
  744879.1 |##########################
  745824.3 |
  746769.4 |
  747714.6 |
  748659.7 |#############
  749604.8 |#############
  750550.0 |
  751495.1 |
  752440.3 |##########################
  753385.4 |##########################
  754330.5 |
  755275.7 |#############
  756220.8 |#############
  757166.0 |
  758111.1 |
  759056.2 |
  760001.4 |########################################
  760946.5 |#############
  (2 below, 2 above range)

std_parallel (n=18, range 104138.9-113277.1 ns)
  104138.9 |
  104595.8 |
  105052.7 |##########################
  105509.6 |#############
  105966.5 |
  106423.4 |#############
  106880.4 |
  107337.3 |##########################
  107794.2 |
  108251.1 |
  108708.0 |#############
  109164.9 |
  109621.8 |#############
  110078.7 |
  110535.6 |
  110992.5 |
  111449.4 |#############
  111906.4 |########################################
  112363.3 |#############
  112820.2 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=5207.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=5221.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=8053.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=5210.4% of algo (FFI overhead may distort results)
- **nway**: bridge=2797.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=854.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=5314.6% of algo (FFI overhead may distort results)
