# fold_heavy, frames of tens of milliseconds, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq shows alternating (throttle bounce) (autocorr -0.58)

kernel_seq's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, nway, std_parallel, head_tail, head_tail_dyn} vs {kernel_seq, sequential} (73% apart)

The field splits into a fast tier {kernel_nway, nway, std_parallel, head_tail, head_tail_dyn} and a slow tier {kernel_seq, sequential} with a 73% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: kernel_nway** at 414531.2 ns median (-5.5% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.85x (fastest 414531.2 ns, slowest 767286.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 440073ns | 439255ns | 435562ns | 438042ns | 445375ns | base |
| head_tail_dyn | 441026ns | 441875ns | 435531ns | 440861ns | 444021ns | +0.22% |
| kernel_nway | 416878ns | 415740ns | 405344ns | 413653ns | 427484ns | -5.27% |
| kernel_seq | 762288ns | 761786ns | 757896ns | 760528ns | 767125ns | +73.22% |
| nway | 422611ns | 421203ns | 415573ns | 420715ns | 428974ns | -3.97% |
| sequential | 770488ns | 767573ns | 761146ns | 766927ns | 780500ns | +75.08% |
| std_parallel | 425276ns | 421568ns | 413802ns | 419253ns | 440047ns | -3.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 439135ns | 434354ns | 444328ns | base | 2.388 |
| head_tail_dyn | 440375ns | 435135ns | 443146ns | +0.28% | 2.381 |
| kernel_nway | 415722ns | 404677ns | 426286ns | -5.33% | 2.522 |
| kernel_seq | 761759ns | 757552ns | 766115ns | +73.47% | 1.377 |
| nway | 421793ns | 414656ns | 428078ns | -3.95% | 2.486 |
| sequential | 769127ns | 760615ns | 777208ns | +75.15% | 1.363 |
| std_parallel | 424714ns | 413375ns | 439458ns | -3.28% | 2.469 |

## Performance model

- Peak throughput: **2.591 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.392 | 92.3% |
| head_tail_dyn | 2.376 | 91.7% |
| kernel_nway | 2.530 | 97.6% |
| kernel_seq | 1.377 | 53.1% |
| nway | 2.494 | 96.3% |
| sequential | 1.367 | 52.7% |
| std_parallel | 2.490 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 440073ns | 440073ns | base |
| head_tail_dyn | 441026ns | 441026ns | +0.22% |
| kernel_nway | 416878ns | 416878ns | -5.27% |
| kernel_seq | 762288ns | 762288ns | +73.22% |
| nway | 422611ns | 422611ns | -3.97% |
| sequential | 770488ns | 770488ns | +75.08% |
| std_parallel | 425276ns | 425276ns | -3.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 438427ns | base | --- | [434651, 444328] | --- | --- | --- | --- |
| head_tail_dyn | 441245ns | no significant difference | [-1604, +4844]ns | [436734, 443146] | no | 0.2625 | 0.2188 | 0 |
| kernel_nway | 414531ns | -25677.1ns (-5.9%) | [-33698, -10865]ns | [406349, 426286] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 761490ns | +321744.8ns (+73.4%) | [+318943, +327182]ns | [757672, 766115] | YES | 0.0469 | 0.0313 | 0 |
| nway | 420391ns | -16979.1ns (-3.9%) | [-20635, -14411]ns | [416911, 428078] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 767286ns | +326958.3ns (+74.6%) | [+324740, +338276]ns | [762885, 777208] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 421089ns | no significant difference | [-30734, +2307]ns | [413594, 439458] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 434651ns | +1.0% | -3.5% | +75.0% | -4.1% | +77.3% | -2.2% |
| 1 | 2 | 440208ns | +0.1% | -5.8% | +73.4% | -4.5% | +74.7% | -4.4% |
| 1 | 3 | 442547ns | -0.3% | -6.7% | +72.1% | -3.3% | +73.4% | -3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.035 | ok |
| head_tail_dyn | -0.048 | ok |
| kernel_nway | -0.059 | ok |
| kernel_seq | -0.584 | HIGH- (thermal bounce) |
| nway | -0.037 | ok |
| sequential | -0.002 | ok |
| std_parallel | -0.541 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 435359.4ns | 439135.4ns | 99.1% | HIGH |
| head_tail_dyn | 444390.6ns | 440375.0ns | 100.9% | HIGH |
| kernel_nway | 411524.3ns | 415722.2ns | 99.0% | HIGH |
| kernel_seq | 765519.1ns | 761758.7ns | 100.5% | HIGH |
| nway | 415189.2ns | 421793.4ns | 98.4% | HIGH |
| sequential | 765788.2ns | 769126.7ns | 99.6% | HIGH |
| std_parallel | 421270.8ns | 424713.5ns | 99.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 434354.2-444328.2 ns)
  434354.2 |########################################
  434852.9 |########################################
  435351.6 |
  435850.3 |
  436349.0 |
  436847.7 |
  437346.4 |########################################
  437845.1 |
  438343.8 |
  438842.5 |
  439341.2 |########################################
  439839.9 |
  440338.6 |
  440837.3 |
  441336.0 |
  441834.7 |
  442333.4 |
  442832.1 |########################################
  443330.8 |
  443829.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 435135.4-443145.8 ns)
  435135.4 |########################################
  435535.9 |
  435936.4 |
  436337.0 |
  436737.5 |
  437138.0 |
  437538.5 |
  437939.0 |########################################
  438339.6 |
  438740.1 |
  439140.6 |
  439541.1 |
  439941.6 |########################################
  440342.2 |
  440742.7 |
  441143.2 |
  441543.7 |
  441944.2 |
  442344.8 |########################################
  442745.3 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 404677.1-426286.5 ns)
  404677.1 |########################################
  405757.6 |
  406838.0 |
  407918.5 |########################################
  408999.0 |
  410079.4 |
  411159.9 |########################################
  412240.4 |
  413320.8 |
  414401.3 |
  415481.8 |
  416562.2 |########################################
  417642.7 |
  418723.2 |
  419803.6 |
  420884.1 |########################################
  421964.6 |
  423045.0 |
  424125.5 |
  425206.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 757552.1-766114.6 ns)
  757552.1 |########################################
  757980.2 |
  758408.3 |
  758836.5 |
  759264.6 |####################
  759692.7 |
  760120.8 |
  760549.0 |
  760977.1 |
  761405.2 |
  761833.3 |
  762261.5 |
  762689.6 |
  763117.7 |########################################
  763545.8 |
  763974.0 |
  764402.1 |
  764830.2 |
  765258.3 |
  765686.5 |
  (0 below, 1 above range)

nway (n=6, range 414656.2-428078.2 ns)
  414656.2 |########################################
  415327.3 |
  415998.4 |
  416669.5 |
  417340.6 |
  418011.7 |
  418682.8 |########################################
  419353.9 |########################################
  420025.0 |
  420696.1 |########################################
  421367.2 |
  422038.3 |
  422709.4 |
  423380.5 |
  424051.6 |
  424722.7 |
  425393.8 |########################################
  426064.9 |
  426736.0 |
  427407.1 |
  (0 below, 1 above range)

sequential (n=6, range 760614.6-777208.3 ns)
  760614.6 |########################################
  761444.3 |
  762274.0 |
  763103.7 |
  763933.3 |
  764763.0 |########################################
  765592.7 |########################################
  766422.4 |
  767252.1 |
  768081.8 |
  768911.4 |########################################
  769741.1 |
  770570.8 |
  771400.5 |
  772230.2 |
  773059.9 |########################################
  773889.6 |
  774719.2 |
  775548.9 |
  776378.6 |
  (0 below, 1 above range)

std_parallel (n=6, range 413375.0-439458.3 ns)
  413375.0 |########################################
  414679.2 |
  415983.3 |
  417287.5 |
  418591.7 |
  419895.8 |
  421200.0 |
  422504.2 |
  423808.3 |
  425112.5 |
  426416.7 |
  427720.8 |#############
  429025.0 |
  430329.1 |
  431633.3 |
  432937.5 |
  434241.6 |
  435545.8 |#############
  436850.0 |
  438154.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=98.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: bridge=98.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.5% of algo (FFI overhead may distort results)
