# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {nway, kernel_nway, std_parallel, head_tail, head_tail_dyn} vs {sequential, kernel_seq} (54% apart)

The field splits into a fast tier {nway, kernel_nway, std_parallel, head_tail, head_tail_dyn} and a slow tier {sequential, kernel_seq} with a 54% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: nway** at 42215672.0 ns median (-6.6% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.97x (fastest 42215672.0 ns, slowest 83202615.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 46006387ns | 45210134ns | 44315250ns | 45011736ns | 48343933ns | base |
| head_tail_dyn | 53089846ns | 53186005ns | 50816602ns | 52616931ns | 54935840ns | +15.40% |
| kernel_nway | 43605287ns | 43116279ns | 41149718ns | 43055323ns | 45658018ns | -5.22% |
| kernel_seq | 83270491ns | 83208565ns | 81554438ns | 82827457ns | 84793069ns | +81.00% |
| nway | 42410382ns | 42219301ns | 41312239ns | 42062875ns | 43480714ns | -7.82% |
| sequential | 82147828ns | 81978404ns | 79794734ns | 81741713ns | 83933546ns | +78.56% |
| std_parallel | 44171113ns | 43225693ns | 40226854ns | 43210574ns | 47584051ns | -3.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 46000316ns | 44307862ns | 48338341ns | base | 2.918 |
| head_tail_dyn | 53083857ns | 50809010ns | 54930108ns | +15.40% | 2.528 |
| kernel_nway | 43595451ns | 41143560ns | 45640325ns | -5.23% | 3.079 |
| kernel_seq | 83263386ns | 81546581ns | 84784038ns | +81.01% | 1.612 |
| nway | 42406617ns | 41308468ns | 43476967ns | -7.81% | 3.165 |
| sequential | 82144188ns | 79790928ns | 83930048ns | +78.57% | 1.634 |
| std_parallel | 44167129ns | 40224241ns | 47580734ns | -3.99% | 3.039 |

## Performance model

- Peak throughput: **3.337 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.969 | 89.0% |
| head_tail_dyn | 2.524 | 75.6% |
| kernel_nway | 3.113 | 93.3% |
| kernel_seq | 1.613 | 48.3% |
| nway | 3.179 | 95.3% |
| sequential | 1.637 | 49.1% |
| std_parallel | 3.105 | 93.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 46006387ns | 46006387ns | base |
| head_tail_dyn | 53089846ns | 53089846ns | +15.40% |
| kernel_nway | 43605287ns | 43605287ns | -5.22% |
| kernel_seq | 83270491ns | 83270491ns | +81.00% |
| nway | 42410382ns | 42410382ns | -7.82% |
| sequential | 82147828ns | 82147828ns | +78.56% |
| std_parallel | 44171113ns | 44171113ns | -3.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 45204340ns | base | --- | [44458268, 48338341] | --- | --- | --- | --- |
| head_tail_dyn | 53179957ns | +8128975.0ns (+18.0%) | [+3395880, +9725768]ns | [51141507, 54930108] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| kernel_nway | 43110039ns | no significant difference | [-5846903, +266160]ns | [42035988, 45640325] | no | 0.2625 | 0.2188 | 0 |
| kernel_seq | 83202615ns | +37602539.6ns (+83.2%) | [+34200270, +39986400]ns | [81803505, 84784038] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 42215672ns | -3034481.6ns (-6.7%) | [-6168969, -1577647]ns | [41527212, 43476967] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 81974532ns | +37070323.8ns (+82.0%) | [+32632632, +38728660]ns | [80527984, 83930048] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 43220689ns | no significant difference | [-6045662, +2229621]ns | [41699965, 47580734] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 47745627ns | +7.1% | -12.0% | +77.1% | -10.0% | +73.2% | -12.7% |
| 1 | 2 | 45204340ns | +21.5% | -3.9% | +84.6% | -6.4% | +78.8% | -2.4% |
| 1 | 3 | 45050982ns | +18.0% | +0.6% | +81.6% | -6.9% | +84.0% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.138 | ok |
| head_tail_dyn | -0.474 | moderate- |
| kernel_nway | -0.005 | ok |
| kernel_seq | -0.007 | ok |
| nway | -0.016 | ok |
| sequential | -0.663 | HIGH- (thermal bounce) |
| std_parallel | -0.000 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 46461880.0ns | 46000316.2ns | 101.0% | HIGH |
| head_tail_dyn | 52627511.1ns | 53083857.4ns | 99.1% | HIGH |
| kernel_nway | 43724881.6ns | 43595450.6ns | 100.3% | HIGH |
| kernel_seq | 83295161.4ns | 83263386.1ns | 100.0% | HIGH |
| nway | 43043453.3ns | 42406617.1ns | 101.5% | HIGH |
| sequential | 81379382.9ns | 82144187.9ns | 99.1% | HIGH |
| std_parallel | 42967983.4ns | 44167129.5ns | 97.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 44307861.9-48338341.1 ns)
  44307861.9 |########################################
  44509385.9 |########################################
  44710909.8 |########################################
  44912433.8 |
  45113957.7 |
  45315481.7 |########################################
  45517005.7 |
  45718529.6 |########################################
  45920053.6 |
  46121577.5 |
  46323101.5 |
  46524625.5 |
  46726149.4 |
  46927673.4 |
  47129197.3 |
  47330721.3 |
  47532245.3 |
  47733769.2 |
  47935293.2 |
  48136817.1 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 50809010.4-54930108.1 ns)
  50809010.4 |########################################
  51015065.3 |
  51221120.2 |
  51427175.1 |########################################
  51633229.9 |
  51839284.8 |
  52045339.7 |
  52251394.6 |
  52457449.5 |
  52663504.4 |
  52869559.3 |########################################
  53075614.2 |
  53281669.0 |########################################
  53487723.9 |
  53693778.8 |
  53899833.7 |########################################
  54105888.6 |
  54311943.5 |
  54517998.4 |
  54724053.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 41143560.2-45640324.6 ns)
  41143560.2 |########################################
  41368398.4 |
  41593236.6 |
  41818074.9 |
  42042913.1 |
  42267751.3 |
  42492589.5 |
  42717427.8 |########################################
  42942266.0 |########################################
  43167104.2 |########################################
  43391942.4 |
  43616780.6 |
  43841618.9 |########################################
  44066457.1 |
  44291295.3 |
  44516133.5 |
  44740971.8 |
  44965810.0 |
  45190648.2 |
  45415486.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 81546581.4-84784038.3 ns)
  81546581.4 |####################
  81708454.2 |
  81870327.1 |
  82032199.9 |####################
  82194072.8 |
  82355945.6 |
  82517818.5 |
  82679691.3 |
  82841564.2 |
  83003437.0 |####################
  83165309.9 |
  83327182.7 |########################################
  83489055.6 |
  83650928.4 |
  83812801.3 |
  83974674.1 |
  84136547.0 |
  84298419.8 |
  84460292.7 |
  84622165.5 |
  (0 below, 1 above range)

nway (n=6, range 41308468.2-43476967.2 ns)
  41308468.2 |####################
  41416893.2 |
  41525318.1 |
  41633743.1 |
  41742168.0 |########################################
  41850593.0 |
  41959017.9 |
  42067442.9 |
  42175867.8 |
  42284292.8 |
  42392717.7 |
  42501142.7 |####################
  42609567.6 |
  42717992.6 |####################
  42826417.5 |
  42934842.5 |
  43043267.4 |
  43151692.4 |
  43260117.3 |
  43368542.3 |
  (0 below, 1 above range)

sequential (n=6, range 79790927.7-83930047.5 ns)
  79790927.7 |########################################
  79997883.7 |
  80204839.7 |
  80411795.7 |
  80618751.7 |
  80825707.7 |
  81032663.6 |
  81239619.6 |########################################
  81446575.6 |
  81653531.6 |
  81860487.6 |########################################
  82067443.6 |########################################
  82274399.6 |
  82481355.6 |
  82688311.6 |
  82895267.5 |
  83102223.5 |
  83309179.5 |
  83516135.5 |########################################
  83723091.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 40224241.0-47580734.1 ns)
  40224241.0 |#############
  40592065.7 |
  40959890.3 |
  41327715.0 |
  41695539.6 |
  42063364.3 |
  42431188.9 |
  42799013.6 |
  43166838.2 |########################################
  43534662.9 |
  43902487.5 |
  44270312.2 |
  44638136.9 |
  45005961.5 |#############
  45373786.2 |
  45741610.8 |
  46109435.5 |
  46477260.1 |
  46845084.8 |
  47212909.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=102.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=97.9% of algo (FFI overhead may distort results)
